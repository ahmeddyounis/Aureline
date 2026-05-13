//! Native desktop shell bootstrap and event-loop wiring.
//!
//! Owns the canonical native window bootstrap, input dispatch root, and
//! startup-milestone emission for the desktop shell.

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::env;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::sync::mpsc::{self, Receiver};
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::a11y::shell_bridge::{
    materialize_shell_accessibility_tree, write_shell_accessibility_tree_log,
    ShellA11yEnablementContext,
};
use crate::activity_center::{
    ActivityCenterRow, ActivityCenterRuntime, ActivityCenterSnapshot, ActivityRowLifecycleClass,
    ActivityRowProgress, ActivityRowRetryability, DurableJobObservation,
};
use crate::app_frame::desktop_frame::{
    DesktopFrame, EditorTabId, NewEditorGroupOutcome, SplitViolation,
};
use crate::bootstrap::startup_trace::{StartupMilestone, StartupTrace, StartupTraceConfig};
use crate::chrome::title_context_bar::{
    DeploymentProfileClass, HostStateClass, IdentityMode as TitleIdentityMode, ProfileModeClass,
    SurfaceKind, TitleContextBarRuntimeInputs, TitleContextBarRuntimeState,
    TitleContextBarStateRecord,
};
use crate::clone::{
    CloneError, CloneErrorClass, CloneProgressEvent, CloneProgressPhase, CloneRequest,
    GitCloneBackend, GitProbe, SystemGitCloneBackend,
};
use crate::commands::diagnostics_sheet::{
    diagnostics_sheet_lines, materialize_command_diagnostics_sheet_record_with_arguments,
    write_diagnostics_sheet_log, CommandDiagnosticsSheetRecord,
};
use crate::commands::invocation_preview::{
    invocation_preview_sheet_lines, materialize_command_invocation_preview_sheet_record,
    write_invocation_preview_sheet_log, CommandInvocationPreviewSheetRecord,
};
use crate::commands::CommandReviewRuntimeInputs;
use crate::embedded::boundary_card::EmbeddedBoundaryCardRecord;
use crate::embedded::docs_help::{resolve_docs_help_handoff_url, seeded_docs_help_boundary_card};
use crate::explorer::{
    ExplorerNode, ExplorerNodeId, ExplorerNodeKind, ExplorerTree, ExplorerViewportRow,
    GeneratedArtifactHint, NodeReadinessClass,
};
use crate::help::keybinding_inspector::build_inspector_lines;
use crate::host_boundary_cues::{HostBoundaryCueCardRecord, HostBoundaryCueWedge};
use crate::import::{
    materialize_import_diff_review_packet, write_import_diff_review_log, write_import_review_log,
    CompetitorConfigClassifier, ImportDiffReviewPacket, ImportReviewDecisionClass,
    ImportReviewRecord,
};
use crate::layout::split_tree::PaneId;
use crate::layout::zone_registry::{Rect, ShellZoneId};
use crate::notifications::{
    DedupeKeyScheme, FanoutReceiptState, FanoutSurfaceClass, NotificationEnvelope,
    NotificationRouter, NotificationRoutingError, NotificationSurfaceRow, PrivacyClass,
    PrivacyPayloadClass, QuietHoursMode, QuietHoursPosture, RedactionClass, ReopenTarget,
    ReopenTargetKind, RoutedNotification, SeverityClass, SourceSubsystem, StableAction,
    SuppressionState, NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
};
use crate::palette::preview::{
    argument_provenance_map_for, copy_payload_for,
    materialize_palette_preview_record_with_arguments, write_preview_log, PaletteCopyIntent,
    PalettePreviewRuntimeInputs, PalettePreviewSelection,
};
use crate::palette::results_view::palette_view_rows;
use crate::palette::{CommandPaletteCommit, CommandPaletteState};
use crate::restore::placeholders::recent_work_placeholder_card;
use crate::restore::{
    materialize_restore_prompt, restore_prompt_status_line, write_restore_prompt_log,
};
use crate::save_review::{
    materialize_save_review_sheet_record, save_review_sheet_lines, write_save_review_sheet_log,
    SaveReviewChoiceKey, SaveReviewSheetRecord,
};
use crate::start_center::{
    admission_review::{
        admission_packet_for_resolved_entry, clone_form_admission_packet,
        compact_admission_review_lines, import_form_admission_packet,
    },
    build_action_rows as start_center_action_rows, StartCenterPrimaryActionId,
    StartCenterRuntimeInputs, StartCenterState, START_CENTER_PRESENTATION_LABEL,
    START_CENTER_PRESENTATION_SUBTITLE,
};
use crate::state_cards::{shell_slot_label, DegradedStateToken, ShellPlaceholderCard};
use crate::status_bar::{
    BackgroundStateSnapshot, EncodingSnapshot, ProfileSnapshot, StatusBarInputs, StatusBarItemKind,
    StatusBarItemRecord, StatusBarSnapshot, TargetSnapshot,
};
use crate::terminal_pane::{TerminalPaneSnapshot, TerminalPaneTabRecord};
use crate::wedge_inspector::{
    WedgeInspectorInputs, WedgeInspectorOverlay, WEDGE_INSPECTOR_COMMAND_ID,
};
use crate::workspace_switcher::{
    build_switcher_rows, WorkspaceSwitcherRow, WorkspaceSwitcherState,
    WORKSPACE_SWITCHER_PRESENTATION_LABEL,
};
use aureline_buffer::{Buffer, RevisionId, Snapshot, TransactionId, TransactionSpec, UndoClass};
use aureline_build_info as build_info;
use aureline_commands::invocation::{
    mint_approval_ticket_ref, mint_basis_snapshot_ref, mint_invocation_session_id,
    mint_preview_record_ref, now_rfc3339, ActivityRefEntry, AliasUsedBlock, ApprovalPostureBlock,
    ArgumentProvenanceEntry, ArtifactRefEntry, CommandInvocationSession, CommandResultPacketRecord,
    ContextRefsBlock, EnablementDecisionBlock, EvidenceRefEntry, ExportPostureBlock,
    InvocationContextSnapshot, InvocationCreatedArtifactRefEntry, InvocationOutcomeBlock,
    InvocationSessionPacketRecord, NoBypassGuards, NotificationRefEntry, ResultBodyBlock,
    RollbackHandleRefBlock,
};
use aureline_commands::registry::seeded_registry;
use aureline_commands::{
    CommandEnablementContext, CommandRegistry, CommandRegistryEntryRecord, DisabledReasonCode,
    EnablementDecisionClass, PreflightDecisionClass,
};
use aureline_history::{HistoryStorageRoot, LocalHistoryStore, MutationJournalStore};
use aureline_input::keybindings::{
    seeded_keybinding_resolver, InspectionScope, KeySequence, KeyStroke, KeybindingResolver,
    Modifiers, PlatformClass, SequenceResolutionState, SurfaceSupportClass, WinningResolutionKind,
};
use aureline_input::presets::{preset_binding_rows, resolver_with_preset, KeymapPresetId};
use aureline_runtime::{
    CapsuleDriftState, EnvironmentCapsuleRef, ExecutionContext, ExecutionContextRequest,
    ExecutionContextResolver, ExecutionContextResolverConfig, IdentityMode, ScopeClass,
    TargetClass,
};
use aureline_settings::{
    EffectiveSettingsResolver, EffectiveValue, SchemaRegistry, ScopeOverlay, SettingScope,
    SettingValue, WriteAttemptOutcome, WriteDenialReason, WriteIntent,
};
use aureline_telemetry::hot_path_metrics::{
    HotPathMetrics, HotPathMetricsConfig, HotPathMetricsContext,
};
use aureline_telemetry::trace_event::BuildIdentityRecord as TelemetryBuildIdentityRecord;
use aureline_terminal::{HostClass, OpenSessionRequest, PtyHost, PtySessionId};
use aureline_ui::components::{
    ComponentStateRegistry, ComponentStates, ComponentSurfaceTone, FocusReturnStack,
};
use aureline_ui::density::DensityProfile;
use aureline_ui::motion::{ReducedMotionSubstitutionClass, OVERLAY_DIALOG_ENTER};
use aureline_ui::themes::{
    AccessibilityPostureClass, AppearanceSessionRecord, ContrastMode, DensityClass,
    LiveFollowSystemPolicyRecord, ReducedMotionSource,
};
use aureline_ui::tokens::{
    seeded_token_registry, ColorRgba, ThemeClass, TokenRegistry, TokenRegistryError,
};
use aureline_vfs::save::open_save_target;
use aureline_vfs::{
    HookCounters, IdentityRecord as VfsIdentityRecord, LocalFilesystemRoot, SaveTargetToken,
    VfsChangeKind, VfsRoot, VfsUri, VirtualDocumentKind, VirtualDocumentRoot, VirtualDocumentSpec,
    WatcherEvent, WatcherHealth,
};
use aureline_workspace::save::{
    detect_and_decode_for_buffer, encode_for_save, SaveResult, SourceFidelityRecord,
    StagedSaveCoordinator, StagedSaveRequest,
};
use aureline_workspace::{
    normalize_recent_work_entry_recovery_actions, resolve_entry_flow, write_admission_review_log,
    AdmissionReviewPacket, AdmissionSourceSurface, EntryFlowOutcome, EntryFlowRequest,
    EntryFlowTarget, EntryVerb, OpenFlowSheetClass, PortabilityClass, RecentWorkEntryRecord,
    RecentWorkEntryRecordKind, RecentWorkRegistry, RecentWorkRegistryError,
    RecentWorkRegistryRecordKind, RecentWorkTargetState, RestoreAvailability, ResultingMode,
    SafeRecoveryAction, TargetKind, TrustState, WorkspaceLifecycleMachine, WorkspaceLifecycleState,
    WorkspaceRootKind,
};

use crate::bootstrap::appearance_golden::write_png_0rgb;
use crate::windowing::display_safety::{
    materialize_adjustment_record, materialize_topology_record, write_display_safety_log,
    write_display_safety_topology_log, DisplaySafetyGuard,
};
use crate::windowing::folder_picker;
use crate::windowing::winit_softbuffer::{create_softbuffer_surface, SoftbufferSurface};
use crate::windowing::winit_window::WinitWindow;
use arboard::Clipboard;
use aureline_editor::{
    open_document, CaretMove, ClassificationPolicy, DocumentOpenDisposition, DocumentOpenOutcome,
    EditorAction, EditorTextRuntime, EditorViewport, EditorViewportSnapshot, FindReplaceMode,
    FindReplaceState, LargeFileDocument, LargeFileOverrideInfo, LargeFileViewerConfig,
    SelectionDelta, ViewportCompositor, ViewportPaintStyle,
};
use aureline_render::hooks::{Clock, Hook};
use aureline_render::{
    CompositionLayerId, DamageClassId, DamageEvent, DamageRegion, DirtyRegionEngine,
    FrameScheduler, FrameSchedulerDecision, GlyphAtlas, GlyphKey, PixelRect, WallClock,
    WgpuBlitRenderer,
};
use aureline_text::shaping::{FeatureSet, FontFallbackConfig, FontSystem, TextShaper};
use font8x8::{UnicodeFonts as _, BASIC_FONTS};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, Ime, KeyEvent, MouseScrollDelta, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use aureline_recovery::crash_journal::{CrashJournalStore, CrashMarkerGuard};
use aureline_recovery::session_restore::{
    RestoreDirtyBufferReplay, RestoreOutcome, RestorePaneExecutionKind, RestoreProposal,
    RestoreRuntime, SessionRestoreStore,
};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug, Clone)]
struct ShellRenderTokens {
    bg_canvas: ColorRgba,
    bg_subtle: ColorRgba,
    bg_surface: ColorRgba,
    bg_raised: ColorRgba,
    bg_hover: ColorRgba,
    #[allow(dead_code)]
    bg_active: ColorRgba,
    bg_overlay: ColorRgba,
    text_primary: ColorRgba,
    text_secondary: ColorRgba,
    text_muted: ColorRgba,
    #[allow(dead_code)]
    text_inverse: ColorRgba,
    border_default: ColorRgba,
    border_strong: ColorRgba,
    #[allow(dead_code)]
    icon_default: ColorRgba,
    #[allow(dead_code)]
    icon_muted: ColorRgba,
    #[allow(dead_code)]
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
    component_states: ComponentStateRegistry,
    #[allow(dead_code)]
    density_class: DensityClass,
    density_row_height: u32,
    density_control_height: u32,
    density_tab_height: u32,
    density_panel_padding: u32,
    density_zone_inset: u32,
    density_gutter: u32,
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
    status_danger: ColorRgba,
    status_danger_border: ColorRgba,
    status_danger_fill: ColorRgba,
}

impl ShellRenderStyle {
    fn load(
        registry: &TokenRegistry,
        density_class: DensityClass,
        scale_factor: f64,
    ) -> Result<Self, TokenRegistryError> {
        let scale_px = |value: u32| -> u32 {
            if !scale_factor.is_finite() || scale_factor <= 0.0 {
                return value;
            }
            ((value as f64) * scale_factor).round().max(1.0) as u32
        };

        let density = DensityProfile::load(registry, density_class)?;
        Ok(Self {
            tokens: ShellRenderTokens::load(registry)?,
            component_states: ComponentStateRegistry::load(registry)?,
            density_class,
            density_row_height: scale_px(density.row_height_px()),
            density_control_height: scale_px(density.control_height_px()),
            density_tab_height: scale_px(density.tab_height_px()),
            density_panel_padding: scale_px(density.panel_padding_px()),
            density_zone_inset: scale_px(density.zone_inset_px()),
            density_gutter: scale_px(density.gutter_px()),
            stroke_default: scale_px(registry.require_stroke_px("stroke.border.default")?),
            stroke_focus: scale_px(registry.require_stroke_px("stroke.focus.ring")?),
            space_2: scale_px(registry.require_space_px("space.2")?),
            space_3: scale_px(registry.require_space_px("space.3")?),
            space_4: scale_px(registry.require_space_px("space.4")?),
            space_6: scale_px(registry.require_space_px("space.6")?),
            status_warning: registry.require_color("status.warning")?,
            status_warning_border: registry.require_color("status.warning.border")?,
            status_warning_fill: registry.require_color("status.warning.fill")?,
            status_success: registry.require_color("status.success")?,
            status_success_border: registry.require_color("status.success.border")?,
            status_success_fill: registry.require_color("status.success.fill")?,
            status_danger: registry.require_color("status.danger")?,
            status_danger_border: registry.require_color("status.danger.border")?,
            status_danger_fill: registry.require_color("status.danger.fill")?,
        })
    }
}

#[derive(Debug, Default)]
struct NativeShellArgs {
    startup_trace: StartupTraceConfig,
    hot_path_metrics: HotPathMetricsConfig,
    disable_clipboard: bool,
    renderer: ShellRendererChoice,
    open_workspace_path: Option<PathBuf>,
    emit_onboarding_alpha_path: Option<PathBuf>,
    headless_edit_save: HeadlessEditSaveArgs,
    window_size: Option<(f64, f64)>,
    screenshot_path: Option<PathBuf>,
    theme_class: Option<ThemeClass>,
    density_class: Option<DensityClass>,
    reduced_motion_posture: Option<AccessibilityPostureClass>,
    ui_theme: Option<String>,
    ui_density: Option<String>,
    ui_motion: Option<String>,
}

#[derive(Debug, Default)]
struct HeadlessEditSaveArgs {
    file_path: Option<PathBuf>,
    write_hex: Option<String>,
    report_path: Option<PathBuf>,
}

impl HeadlessEditSaveArgs {
    fn is_requested(&self) -> bool {
        self.file_path.is_some() || self.write_hex.is_some() || self.report_path.is_some()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShellRendererChoice {
    Gpu,
    Software,
}

impl Default for ShellRendererChoice {
    fn default() -> Self {
        Self::Gpu
    }
}

fn parse_native_shell_args() -> Result<NativeShellArgs, String> {
    parse_native_shell_args_from(env::args().skip(1))
}

fn parse_native_shell_args_from<I, S>(argv: I) -> Result<NativeShellArgs, String>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut iter = argv.into_iter().map(Into::into);
    let mut args = NativeShellArgs::default();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--emit-startup-trace" => {
                let path = iter.next().ok_or_else(|| {
                    "--emit-startup-trace requires an output file path".to_string()
                })?;
                args.startup_trace.output_path = Some(path);
            }
            "--emit-hot-path-metrics" => {
                let path = iter.next().ok_or_else(|| {
                    "--emit-hot-path-metrics requires an output file path".to_string()
                })?;
                args.hot_path_metrics.output_path = Some(path);
            }
            "--emit-onboarding-alpha" => {
                let path = iter.next().ok_or_else(|| {
                    "--emit-onboarding-alpha requires an output file path".to_string()
                })?;
                args.emit_onboarding_alpha_path = Some(PathBuf::from(path));
            }
            "--exit-after-first-frame" => {
                args.startup_trace.exit_after_first_frame = true;
            }
            "--open" => {
                let path = iter
                    .next()
                    .ok_or_else(|| "--open requires a folder path".to_string())?;
                args.open_workspace_path = Some(resolve_open_workspace_path(&path)?);
            }
            "--headless-test-edit-save" => {
                let path = iter
                    .next()
                    .ok_or_else(|| "--headless-test-edit-save requires a file path".to_string())?;
                args.headless_edit_save.file_path = Some(PathBuf::from(path));
            }
            "--headless-test-write-hex" => {
                let value = iter.next().ok_or_else(|| {
                    "--headless-test-write-hex requires a hex-encoded byte payload".to_string()
                })?;
                args.headless_edit_save.write_hex = Some(value);
            }
            "--headless-test-report" => {
                let path = iter.next().ok_or_else(|| {
                    "--headless-test-report requires an output file path".to_string()
                })?;
                args.headless_edit_save.report_path = Some(PathBuf::from(path));
            }
            "--emit-screenshot" => {
                let path = iter
                    .next()
                    .ok_or_else(|| "--emit-screenshot requires an output file path".to_string())?;
                args.screenshot_path = Some(PathBuf::from(path));
                args.startup_trace.exit_after_first_frame = true;
            }
            "--window-size" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--window-size requires a value like 1280x720".to_string())?;
                args.window_size = Some(parse_window_size(&value)?);
            }
            "--theme-class" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--theme-class requires a theme class token".to_string())?;
                args.theme_class = Some(parse_theme_class(&value)?);
            }
            "--density-class" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--density-class requires a density class token".to_string())?;
                args.density_class = Some(parse_density_class(&value)?);
            }
            "--reduced-motion-posture" => {
                let value = iter.next().ok_or_else(|| {
                    "--reduced-motion-posture requires a posture token".to_string()
                })?;
                args.reduced_motion_posture = Some(parse_accessibility_posture(&value)?);
            }
            "--ui-theme" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--ui-theme requires light | dark | system".to_string())?;
                args.ui_theme = Some(parse_ui_theme_setting(&value)?.to_string());
            }
            "--ui-density" => {
                let value = iter.next().ok_or_else(|| {
                    "--ui-density requires compact | comfortable | spacious".to_string()
                })?;
                args.ui_density = Some(parse_ui_density_setting(&value)?.to_string());
            }
            "--ui-motion" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--ui-motion requires full | reduced | none".to_string())?;
                args.ui_motion = Some(parse_ui_motion_setting(&value)?.to_string());
            }
            "--disable-clipboard" => args.disable_clipboard = true,
            "--renderer" => {
                let value = iter.next().ok_or_else(|| {
                    "--renderer requires a backend name (gpu | software)".to_string()
                })?;
                args.renderer = match value.as_str() {
                    "gpu" => ShellRendererChoice::Gpu,
                    "software" => ShellRendererChoice::Software,
                    other => {
                        return Err(format!("unknown renderer backend: {other}\n\n{}", usage()))
                    }
                };
            }
            "--help" | "-h" => return Err(usage()),
            other if other.starts_with('-') => {
                return Err(format!("unknown argument: {other}\n\n{}", usage()))
            }
            other => {
                if args.open_workspace_path.is_some() {
                    return Err(format!(
                        "unexpected positional argument: {other}\n\n{}",
                        usage()
                    ));
                }
                args.open_workspace_path = Some(resolve_open_workspace_path(other)?);
            }
        }
    }
    Ok(args)
}

fn usage() -> String {
    "aureline_shell — Aureline desktop shell\n\n\
     Usage:\n\
     \taureline_shell\n\
     \taureline_shell --open <folder>\n\
     \taureline_shell <folder>\n\
     \taureline_shell --emit-startup-trace <path> [--exit-after-first-frame] [--disable-clipboard]\n\
     \taureline_shell --emit-onboarding-alpha <path>\n\
     \taureline_shell --open <folder> --headless-test-edit-save <file> --headless-test-write-hex <hex> [--headless-test-report <path>]\n\
     \taureline_shell --emit-hot-path-metrics <path>\n\
     \taureline_shell --emit-screenshot <path> [--theme-class <token>] [--density-class <token>] [--reduced-motion-posture <token>] [--ui-theme <light|dark|system>] [--ui-density <compact|comfortable|spacious>] [--ui-motion <full|reduced|none>] [--window-size <WxH>] [--renderer (gpu|software)]\n\
     \taureline_shell --renderer (gpu|software)\n"
        .to_string()
}

fn resolve_open_workspace_path(path: impl AsRef<Path>) -> Result<PathBuf, String> {
    let path = path.as_ref();
    let metadata = std::fs::metadata(path).map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            format!("workspace path does not exist: {}", path.display())
        } else {
            format!(
                "workspace path is not accessible: {} ({err})",
                path.display()
            )
        }
    })?;
    if !metadata.is_dir() {
        return Err(format!(
            "workspace path is not a folder: {}",
            path.display()
        ));
    }
    path.canonicalize().map_err(|err| {
        format!(
            "workspace path could not be canonicalized: {} ({err})",
            path.display()
        )
    })
}

fn run_headless_edit_save(args: &NativeShellArgs) -> Result<(), String> {
    let workspace_root = args
        .open_workspace_path
        .as_ref()
        .ok_or_else(|| "--headless-test-edit-save requires --open <folder>".to_string())?;
    let request = &args.headless_edit_save;
    let target_arg = request
        .file_path
        .as_ref()
        .ok_or_else(|| "--headless-test-edit-save requires a file path".to_string())?;
    let write_hex = request
        .write_hex
        .as_deref()
        .ok_or_else(|| "--headless-test-write-hex is required".to_string())?;
    let bytes = parse_hex_bytes(write_hex)?;
    let text = std::str::from_utf8(&bytes).map_err(|err| {
        format!("--headless-test-write-hex must decode to UTF-8 text for this editor path: {err}")
    })?;

    let canonical_workspace = workspace_root
        .canonicalize()
        .map_err(|err| format!("workspace path could not be canonicalized: {err}"))?;
    let target_path = if target_arg.is_absolute() {
        target_arg.clone()
    } else {
        canonical_workspace.join(target_arg)
    };
    let canonical_target = target_path.canonicalize().map_err(|err| {
        format!(
            "headless edit target is not accessible: {} ({err})",
            target_path.display()
        )
    })?;
    if !canonical_target.starts_with(&canonical_workspace) {
        return Err(format!(
            "headless edit target must stay inside workspace: {}",
            canonical_target.display()
        ));
    }

    let mut frame = DesktopFrame::new(1280, 720);
    let group = frame.focused_editor_group();
    let tab = frame
        .open_tab()
        .ok_or_else(|| "headless editor tab could not be opened".to_string())?;
    let mut editor_runtime = EditorWorkspaceRuntimeState::with_log_root(
        headless_edit_save_log_root(request.report_path.as_deref()),
    );
    editor_runtime.open_file(group, tab, &canonical_target)?;
    if editor_runtime
        .tab_render_info(group, tab)
        .is_some_and(|info| info.large_file_state.is_some())
    {
        editor_runtime.open_anyway(group, tab)?;
    }
    editor_runtime.replace_tab_contents(group, tab, text, "headless_test_edit_save")?;
    let save = editor_runtime.save_tab(group, tab)?;
    let (outcome, write_strategy, committed) = match save {
        SaveTabAttempt::Saved(result) => (
            result.manifest.outcome.as_str().to_string(),
            result.write_strategy.as_str().to_string(),
            result.committed(),
        ),
        SaveTabAttempt::NoTarget => ("no_target".to_string(), "none".to_string(), false),
        SaveTabAttempt::ReviewRequired { outcome, .. } => {
            (outcome.as_str().to_string(), "blocked".to_string(), false)
        }
    };

    if !committed {
        return Err(format!("headless edit/save did not commit: {outcome}"));
    }

    if let Some(report_path) = request.report_path.as_ref() {
        if let Some(parent) = report_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|err| format!("create headless report directory failed: {err}"))?;
        }
        let payload = serde_json::json!({
            "schema_version": 1,
            "mode": "headless_edit_save",
            "workspace_root": canonical_workspace.display().to_string(),
            "target_path": canonical_target.display().to_string(),
            "byte_count": bytes.len(),
            "outcome": outcome,
            "write_strategy": write_strategy,
            "exact_build_identity_ref": build_info::exact_build_identity_ref(),
        });
        let json = serde_json::to_string_pretty(&payload)
            .map_err(|err| format!("serialize headless edit/save report failed: {err}"))?;
        std::fs::write(report_path, format!("{json}\n"))
            .map_err(|err| format!("write headless edit/save report failed: {err}"))?;
    }

    Ok(())
}

fn parse_hex_bytes(value: &str) -> Result<Vec<u8>, String> {
    let value = value.trim();
    if value.is_empty() {
        return Err("hex payload must not be empty".to_string());
    }
    if value.len() % 2 != 0 {
        return Err("hex payload must contain an even number of characters".to_string());
    }

    let mut bytes = Vec::with_capacity(value.len() / 2);
    for (idx, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let high = hex_nibble(pair[0]).ok_or_else(|| {
            format!(
                "hex payload contains non-hex characters at byte {}",
                idx * 2
            )
        })?;
        let low = hex_nibble(pair[1]).ok_or_else(|| {
            format!(
                "hex payload contains non-hex characters at byte {}",
                idx * 2 + 1
            )
        })?;
        bytes.push((high << 4) | low);
    }
    Ok(bytes)
}

fn headless_edit_save_log_root(report_path: Option<&Path>) -> PathBuf {
    if let Some(parent) = report_path.and_then(Path::parent) {
        return parent.join("headless_logs");
    }
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    env::temp_dir().join(format!(
        "aureline-headless-edit-save-{}-{}",
        std::process::id(),
        duration.as_nanos()
    ))
}

fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

#[cfg(test)]
mod native_shell_arg_tests {
    use super::*;
    use std::fmt::Write as _;

    #[test]
    fn open_flag_canonicalizes_existing_folder() {
        let dir = tempfile::tempdir().expect("tempdir");
        let parsed =
            parse_native_shell_args_from(["--open", dir.path().to_str().expect("utf-8 temp path")])
                .expect("parse args");

        assert_eq!(
            parsed.open_workspace_path,
            Some(dir.path().canonicalize().expect("canonical tempdir"))
        );
    }

    #[test]
    fn positional_folder_canonicalizes_first_non_flag_argument() {
        let dir = tempfile::tempdir().expect("tempdir");
        let parsed = parse_native_shell_args_from([dir.path().to_str().expect("utf-8 temp path")])
            .expect("parse args");

        assert_eq!(
            parsed.open_workspace_path,
            Some(dir.path().canonicalize().expect("canonical tempdir"))
        );
    }

    #[test]
    fn open_flag_rejects_missing_path() {
        let dir = tempfile::tempdir().expect("tempdir");
        let missing = dir.path().join("missing");
        let err =
            parse_native_shell_args_from(vec!["--open".to_string(), missing.display().to_string()])
                .expect_err("missing path should be rejected");

        assert!(
            err.contains("workspace path does not exist"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn duplicate_positional_folder_is_rejected() {
        let first = tempfile::tempdir().expect("first tempdir");
        let second = tempfile::tempdir().expect("second tempdir");
        let err = parse_native_shell_args_from(vec![
            first.path().display().to_string(),
            second.path().display().to_string(),
        ])
        .expect_err("second positional path should be rejected");

        assert!(
            err.contains("unexpected positional argument"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn headless_edit_save_requires_open_root_at_runtime() {
        let parsed = parse_native_shell_args_from([
            "--headless-test-edit-save",
            "README.md",
            "--headless-test-write-hex",
            "6869",
        ])
        .expect("parse headless edit/save args");

        assert!(parsed.headless_edit_save.is_requested());
        let err = run_headless_edit_save(&parsed).expect_err("missing open root should fail");
        assert!(
            err.contains("--headless-test-edit-save requires --open"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn headless_edit_save_rejects_invalid_hex() {
        let dir = tempfile::tempdir().expect("tempdir");
        let parsed = parse_native_shell_args_from(vec![
            "--open".to_string(),
            dir.path().display().to_string(),
            "--headless-test-edit-save".to_string(),
            "README.md".to_string(),
            "--headless-test-write-hex".to_string(),
            "abc".to_string(),
        ])
        .expect("parse headless edit/save args");

        let err = run_headless_edit_save(&parsed).expect_err("odd hex length should fail");
        assert!(
            err.contains("even number of characters"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn headless_edit_save_commits_bytes_and_report() {
        let dir = tempfile::tempdir().expect("tempdir");
        let file_path = dir.path().join("notes.md");
        std::fs::write(&file_path, b"old\n").expect("seed file");
        let report_path = dir.path().join("headless_report.json");
        let payload = b"known-byte-sequence\n";
        let parsed = parse_native_shell_args_from(vec![
            "--open".to_string(),
            dir.path().display().to_string(),
            "--headless-test-edit-save".to_string(),
            "notes.md".to_string(),
            "--headless-test-write-hex".to_string(),
            hex_encode(payload),
            "--headless-test-report".to_string(),
            report_path.display().to_string(),
        ])
        .expect("parse headless edit/save args");

        run_headless_edit_save(&parsed).expect("headless edit/save should commit");

        let on_disk = std::fs::read(&file_path).expect("read saved file");
        assert_eq!(on_disk, payload);
        let report: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(&report_path).expect("read report"))
                .expect("report json");
        assert_eq!(report["outcome"], "committed");
        assert_eq!(report["byte_count"], payload.len());
    }

    fn hex_encode(bytes: &[u8]) -> String {
        let mut encoded = String::with_capacity(bytes.len().saturating_mul(2));
        for byte in bytes {
            write!(&mut encoded, "{byte:02x}").expect("hex encoding should write to string");
        }
        encoded
    }
}

fn parse_window_size(value: &str) -> Result<(f64, f64), String> {
    let value = value.trim();
    let Some((w, h)) = value.split_once('x') else {
        return Err(format!(
            "invalid --window-size value: {value:?} (expected <width>x<height>)"
        ));
    };
    let width: f64 = w
        .trim()
        .parse()
        .map_err(|_| format!("invalid --window-size width: {w:?} (expected numeric value)"))?;
    let height: f64 = h
        .trim()
        .parse()
        .map_err(|_| format!("invalid --window-size height: {h:?} (expected numeric value)"))?;
    if !width.is_finite() || !height.is_finite() || width <= 0.0 || height <= 0.0 {
        return Err(format!(
            "invalid --window-size value: {value:?} (width and height must be positive)"
        ));
    }
    Ok((width, height))
}

fn parse_theme_class(value: &str) -> Result<ThemeClass, String> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "dark_reference" => Ok(ThemeClass::DarkReference),
        "light_parity" => Ok(ThemeClass::LightParity),
        "high_contrast_dark" => Ok(ThemeClass::HighContrastDark),
        "high_contrast_light" => Ok(ThemeClass::HighContrastLight),
        _ => Err(format!(
            "invalid theme class token: {value:?} (expected dark_reference | light_parity | high_contrast_dark | high_contrast_light)"
        )),
    }
}

fn parse_density_class(value: &str) -> Result<DensityClass, String> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "compact" => Ok(DensityClass::Compact),
        "standard" => Ok(DensityClass::Standard),
        "comfortable" => Ok(DensityClass::Comfortable),
        _ => Err(format!(
            "invalid density class token: {value:?} (expected compact | standard | comfortable)"
        )),
    }
}

fn parse_accessibility_posture(value: &str) -> Result<AccessibilityPostureClass, String> {
    let normalized = value.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "motion_standard" => Ok(AccessibilityPostureClass::MotionStandard),
        "motion_reduced" => Ok(AccessibilityPostureClass::MotionReduced),
        "motion_low_motion" => Ok(AccessibilityPostureClass::MotionLowMotion),
        "motion_power_saver" => Ok(AccessibilityPostureClass::MotionPowerSaver),
        "motion_critical_hot_path" => Ok(AccessibilityPostureClass::MotionCriticalHotPath),
        _ => Err(format!(
            "invalid accessibility posture token: {value:?} (expected motion_standard | motion_reduced | motion_low_motion | motion_power_saver | motion_critical_hot_path)"
        )),
    }
}

fn parse_ui_theme_setting(value: &str) -> Result<&'static str, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "light" => Ok(UI_THEME_LIGHT),
        "dark" => Ok(UI_THEME_DARK),
        "system" | "auto" => Ok(UI_THEME_SYSTEM),
        _ => Err(format!(
            "invalid UI theme setting: {value:?} (expected light | dark | system)"
        )),
    }
}

fn parse_ui_density_setting(value: &str) -> Result<&'static str, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "compact" => Ok(UI_DENSITY_COMPACT),
        "comfortable" | "standard" => Ok(UI_DENSITY_COMFORTABLE),
        "spacious" => Ok(UI_DENSITY_SPACIOUS),
        _ => Err(format!(
            "invalid UI density setting: {value:?} (expected compact | comfortable | spacious)"
        )),
    }
}

fn parse_ui_motion_setting(value: &str) -> Result<&'static str, String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "full" | "motion_standard" => Ok(UI_MOTION_FULL),
        "reduced" | "motion_reduced" => Ok(UI_MOTION_REDUCED),
        "none" | "motion_low_motion" | "motion_critical_hot_path" => Ok(UI_MOTION_NONE),
        _ => Err(format!(
            "invalid UI motion setting: {value:?} (expected full | reduced | none)"
        )),
    }
}

#[derive(Debug)]
enum ShellRenderBackend {
    Gpu {
        renderer: WgpuBlitRenderer,
        retained_frame: Vec<u32>,
        last_size: (u32, u32),
    },
    Software {
        surface: SoftbufferSurface,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ShellDamageHint {
    None,
    FullWindow,
    Rect {
        layer: CompositionLayerId,
        class: DamageClassId,
        rect: Rect,
    },
}

#[derive(Debug, Default, Clone)]
struct ShellDamageGeometryCache {
    command_palette_panel: Option<Rect>,
    command_palette_query: Option<Rect>,
    focused_editor_group: Option<Rect>,
    focused_editor_viewport: Option<Rect>,
}

fn enqueue_damage_hint(scheduler: &mut FrameScheduler, hint: ShellDamageHint) {
    match hint {
        ShellDamageHint::None => {}
        ShellDamageHint::FullWindow => {
            scheduler.invalidate(DamageEvent::new(
                CompositionLayerId::WindowChromeBase,
                DamageClassId::WindowExposedRegionRefresh,
            ));
            scheduler.invalidate(DamageEvent::new(
                CompositionLayerId::TextAndDecoration,
                DamageClassId::WindowExposedRegionRefresh,
            ));
        }
        ShellDamageHint::Rect { layer, class, rect } => {
            if rect.is_empty() {
                return;
            }
            let pixel_rect = PixelRect::new(rect.x, rect.y, rect.width, rect.height);
            scheduler.invalidate(DamageEvent::with_region(
                layer,
                class,
                DamageRegion::Rect(pixel_rect),
            ));
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ShellFocusReturnTarget {
    zone: ShellZoneId,
    editor_group: PaneId,
}

impl ShellFocusReturnTarget {
    fn capture(frame: &DesktopFrame) -> Self {
        Self {
            zone: frame.focused_zone(),
            editor_group: frame.focused_editor_group(),
        }
    }

    fn apply(self, frame: &mut DesktopFrame) {
        frame.focus_zone(self.zone);
        if self.zone == ShellZoneId::MainWorkspace {
            frame.focus_editor_group(self.editor_group);
        }
    }
}

pub fn run_native_shell() -> Result<(), Box<dyn std::error::Error>> {
    let args = parse_native_shell_args()
        .map_err(|message| -> Box<dyn std::error::Error> { message.into() })?;
    if let Some(path) = args.emit_onboarding_alpha_path.as_ref() {
        return crate::onboarding::write_onboarding_alpha_export(
            path,
            aureline_commands::invocation::now_rfc3339(),
        );
    }
    if args.headless_edit_save.is_requested() {
        return run_headless_edit_save(&args)
            .map_err(|message| -> Box<dyn std::error::Error> { message.into() });
    }
    let capture_a11y_tree = env::var("AURELINE_CAPTURE_A11Y_TREE")
        .ok()
        .map(|value| {
            let normalized = value.trim().to_ascii_lowercase();
            matches!(normalized.as_str(), "1" | "true" | "yes" | "on")
        })
        .unwrap_or(false);
    let mut startup_trace = StartupTrace::new(args.startup_trace);
    let identity = build_info::build_identity();
    let hot_path_trace_id = format!(
        "trace.shell.hot_path:{}:{}",
        identity.commit_short, identity.build_timestamp_utc
    );
    let hot_path_build = TelemetryBuildIdentityRecord {
        crate_name: env!("CARGO_PKG_NAME").to_string(),
        crate_version: env!("CARGO_PKG_VERSION").to_string(),
        rustc_target_triple: identity.target_triple.clone(),
    };
    let host_os = if cfg!(target_os = "macos") {
        Cow::Borrowed("macos")
    } else if cfg!(target_os = "windows") {
        Cow::Borrowed("windows")
    } else if cfg!(target_os = "linux") {
        Cow::Borrowed("linux")
    } else {
        Cow::Borrowed("unknown")
    };
    let hot_path_context = HotPathMetricsContext {
        trace_id: hot_path_trace_id,
        backend: Cow::Borrowed("native_window"),
        host_os,
        build: hot_path_build,
        exact_build_identity_ref: Some(build_info::exact_build_identity_ref()),
        hardware_definition_ref: None,
        environment_ref: None,
        fixture_ref: None,
        corpus_manifest: None,
        sampling_profile: Cow::Borrowed("developer_local"),
        sampling_profile_ref: "profile.trace_sampling.developer_local".to_string(),
        retention_class: Cow::Borrowed("hot_path_volatile"),
        export_posture: Cow::Borrowed("excluded_by_default"),
        redaction_class: Cow::Borrowed("metadata_safe_default"),
    };
    let mut hot_path_metrics = HotPathMetrics::new(args.hot_path_metrics, hot_path_context);
    let clock = WallClock::new();
    let mut scheduler = FrameScheduler::new();
    let event_loop = EventLoop::new()?;
    let registry = seeded_registry();
    let (default_width, default_height) = (1920.0, 1080.0);
    let (window_width, window_height) = args.window_size.unwrap_or((default_width, default_height));
    let mut title_context_bar = TitleContextBarRuntimeState::new();
    let window = WinitWindow::new(
        &event_loop,
        window_title(title_context_bar.record()),
        LogicalSize::new(window_width, window_height),
    )?
    .into_arc();
    let mut display_safety = DisplaySafetyGuard::new();

    let mut render_backend = match args.renderer {
        ShellRendererChoice::Software => ShellRenderBackend::Software {
            surface: create_softbuffer_surface(window.clone())?,
        },
        ShellRendererChoice::Gpu => match WgpuBlitRenderer::new(window.clone()) {
            Ok(renderer) => {
                let size = window.inner_size();
                ShellRenderBackend::Gpu {
                    renderer,
                    retained_frame: Vec::new(),
                    last_size: (size.width, size.height),
                }
            }
            Err(err) => {
                scheduler
                    .note_degraded_renderer(format!("gpu backend unavailable — {err}"), &clock);
                ShellRenderBackend::Software {
                    surface: create_softbuffer_surface(window.clone())?,
                }
            }
        },
    };

    let mut frame = {
        let logical = window.inner_size().to_logical::<u32>(window.scale_factor());
        DesktopFrame::new(logical.width, logical.height)
    };
    startup_trace.mark(StartupMilestone::EditorSurfaceReady);
    hot_path_metrics.mark_editor_surface_ready(clock.now().0);

    let mut held_modifiers = HeldModifiers::default();
    let mut damage_geometry = ShellDamageGeometryCache::default();
    let mut last_cursor_pos: Option<(u32, u32)> = None;
    let mut palette = CommandPaletteState::new(registry);
    let mut palette_focus_return: FocusReturnStack<ShellFocusReturnTarget> =
        FocusReturnStack::new();
    let mut start_center = StartCenterState::new();
    let mut overlay: Option<ShellOverlayState> = None;
    let mut command_runtime = CommandRuntimeState::default();
    let clone_startup_probe = SystemGitCloneBackend::default().probe();
    if let Err(err) = clone_startup_probe.as_ref() {
        command_runtime.note_non_command_action(format!(
            "clone unavailable - {} ({})",
            err.class.as_str(),
            err.message
        ));
    }
    let mut clone_jobs = CloneJobRuntimeState::new(clone_startup_probe);
    let mut keybinding_runtime = KeybindingRuntimeState::new(platform_class_for_shell());
    let mut enablement_runtime = CommandEnablementRuntimeState::default();
    palette.set_labs_enabled(registry, enablement_runtime.labs_enabled);
    let mut recent_work = RecentWorkRuntimeState::load();
    let mut activity_center = ActivityCenterRuntimeState::load(&recent_work.store_path);
    let mut workspace_lifecycle = WorkspaceLifecycleRuntimeState::new();
    let mut clipboard = ClipboardState::new(!args.disable_clipboard);
    let mut appearance = AppearanceRuntimeState::load();
    appearance.apply_cli_overrides(
        args.theme_class,
        args.density_class,
        args.reduced_motion_posture,
        args.ui_theme.as_deref(),
        args.ui_density.as_deref(),
        args.ui_motion.as_deref(),
    );
    let docs_help_boundary_card =
        seeded_docs_help_boundary_card(build_info::exact_build_identity_ref());
    let mut text_runtime = ShellTextRuntime::new();
    let mut editor_runtime = EditorWorkspaceRuntimeState::new();
    let recovery_root = PathBuf::from(".logs").join("recovery");
    let (mut crash_marker_guard, prior_run_abnormal) =
        match CrashMarkerGuard::begin(&recovery_root, &mono_timestamp_now()) {
            Ok((guard, outcome)) => (Some(guard), outcome.prior_run_abnormal),
            Err(err) => {
                command_runtime
                    .note_non_command_action(format!("crash marker unavailable — {err}"));
                (None, false)
            }
        };
    let mut session_restore_store = SessionRestoreStore::new(&recovery_root);

    {
        let crash_journal_reader = CrashJournalStore::new(&recovery_root);
        match RestoreProposal::build(
            &session_restore_store,
            &crash_journal_reader,
            prior_run_abnormal,
        ) {
            Ok(proposal) => {
                if let Err(err) = write_restore_proposal_log(&recovery_root, &proposal) {
                    command_runtime.note_non_command_action(format!(
                        "restore proposal log unavailable — {err}"
                    ));
                }
                let prompt = materialize_restore_prompt(&proposal);
                if let Err(err) = write_restore_prompt_log(&recovery_root, &prompt) {
                    command_runtime
                        .note_non_command_action(format!("restore prompt log unavailable — {err}"));
                }
                if prior_run_abnormal && !prompt.is_empty() {
                    command_runtime.note_non_command_action(restore_prompt_status_line(&prompt));
                } else if prior_run_abnormal {
                    command_runtime.note_non_command_action(
                        "prior run terminated abnormally; nothing to restore",
                    );
                }
            }
            Err(err) => {
                command_runtime
                    .note_non_command_action(format!("restore proposal unavailable — {err}"));
            }
        }
    }

    let mut last_a11y_fingerprint: Option<Vec<u8>> = None;

    startup_trace.mark(StartupMilestone::FirstInteractiveShell);
    hot_path_metrics.mark_first_interactive_shell(clock.now().0);

    let outcome = display_safety.poll_and_apply(&window);
    if let Some(adjustment) = outcome.adjustment {
        command_runtime.note_non_command_action("layout adjusted — moved window into safe bounds");
        let record = materialize_adjustment_record(&window, &adjustment);
        write_display_safety_log(&record);
        scheduler.invalidate(DamageEvent::new(
            CompositionLayerId::WindowChromeBase,
            DamageClassId::ViewportResizeOrScaleChange,
        ));
        scheduler.invalidate(DamageEvent::new(
            CompositionLayerId::TextAndDecoration,
            DamageClassId::ViewportResizeOrScaleChange,
        ));
        window.request_redraw();
    } else if !outcome.topology_change_classes.is_empty() {
        let record = materialize_topology_record(&window, &outcome.topology_change_classes, None);
        write_display_safety_topology_log(&record);
    }

    if let Some(err) = recent_work.last_error.as_deref() {
        command_runtime
            .note_non_command_action(format!("recent work registry unavailable — {err}"));
    }
    if let Some(err) = activity_center.last_error.as_deref() {
        command_runtime.note_non_command_action(err.to_string());
    }
    if let Some(err) = activity_center.notifications.last_error.as_deref() {
        command_runtime.note_non_command_action(err.to_string());
    }
    if let Some(err) = appearance.last_error.as_deref() {
        command_runtime.note_non_command_action(format!("appearance session unavailable — {err}"));
    }
    if let Some(open_workspace_path) = args.open_workspace_path.as_deref() {
        open_local_folder_workspace(
            open_workspace_path,
            LocalFolderOpenSource::CliArgument,
            &mut command_runtime,
            &mut frame,
            &mut editor_runtime,
            &mut palette,
            &enablement_runtime,
            &mut workspace_lifecycle,
            &mut recent_work,
            &mut activity_center,
        );
    }

    scheduler.invalidate(DamageEvent::new(
        CompositionLayerId::WindowChromeBase,
        DamageClassId::StartupFirstPaint,
    ));
    scheduler.invalidate(DamageEvent::new(
        CompositionLayerId::TextAndDecoration,
        DamageClassId::StartupFirstPaint,
    ));
    if scheduler.decision() == FrameSchedulerDecision::RequestRedraw {
        window.request_redraw();
    }

    fn maybe_capture_shell_accessibility_tree(
        enabled: bool,
        last_fingerprint: &mut Option<Vec<u8>>,
        scheduler: &mut FrameScheduler,
        clock: &WallClock,
        registry: &CommandRegistry,
        shortcuts_by_command_id: &HashMap<String, Vec<String>>,
        frame: &DesktopFrame,
        palette: &CommandPaletteState,
        start_center: &StartCenterState,
        docs_help_boundary_card: &EmbeddedBoundaryCardRecord,
        enablement_runtime: &CommandEnablementRuntimeState,
    ) {
        if !enabled {
            return;
        }

        let enablement = ShellA11yEnablementContext {
            client_scope: "desktop_product",
            workspace_trust_state: enablement_runtime.workspace_trust_state.as_str(),
            execution_context_available: enablement_runtime.execution_context_available,
            provider_linked: enablement_runtime.provider_linked,
            credential_available: enablement_runtime.credential_available,
            policy_disabled: enablement_runtime.policy_disabled,
            policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
            labs_enabled: enablement_runtime.labs_enabled,
        };

        let snapshot = materialize_shell_accessibility_tree(
            registry,
            shortcuts_by_command_id,
            frame,
            palette,
            start_center,
            docs_help_boundary_card,
            enablement,
        );

        let pinned_timestamp = "1970-01-01T00:00:00Z".to_string();
        let mut pinned = snapshot.clone();
        pinned.minted_at = pinned_timestamp.clone();
        for node in pinned.nodes.iter_mut() {
            node.minted_at = pinned_timestamp.clone();
        }

        let Ok(fingerprint) = serde_json::to_vec(&pinned) else {
            return;
        };
        if last_fingerprint.as_ref() == Some(&fingerprint) {
            return;
        }

        *last_fingerprint = Some(fingerprint);
        scheduler.mark_hook(Hook::AccessibilityTreeUpdate, clock);
        write_shell_accessibility_tree_log(&snapshot);
    }

    let screenshot_path = args.screenshot_path.clone();

    event_loop.run(move |event, elwt| match event {
        Event::AboutToWait => {
            let now = Instant::now();
            let outcome = display_safety.poll_and_apply(&window);
            if let Some(adjustment) = outcome.adjustment {
                command_runtime
                    .note_non_command_action("layout adjusted — moved window into safe bounds");
                let record = materialize_adjustment_record(&window, &adjustment);
                write_display_safety_log(&record);
                relayout_and_redraw(&window, &mut render_backend, &mut frame, &mut scheduler);
                window.request_redraw();
            } else if !outcome.topology_change_classes.is_empty() {
                let bounds = window.outer_position().ok().map(|position| {
                    let size = window.outer_size();
                    crate::windowing::display_safety::PhysicalRect::from_position_size(
                        position, size,
                    )
                });
                let record =
                    materialize_topology_record(&window, &outcome.topology_change_classes, bounds);
                write_display_safety_topology_log(&record);

                if outcome.topology_change_classes.contains(&"scale_changed") {
                    relayout_and_redraw(&window, &mut render_backend, &mut frame, &mut scheduler);
                    window.request_redraw();
                }
            }
            let palette_changed =
                palette.tick(registry, &keybinding_runtime.shortcuts_by_command_id, now);
            if palette_changed && palette.is_open() {
                if let Some(rect) = damage_geometry.command_palette_panel {
                    enqueue_damage_hint(
                        &mut scheduler,
                        ShellDamageHint::Rect {
                            layer: CompositionLayerId::FloatingSurface,
                            class: DamageClassId::FloatingSurfaceToggle,
                            rect,
                        },
                    );
                } else {
                    scheduler.invalidate(DamageEvent::new(
                        CompositionLayerId::FloatingSurface,
                        DamageClassId::FloatingSurfaceToggle,
                    ));
                }
            }
            if editor_runtime
                .explorer
                .apply_watcher_events(palette.take_workspace_watcher_events())
            {
                if let Some(rect) = frame.layout().zone(ShellZoneId::LeftSidebar) {
                    enqueue_damage_hint(
                        &mut scheduler,
                        ShellDamageHint::Rect {
                            layer: CompositionLayerId::TextAndDecoration,
                            class: DamageClassId::TextReflowLocal,
                            rect: to_physical_rect(rect, window.scale_factor()),
                        },
                    );
                } else {
                    enqueue_damage_hint(&mut scheduler, ShellDamageHint::FullWindow);
                }
            }
            let file_index_readiness = palette.workspace_file_index_readiness();
            if workspace_lifecycle.update_from_file_index(
                file_index_readiness.map(|v| v.watcher_health),
                file_index_readiness.map(|v| v.hot_index_ready),
            ) {
                enqueue_damage_hint(
                    &mut scheduler,
                    ShellDamageHint::Rect {
                        layer: CompositionLayerId::WindowChromeBase,
                        class: DamageClassId::TextReflowLocal,
                        rect: to_physical_rect(frame.layout().status_bar, window.scale_factor()),
                    },
                );
            }
            if let (Some(readiness), Some(root)) = (file_index_readiness, palette.workspace_root())
            {
                if activity_center.note_quick_open_file_index(
                    recent_work.active_recent_work_id.as_deref(),
                    root,
                    readiness.hot_index_ready,
                ) {
                    enqueue_damage_hint(
                        &mut scheduler,
                        ShellDamageHint::Rect {
                            layer: CompositionLayerId::WindowChromeBase,
                            class: DamageClassId::TextReflowLocal,
                            rect: to_physical_rect(
                                frame.layout().activity_rail,
                                window.scale_factor(),
                            ),
                        },
                    );
                }
            }

            let workspace_root = workspace_lifecycle
                .machine
                .as_ref()
                .and_then(|_| palette.workspace_root());
            if title_context_bar.update(TitleContextBarRuntimeInputs {
                workspace_label: recent_work.active_workspace_label(),
                workspace_root,
                workspace_lifecycle: workspace_lifecycle.machine.as_ref(),
                workspace_trust_state_token: enablement_runtime.workspace_trust_state.as_str(),
            }) {
                window.set_title(&window_title(title_context_bar.record()));
                enqueue_damage_hint(
                    &mut scheduler,
                    ShellDamageHint::Rect {
                        layer: CompositionLayerId::WindowChromeBase,
                        class: DamageClassId::TextReflowLocal,
                        rect: to_physical_rect(
                            frame.layout().title_context_bar,
                            window.scale_factor(),
                        ),
                    },
                );
                enqueue_damage_hint(
                    &mut scheduler,
                    ShellDamageHint::Rect {
                        layer: CompositionLayerId::WindowChromeBase,
                        class: DamageClassId::TextReflowLocal,
                        rect: to_physical_rect(frame.layout().status_bar, window.scale_factor()),
                    },
                );
            }
            if editor_runtime
                .terminal_pane
                .drain_outputs(&mono_timestamp_now())
            {
                if let Some(rect) = frame.layout().bottom_panel {
                    enqueue_damage_hint(
                        &mut scheduler,
                        ShellDamageHint::Rect {
                            layer: CompositionLayerId::TextAndDecoration,
                            class: DamageClassId::TextReflowLocal,
                            rect: to_physical_rect(rect, window.scale_factor()),
                        },
                    );
                } else {
                    enqueue_damage_hint(&mut scheduler, ShellDamageHint::FullWindow);
                }
            }
            if poll_clone_jobs(
                &mut clone_jobs,
                &mut command_runtime,
                &mut frame,
                &mut editor_runtime,
                &mut palette,
                &mut overlay,
                &enablement_runtime,
                &mut workspace_lifecycle,
                &mut recent_work,
                &mut activity_center,
            ) {
                enqueue_damage_hint(&mut scheduler, ShellDamageHint::FullWindow);
            }
            let mut next_deadline = palette.next_wake_deadline(now);
            if editor_runtime.terminal_pane.has_live_sessions() {
                let terminal_poll_deadline = now + Duration::from_millis(50);
                next_deadline = Some(match next_deadline {
                    Some(existing) => existing.min(terminal_poll_deadline),
                    None => terminal_poll_deadline,
                });
            }
            if clone_jobs.has_active() {
                let clone_poll_deadline = now + Duration::from_millis(50);
                next_deadline = Some(match next_deadline {
                    Some(existing) => existing.min(clone_poll_deadline),
                    None => clone_poll_deadline,
                });
            }
            let animation_frame = now + Duration::from_millis(16);
            if activity_center.tick_notifications(now) {
                enqueue_damage_hint(&mut scheduler, ShellDamageHint::FullWindow);
            }
            if activity_center.notifications_need_animation() {
                enqueue_damage_hint(&mut scheduler, ShellDamageHint::FullWindow);
                next_deadline = Some(match next_deadline {
                    Some(existing) => existing.min(animation_frame),
                    None => animation_frame,
                });
            }
            if let Ok(token_registry) = seeded_token_registry(appearance.theme_class()) {
                let posture = appearance.reduced_motion_posture();
                if palette.is_open() {
                    let (_, _, duration) = overlay_dialog_enter_progress(
                        token_registry,
                        posture,
                        palette.opened_at(),
                        now,
                    );
                    if !duration.is_zero()
                        && now.saturating_duration_since(palette.opened_at()) < duration
                    {
                        enqueue_damage_hint(&mut scheduler, ShellDamageHint::FullWindow);
                        next_deadline = Some(match next_deadline {
                            Some(existing) => existing.min(animation_frame),
                            None => animation_frame,
                        });
                    }
                }

                if let Some(state) = overlay.as_ref() {
                    let (_, _, duration) = overlay_dialog_enter_progress(
                        token_registry,
                        posture,
                        state.opened_at,
                        now,
                    );
                    if !duration.is_zero()
                        && now.saturating_duration_since(state.opened_at) < duration
                    {
                        enqueue_damage_hint(&mut scheduler, ShellDamageHint::FullWindow);
                        next_deadline = Some(match next_deadline {
                            Some(existing) => existing.min(animation_frame),
                            None => animation_frame,
                        });
                    }
                }
            }
            if scheduler.decision() == FrameSchedulerDecision::RequestRedraw {
                window.request_redraw();
            }
            if let Some(deadline) = next_deadline {
                elwt.set_control_flow(ControlFlow::WaitUntil(deadline));
            } else {
                elwt.set_control_flow(ControlFlow::Wait);
            }
        }
        Event::WindowEvent { window_id, event } if window_id == window.id() => match event {
            WindowEvent::CloseRequested => {
                let _ = hot_path_metrics.write_if_configured();
                activity_center.persist_clean_shutdown();
                editor_runtime
                    .terminal_pane
                    .close_active_workspace(&mono_timestamp_now(), Some("window_closed"));
                if let Some(guard) = crash_marker_guard.as_mut() {
                    if let Err(err) = guard.mark_clean_shutdown() {
                        command_runtime.note_non_command_action(format!(
                            "clean shutdown marker clear failed — {err}"
                        ));
                    }
                }
                elwt.exit();
            }
            WindowEvent::Resized(_) => {
                relayout_and_redraw(&window, &mut render_backend, &mut frame, &mut scheduler);
            }
            WindowEvent::ScaleFactorChanged { .. } => {
                relayout_and_redraw(&window, &mut render_backend, &mut frame, &mut scheduler);
            }
            WindowEvent::Occluded(occluded) => {
                scheduler.set_occluded(occluded, &clock);
                if !occluded {
                    scheduler.invalidate(DamageEvent::new(
                        CompositionLayerId::WindowChromeBase,
                        DamageClassId::WindowExposedRegionRefresh,
                    ));
                    scheduler.invalidate(DamageEvent::new(
                        CompositionLayerId::TextAndDecoration,
                        DamageClassId::WindowExposedRegionRefresh,
                    ));
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                let x = position.x.round().max(0.0) as u32;
                let y = position.y.round().max(0.0) as u32;
                last_cursor_pos = Some((x, y));
            }
            WindowEvent::MouseWheel { delta, .. } => {
                if frame.focused_zone() != ShellZoneId::MainWorkspace
                    || palette.is_open()
                    || overlay.is_some()
                {
                    return;
                }

                let focused = frame.focused_editor_group();
                let has_tabs = frame
                    .editor_group_layouts()
                    .into_iter()
                    .find(|g| g.group_id == focused)
                    .is_some_and(|g| g.tab_count > 0);
                if !has_tabs {
                    return;
                }

                let dy_lines = match delta {
                    MouseScrollDelta::LineDelta(_, y) => (-y).round() as i32,
                    MouseScrollDelta::PixelDelta(pos) => (-pos.y / 40.0).round() as i32,
                };
                if dy_lines == 0 {
                    return;
                }

                let Some(active_tab) = frame.active_tab_id(focused) else {
                    return;
                };
                let Some(viewport_rect) = damage_geometry
                    .focused_editor_viewport
                    .map(|rect| PixelRect::new(rect.x, rect.y, rect.width, rect.height))
                else {
                    return;
                };

                if !editor_runtime.has_tab_session(focused, active_tab) {
                    editor_runtime.open_placeholder(focused, active_tab);
                }

                if let Some(damage) = editor_runtime.apply_action(
                    focused,
                    active_tab,
                    &EditorAction::ScrollLines { dy_lines },
                    viewport_rect,
                ) {
                    if damage.hook == Hook::ScrollFrame {
                        hot_path_metrics.note_scroll_to_paint_admitted(clock.now().0);
                    }
                    scheduler.invalidate(damage.event);
                    scheduler.mark_hook(damage.hook, &clock);
                }
            }
            WindowEvent::Ime(ime) => {
                let should_update_ime_cursor_area = !matches!(&ime, Ime::Disabled);

                if palette.is_open() {
                    let changed = palette.handle_ime_event(
                        match ime {
                            Ime::Enabled => aureline_input::text_input::ImeEvent::Enabled,
                            Ime::Disabled => aureline_input::text_input::ImeEvent::Disabled,
                            Ime::Preedit(text, cursor) => {
                                aureline_input::text_input::ImeEvent::Preedit { text, cursor }
                            }
                            Ime::Commit(text) => {
                                aureline_input::text_input::ImeEvent::Commit { text }
                            }
                        },
                        registry,
                        &keybinding_runtime.shortcuts_by_command_id,
                    );

                    if changed {
                        if let Some(rect) = damage_geometry.command_palette_panel {
                            enqueue_damage_hint(
                                &mut scheduler,
                                ShellDamageHint::Rect {
                                    layer: CompositionLayerId::FloatingSurface,
                                    class: DamageClassId::TextReflowLocal,
                                    rect,
                                },
                            );
                        } else {
                            enqueue_damage_hint(&mut scheduler, ShellDamageHint::FullWindow);
                        }
                    }

                    if should_update_ime_cursor_area {
                        if let Some(rect) = damage_geometry.command_palette_query {
                            update_ime_cursor_area_for_rect(&window, rect);
                        }
                    }

                    return;
                }

                if frame.focused_zone() != ShellZoneId::MainWorkspace || overlay.is_some() {
                    return;
                }

                let focused = frame.focused_editor_group();
                let has_tabs = frame
                    .editor_group_layouts()
                    .into_iter()
                    .find(|g| g.group_id == focused)
                    .is_some_and(|g| g.tab_count > 0);
                if !has_tabs {
                    return;
                }

                let Some(active_tab) = frame.active_tab_id(focused) else {
                    return;
                };
                let Some(viewport_rect) = damage_geometry
                    .focused_editor_viewport
                    .map(|rect| PixelRect::new(rect.x, rect.y, rect.width, rect.height))
                else {
                    return;
                };

                if !editor_runtime.has_tab_session(focused, active_tab) {
                    editor_runtime.open_placeholder(focused, active_tab);
                }

                let normalized = {
                    let Some(session) = editor_runtime.tab_session_mut(focused, active_tab) else {
                        return;
                    };
                    session.text_input.handle_ime_event(match ime {
                        Ime::Enabled => aureline_input::text_input::ImeEvent::Enabled,
                        Ime::Disabled => aureline_input::text_input::ImeEvent::Disabled,
                        Ime::Preedit(text, cursor) => {
                            aureline_input::text_input::ImeEvent::Preedit { text, cursor }
                        }
                        Ime::Commit(text) => aureline_input::text_input::ImeEvent::Commit { text },
                    })
                };

                if let Some(normalized) = normalized {
                    let action = editor_action_from_text_input(normalized);
                    if let Some(damage) =
                        editor_runtime.apply_action(focused, active_tab, &action, viewport_rect)
                    {
                        if damage.hook == Hook::ReflowLineRange {
                            hot_path_metrics.note_keystroke_to_paint_admitted(clock.now().0);
                        }
                        scheduler.invalidate(damage.event);
                        scheduler.mark_hook(damage.hook, &clock);
                    }
                }

                if should_update_ime_cursor_area {
                    if let Some(session) = editor_runtime.tab_session_mut(focused, active_tab) {
                        update_ime_cursor_area_for_viewport(
                            &window,
                            &session.viewport,
                            viewport_rect,
                        );
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if state != ElementState::Pressed {
                    return;
                }
                if button != winit::event::MouseButton::Left {
                    return;
                }
                let (x, y) = match last_cursor_pos {
                    Some(pos) => pos,
                    None => return,
                };
                if !palette.is_open()
                    && overlay.is_none()
                    && activity_center.dismiss_notification_at(&frame, window.scale_factor(), x, y)
                {
                    enqueue_damage_hint(&mut scheduler, ShellDamageHint::FullWindow);
                    return;
                }
                if !palette.is_open()
                    && overlay.is_none()
                    && handle_status_bar_click(
                        &mut frame,
                        &mut overlay,
                        &mut command_runtime,
                        &editor_runtime,
                        &enablement_runtime,
                        &workspace_lifecycle,
                        &recent_work,
                        &activity_center,
                        title_context_bar.record(),
                        window.scale_factor(),
                        x,
                        y,
                    )
                {
                    enqueue_damage_hint(&mut scheduler, ShellDamageHint::FullWindow);
                    return;
                }
                if !palette.is_open()
                    && overlay.is_none()
                    && handle_activity_rail_click(
                        &mut frame,
                        &mut command_runtime,
                        &activity_center,
                        window.scale_factor(),
                        x,
                        y,
                    )
                {
                    enqueue_damage_hint(&mut scheduler, ShellDamageHint::FullWindow);
                    return;
                }
                if !palette.is_open() && overlay.is_none() {
                    if let Some(row_index) =
                        explorer_row_index_at(&frame, window.scale_factor(), x, y)
                    {
                        frame.focus_zone(ShellZoneId::LeftSidebar);
                        let changed = editor_runtime.explorer.toggle_row(row_index)
                            || editor_runtime.explorer.select_row(row_index);
                        if changed {
                            enqueue_damage_hint(
                                &mut scheduler,
                                left_sidebar_damage_hint(&frame, window.scale_factor()),
                            );
                        }
                        return;
                    }
                }
                if !palette.is_open() && overlay.is_none() {
                    let runtime = StartCenterRuntimeInputs {
                        client_scope: "desktop_product",
                        workspace_trust_state: enablement_runtime.workspace_trust_state.as_str(),
                        execution_context_available: enablement_runtime.execution_context_available,
                        provider_linked: enablement_runtime.provider_linked,
                        credential_available: enablement_runtime.credential_available,
                        policy_disabled: enablement_runtime.policy_disabled,
                        policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
                        labs_enabled: enablement_runtime.labs_enabled,
                    };
                    let rows = start_center_action_rows(registry, runtime);
                    if let Ok(token_registry) = seeded_token_registry(appearance.theme_class()) {
                        if let Ok(style) = ShellRenderStyle::load(
                            token_registry,
                            appearance.density_class(),
                            window.scale_factor(),
                        ) {
                            if let Some((group, row_index)) = start_center_action_at_point(
                                &frame,
                                window.scale_factor(),
                                &style,
                                &mut text_runtime,
                                rows.len(),
                                x,
                                y,
                            ) {
                                start_center.select_index(row_index, rows.len());
                                frame.focus_zone(ShellZoneId::MainWorkspace);
                                frame.focus_editor_group(group);
                                if let Some(row) = rows.get(row_index) {
                                    open_start_center_entry_flow_sheet(
                                        &mut frame,
                                        &mut overlay,
                                        row,
                                    );
                                    enqueue_damage_hint(
                                        &mut scheduler,
                                        ShellDamageHint::FullWindow,
                                    );
                                }
                                return;
                            }
                        }
                    }
                }
                if frame.focused_zone() != ShellZoneId::MainWorkspace
                    || palette.is_open()
                    || overlay.is_some()
                {
                    return;
                }

                let focused = frame.focused_editor_group();
                let has_tabs = frame
                    .editor_group_layouts()
                    .into_iter()
                    .find(|g| g.group_id == focused)
                    .is_some_and(|g| g.tab_count > 0);
                if !has_tabs {
                    return;
                }

                let Some(active_tab) = frame.active_tab_id(focused) else {
                    return;
                };
                let Some(viewport_rect) = damage_geometry
                    .focused_editor_viewport
                    .map(|rect| PixelRect::new(rect.x, rect.y, rect.width, rect.height))
                else {
                    return;
                };

                if !editor_runtime.has_tab_session(focused, active_tab) {
                    editor_runtime.open_placeholder(focused, active_tab);
                }
                let point = {
                    let Some(session) = editor_runtime.tab_session_mut(focused, active_tab) else {
                        return;
                    };
                    hit_test_viewport_point(&session.viewport, viewport_rect, x, y)
                };

                let Some(point) = point else {
                    return;
                };

                {
                    let Some(session) = editor_runtime.tab_session_mut(focused, active_tab) else {
                        return;
                    };
                    if held_modifiers.alt {
                        let previous_primary = session.viewport.caret();
                        session.viewport.add_secondary_caret(previous_primary);
                        session.viewport.set_caret(point);
                        session.viewport.clear_selection();
                    } else {
                        session.viewport.set_caret(point);
                        session.viewport.clear_selection();
                        session.viewport.clear_secondary_carets();
                    }
                    session.text_input.force_clear_composition();
                }

                if let Some(damage) = editor_runtime.apply_action(
                    focused,
                    active_tab,
                    &EditorAction::ClearComposition,
                    viewport_rect,
                ) {
                    scheduler.invalidate(damage.event);
                    scheduler.mark_hook(damage.hook, &clock);
                }

                if let Some(damage) = editor_runtime.apply_action(
                    focused,
                    active_tab,
                    &EditorAction::ChangeSelection {
                        delta: SelectionDelta::Cleared,
                    },
                    viewport_rect,
                ) {
                    scheduler.invalidate(damage.event);
                    scheduler.mark_hook(damage.hook, &clock);
                }

                if let Some(session) = editor_runtime.tab_session_mut(focused, active_tab) {
                    update_ime_cursor_area_for_viewport(&window, &session.viewport, viewport_rect);
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                let before_modifiers = held_modifiers;
                held_modifiers.update_from_key_event(&event);
                let modifiers_changed = before_modifiers != held_modifiers;
                let hint = handle_key_event(
                    &window,
                    registry,
                    &mut frame,
                    &mut editor_runtime,
                    &mut session_restore_store,
                    &damage_geometry,
                    &mut palette,
                    &mut palette_focus_return,
                    &mut start_center,
                    &mut overlay,
                    &mut command_runtime,
                    &mut clone_jobs,
                    &mut keybinding_runtime,
                    &mut enablement_runtime,
                    &mut workspace_lifecycle,
                    &mut recent_work,
                    &mut activity_center,
                    &mut clipboard,
                    &mut appearance,
                    &mut hot_path_metrics,
                    &clock,
                    &held_modifiers,
                    &event,
                );
                if hint != ShellDamageHint::None {
                    enqueue_damage_hint(&mut scheduler, hint);
                } else if frame.focused_zone() == ShellZoneId::MainWorkspace
                    && !palette.is_open()
                    && overlay.is_none()
                {
                    if let PhysicalKey::Code(code) = event.physical_key {
                        let focused = frame.focused_editor_group();
                        let has_tabs = frame
                            .editor_group_layouts()
                            .into_iter()
                            .find(|g| g.group_id == focused)
                            .is_some_and(|g| g.tab_count > 0);

                        if has_tabs {
                            let Some(active_tab) = frame.active_tab_id(focused) else {
                                return;
                            };
                            let Some(viewport_rect) =
                                damage_geometry.focused_editor_viewport.map(|rect| {
                                    PixelRect::new(rect.x, rect.y, rect.width, rect.height)
                                })
                            else {
                                return;
                            };

                            if !editor_runtime.has_tab_session(focused, active_tab) {
                                editor_runtime.open_placeholder(focused, active_tab);
                            }

                            let modifiers = aureline_input::text_input::TextInputModifiers {
                                ctrl: held_modifiers.ctrl,
                                alt: held_modifiers.alt,
                                shift: held_modifiers.shift,
                                logo: held_modifiers.logo,
                            };
                            let is_dead_key =
                                matches!(event.logical_key, winit::keyboard::Key::Dead(_));
                            let key_event = aureline_input::text_input::TextKeyEvent {
                                code: text_input_key_code(code),
                                text: event.text.as_deref().map(|text| text.to_string()),
                                is_repeat: event.repeat,
                                is_dead_key,
                                modifiers,
                            };

                            let normalized = {
                                let Some(session) =
                                    editor_runtime.tab_session_mut(focused, active_tab)
                                else {
                                    return;
                                };
                                session.text_input.handle_key_event(&key_event)
                            };

                            if let Some(normalized) = normalized {
                                let action = editor_action_from_text_input(normalized);
                                if let Some(damage) = editor_runtime.apply_action(
                                    focused,
                                    active_tab,
                                    &action,
                                    viewport_rect,
                                ) {
                                    if damage.hook == Hook::ReflowLineRange {
                                        hot_path_metrics
                                            .note_keystroke_to_paint_admitted(clock.now().0);
                                    }
                                    scheduler.invalidate(damage.event);
                                    scheduler.mark_hook(damage.hook, &clock);
                                }
                                if let Some(session) =
                                    editor_runtime.tab_session_mut(focused, active_tab)
                                {
                                    update_ime_cursor_area_for_viewport(
                                        &window,
                                        &session.viewport,
                                        viewport_rect,
                                    );
                                }
                            }
                        }
                    }
                }
                if palette.is_open() && modifiers_changed {
                    if let Some(rect) = damage_geometry.command_palette_panel {
                        enqueue_damage_hint(
                            &mut scheduler,
                            ShellDamageHint::Rect {
                                layer: CompositionLayerId::FloatingSurface,
                                class: DamageClassId::SelectionOverlayOnly,
                                rect,
                            },
                        );
                    } else {
                        enqueue_damage_hint(&mut scheduler, ShellDamageHint::FullWindow);
                    }
                }
            }
            WindowEvent::RedrawRequested => {
                let pending_frame = scheduler.begin_frame().or_else(|| {
                    scheduler.invalidate(DamageEvent::new(
                        CompositionLayerId::WindowChromeBase,
                        DamageClassId::WindowExposedRegionRefresh,
                    ));
                    scheduler.invalidate(DamageEvent::new(
                        CompositionLayerId::TextAndDecoration,
                        DamageClassId::WindowExposedRegionRefresh,
                    ));
                    scheduler.begin_frame()
                });
                if pending_frame.is_none() {
                    return;
                }
                let pending_frame = pending_frame.expect("pending frame must exist");
                if let Err(err) = draw(
                    &window,
                    &mut render_backend,
                    &mut text_runtime,
                    &mut editor_runtime,
                    registry,
                    &pending_frame.events,
                    &frame,
                    &palette,
                    &start_center,
                    &docs_help_boundary_card,
                    overlay.as_ref(),
                    &command_runtime,
                    &keybinding_runtime,
                    &enablement_runtime,
                    &workspace_lifecycle,
                    &recent_work,
                    &activity_center,
                    title_context_bar.record(),
                    &appearance,
                    &held_modifiers,
                    &mut damage_geometry,
                    screenshot_path.as_deref(),
                ) {
                    eprintln!("aureline_shell: draw failed: {err}");
                    let _ = hot_path_metrics.write_if_configured();
                    elwt.exit();
                    return;
                }
                maybe_capture_shell_accessibility_tree(
                    capture_a11y_tree,
                    &mut last_a11y_fingerprint,
                    &mut scheduler,
                    &clock,
                    registry,
                    &keybinding_runtime.shortcuts_by_command_id,
                    &frame,
                    &palette,
                    &start_center,
                    &docs_help_boundary_card,
                    &enablement_runtime,
                );
                scheduler.note_frame_submitted(&clock);
                let submit_tick = clock.now().0;
                hot_path_metrics.note_frame_submitted(submit_tick);
                hot_path_metrics.mark_first_shell_frame_submitted(submit_tick);
                if !startup_trace.first_frame_emitted() {
                    startup_trace.mark(StartupMilestone::FirstShellFrameSubmitted);
                    let _ = startup_trace.write_if_configured();
                    if startup_trace.config().exit_after_first_frame {
                        let _ = hot_path_metrics.write_if_configured();
                        elwt.exit();
                    }
                }
            }
            _ => {}
        },
        _ => {}
    })?;
    Ok(())
}

fn window_title(title_context: &TitleContextBarStateRecord) -> String {
    let identity = build_info::build_identity();
    let core = title_context
        .native_window_title_label()
        .unwrap_or("Start Center");
    format!("Aureline — {core} ({})", identity.commit_short)
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

#[derive(Debug)]
struct CloneJobRuntimeState {
    startup_probe: Result<GitProbe, CloneError>,
    active: Option<CloneJobHandle>,
    next_seq: usize,
}

#[derive(Debug)]
struct CloneJobHandle {
    operation_id: String,
    request: CloneRequest,
    session: CommandInvocationSession,
    receiver: Receiver<CloneWorkerMessage>,
}

#[derive(Debug, Clone)]
enum CloneWorkerMessage {
    Progress(CloneProgressEvent),
    Completed(Result<(), CloneError>),
}

impl CloneJobRuntimeState {
    fn new(startup_probe: Result<GitProbe, CloneError>) -> Self {
        Self {
            startup_probe,
            active: None,
            next_seq: 1,
        }
    }

    fn has_active(&self) -> bool {
        self.active.is_some()
    }

    fn next_operation_id(&mut self, request: &CloneRequest) -> String {
        let seq = self.next_seq;
        self.next_seq = self.next_seq.saturating_add(1);
        format!(
            "clone-{:016x}-{:02}",
            fnv1a_64(&format!(
                "{}\n{}",
                request.remote_url,
                request.destination_path.display()
            )),
            seq
        )
    }

    fn start(
        &mut self,
        request: CloneRequest,
        session: CommandInvocationSession,
    ) -> Result<String, CloneError> {
        if let Err(err) = &self.startup_probe {
            return Err(err.clone());
        }
        if self.active.is_some() {
            return Err(CloneError::new(
                CloneErrorClass::InvalidInput,
                "another clone is already running",
            ));
        }

        request.validate()?;
        let operation_id = self.next_operation_id(&request);
        let worker_request = request.clone();
        let (sender, receiver) = mpsc::channel();
        thread::spawn(move || {
            let backend = SystemGitCloneBackend::default();
            let result = backend.clone_repository(&worker_request, &mut |event| {
                let _ = sender.send(CloneWorkerMessage::Progress(event));
            });
            let _ = sender.send(CloneWorkerMessage::Completed(result));
        });

        self.active = Some(CloneJobHandle {
            operation_id: operation_id.clone(),
            request,
            session,
            receiver,
        });
        Ok(operation_id)
    }
}

struct ClipboardState {
    enabled: bool,
    clipboard: Option<Clipboard>,
}

impl ClipboardState {
    fn new(enabled: bool) -> Self {
        Self {
            enabled,
            clipboard: enabled.then(|| Clipboard::new().ok()).flatten(),
        }
    }

    fn set_text(&mut self, text: &str) -> Result<(), String> {
        if !self.enabled {
            return Err("clipboard disabled".to_string());
        }
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

    fn get_text(&mut self) -> Result<String, String> {
        if !self.enabled {
            return Err("clipboard disabled".to_string());
        }
        if self.clipboard.is_none() {
            self.clipboard = Clipboard::new().ok();
        }
        let Some(clipboard) = self.clipboard.as_mut() else {
            return Err("clipboard unavailable".to_string());
        };
        clipboard.get_text().map_err(|err| err.to_string())
    }
}

struct ShellTextRuntime {
    font_system: FontSystem,
    shaper: TextShaper,
    atlas: GlyphAtlas,
    ui_fallback: FontFallbackConfig,
    ui_features: FeatureSet,
}

impl ShellTextRuntime {
    fn new() -> Self {
        Self {
            font_system: FontSystem::with_system_fonts(),
            shaper: TextShaper::new(),
            atlas: GlyphAtlas::default(),
            ui_fallback: FontFallbackConfig::ui_sans(),
            ui_features: FeatureSet::ui_default(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReadOnlyState {
    Writable,
    Filesystem,
    Constrained,
}

impl ReadOnlyState {
    const fn token(self) -> Option<&'static str> {
        match self {
            Self::Writable => None,
            Self::Filesystem | Self::Constrained => Some("Read-only"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GeneratedState {
    Authored,
    #[allow(dead_code)]
    Generated,
}

impl GeneratedState {
    const fn token(self) -> Option<&'static str> {
        match self {
            Self::Authored => None,
            Self::Generated => Some("Generated"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ManagedState {
    Unmanaged,
    #[allow(dead_code)]
    Managed,
}

impl ManagedState {
    const fn token(self) -> Option<&'static str> {
        match self {
            Self::Unmanaged => None,
            Self::Managed => Some("Managed"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProjectionState {
    Direct,
    #[allow(dead_code)]
    Projection,
}

impl ProjectionState {
    const fn token(self) -> Option<&'static str> {
        match self {
            Self::Direct => None,
            Self::Projection => Some("Projection"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SnapshotFacingState {
    Live,
    #[allow(dead_code)]
    Snapshot,
}

impl SnapshotFacingState {
    const fn token(self) -> Option<&'static str> {
        match self {
            Self::Live => None,
            Self::Snapshot => Some("Snapshot"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LargeFileSurfaceState {
    Constrained,
    Override,
}

impl LargeFileSurfaceState {
    const fn token(self) -> &'static str {
        match self {
            Self::Constrained => "Large-file mode",
            Self::Override => "Large-file override",
        }
    }
}

struct BufferAuthority {
    label: String,
    file_path: Option<PathBuf>,
    vfs_identity: Option<VfsIdentityRecord>,
    save_target_token: Option<SaveTargetToken>,
    source_fidelity: Option<SourceFidelityRecord>,
    read_only: ReadOnlyState,
    generated: GeneratedState,
    managed: ManagedState,
    projection: ProjectionState,
    snapshot_facing: SnapshotFacingState,
    buffer: Buffer,
    saved_revision: RevisionId,
    find_replace: FindReplaceState,
    last_recorded_mutation_transaction_id: Option<TransactionId>,
    last_recorded_mutation_id: Option<String>,
    last_recorded_crash_journal_revision: Option<RevisionId>,
    large_file_doc: Option<LargeFileDocument>,
    large_file_override: Option<LargeFileOverrideInfo>,
}

impl BufferAuthority {
    fn revision_id(&self) -> RevisionId {
        self.buffer.revision_id()
    }

    fn is_dirty(&self) -> bool {
        self.buffer.revision_id() != self.saved_revision
    }

    fn mark_saved(&mut self) {
        self.saved_revision = self.buffer.revision_id();
    }
}

fn replay_recovered_bytes_into_authority(
    authority: &mut BufferAuthority,
    bytes: &[u8],
) -> Result<(), String> {
    authority.large_file_doc = None;
    authority.large_file_override = None;
    authority.find_replace = FindReplaceState::new();
    authority.last_recorded_mutation_transaction_id = None;
    authority.last_recorded_mutation_id = None;
    authority.last_recorded_crash_journal_revision = None;

    match std::str::from_utf8(bytes) {
        Ok(text) => {
            let len = authority.buffer.len();
            let spec =
                TransactionSpec::new(UndoClass::DecodeRecoveryChange, "crash_journal_restore")
                    .with_label("Recovered from crash journal");
            let mut tx = authority
                .buffer
                .begin(spec)
                .map_err(|err| format!("dirty-buffer replay failed — {err}"))?;
            tx.replace(0..len, text)
                .map_err(|err| format!("dirty-buffer replay failed — {err}"))?;
            tx.commit()
                .map_err(|err| format!("dirty-buffer replay failed — {err}"))?;
        }
        Err(_) => {
            authority.buffer = Buffer::from_bytes(bytes);
            authority.saved_revision = RevisionId(u64::MAX);
        }
    }

    Ok(())
}

#[derive(Debug, Clone)]
struct TabRenderInfo {
    label: String,
    dirty: bool,
    read_only: ReadOnlyState,
    generated: GeneratedState,
    managed: ManagedState,
    projection: ProjectionState,
    snapshot_facing: SnapshotFacingState,
    large_file_state: Option<LargeFileSurfaceState>,
}

struct BufferAuthorityStore {
    next_view_id: u64,
    by_canonical_path: HashMap<PathBuf, Rc<RefCell<BufferAuthority>>>,
    view_state_by_canonical_path: HashMap<PathBuf, EditorViewportSnapshot>,
}

impl BufferAuthorityStore {
    fn new() -> Self {
        Self {
            next_view_id: 1,
            by_canonical_path: HashMap::new(),
            view_state_by_canonical_path: HashMap::new(),
        }
    }

    fn mint_view_id(&mut self) -> u64 {
        let id = self.next_view_id;
        self.next_view_id = self.next_view_id.saturating_add(1);
        id
    }

    fn open_file_authority(&mut self, path: &Path) -> Result<Rc<RefCell<BufferAuthority>>, String> {
        let canonical = canonical_path_key(path);
        if let Some(existing) = self.by_canonical_path.get(&canonical) {
            return Ok(existing.clone());
        }

        let presentation_uri = VfsUri::file_url_for_path_lossy(path)
            .or_else(|| VfsUri::file_url_for_path(&canonical))
            .ok_or_else(|| format!("vfs uri build failed for {path:?}"))?;
        let local_root = LocalFilesystemRoot::host_root("ws-shell_proto", "root-local");
        let policy = shell_large_file_policy();
        let viewer_config = shell_large_file_viewer_config();
        let outcome = open_document(
            &local_root,
            &presentation_uri,
            &policy,
            viewer_config,
            DocumentOpenDisposition::Auto,
        )
        .map_err(|err| err.to_string())?;

        let mut counters = HookCounters::default();
        let save_target_token = open_save_target(
            &local_root,
            &presentation_uri,
            mono_timestamp_now(),
            &mut counters,
        )
        .map_err(|err| err.to_string())?;
        let label = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("untitled")
            .to_string();

        let authority = match outcome {
            DocumentOpenOutcome::Normal(doc) => {
                let raw_bytes = doc.snapshot.as_bytes().to_vec();
                let open_outcome = detect_and_decode_for_buffer(
                    &raw_bytes,
                    &save_target_token.permission_snapshot,
                );

                let (buffer, saved_revision, read_only) = match open_outcome.buffer_utf8_bytes {
                    Some(decoded) => {
                        let buffer = Buffer::from_bytes(&decoded);
                        let saved_revision = buffer.revision_id();
                        (buffer, saved_revision, read_only_state_for_path(&canonical))
                    }
                    None => {
                        let buffer = Buffer::from_str(
                            "<decode recovery: bytes could not be decoded as supported text>",
                        );
                        let saved_revision = buffer.revision_id();
                        (buffer, saved_revision, ReadOnlyState::Constrained)
                    }
                };
                Rc::new(RefCell::new(BufferAuthority {
                    label,
                    file_path: Some(canonical.clone()),
                    vfs_identity: Some(doc.identity),
                    save_target_token: Some(save_target_token.clone()),
                    source_fidelity: Some(open_outcome.record),
                    read_only,
                    generated: GeneratedState::Authored,
                    managed: ManagedState::Unmanaged,
                    projection: ProjectionState::Direct,
                    snapshot_facing: SnapshotFacingState::Live,
                    buffer,
                    saved_revision,
                    find_replace: FindReplaceState::new(),
                    last_recorded_mutation_transaction_id: None,
                    last_recorded_mutation_id: None,
                    last_recorded_crash_journal_revision: None,
                    large_file_doc: None,
                    large_file_override: doc.large_file_override,
                }))
            }
            DocumentOpenOutcome::LargeFile(mut doc) => {
                let notice = doc.notice();
                let preview = doc
                    .viewer
                    .read_prefix_utf8(LARGE_FILE_PREVIEW_MAX_BYTES)
                    .map_err(|err| err.to_string())?;
                let buffer =
                    Buffer::from_str(&large_file_presentation_buffer(&doc, notice, preview));
                let saved_revision = buffer.revision_id();
                Rc::new(RefCell::new(BufferAuthority {
                    label,
                    file_path: Some(canonical.clone()),
                    vfs_identity: Some(doc.identity.clone()),
                    save_target_token: Some(save_target_token.clone()),
                    source_fidelity: None,
                    read_only: ReadOnlyState::Constrained,
                    generated: GeneratedState::Authored,
                    managed: ManagedState::Unmanaged,
                    projection: ProjectionState::Direct,
                    snapshot_facing: SnapshotFacingState::Live,
                    buffer,
                    saved_revision,
                    find_replace: FindReplaceState::new(),
                    last_recorded_mutation_transaction_id: None,
                    last_recorded_mutation_id: None,
                    last_recorded_crash_journal_revision: None,
                    large_file_doc: Some(doc),
                    large_file_override: None,
                }))
            }
        };
        self.by_canonical_path.insert(canonical, authority.clone());
        Ok(authority)
    }

    fn placeholder_authority(
        &mut self,
        label: impl Into<String>,
        text: &str,
    ) -> Rc<RefCell<BufferAuthority>> {
        let label = label.into();
        let vfs_identity = {
            let mut root = VirtualDocumentRoot::new("ws-shell_proto", "root-virtual");
            let document_id = format!("placeholder/{}", self.next_view_id);
            let uri = root.add_document(VirtualDocumentSpec {
                document_id,
                display_label: label.clone(),
                kind: VirtualDocumentKind::Virtual,
                content: text.as_bytes().to_vec(),
            });
            uri.ok().and_then(|uri| root.identity_record(&uri).ok())
        };
        let buffer = Buffer::from_str(text);
        let saved_revision = buffer.revision_id();
        Rc::new(RefCell::new(BufferAuthority {
            label,
            file_path: None,
            vfs_identity,
            save_target_token: None,
            source_fidelity: None,
            read_only: ReadOnlyState::Writable,
            generated: GeneratedState::Authored,
            managed: ManagedState::Unmanaged,
            projection: ProjectionState::Direct,
            snapshot_facing: SnapshotFacingState::Live,
            buffer,
            saved_revision,
            find_replace: FindReplaceState::new(),
            last_recorded_mutation_transaction_id: None,
            last_recorded_mutation_id: None,
            last_recorded_crash_journal_revision: None,
            large_file_doc: None,
            large_file_override: None,
        }))
    }

    fn record_view_state(&mut self, canonical_path: &Path, snapshot: EditorViewportSnapshot) {
        self.view_state_by_canonical_path
            .insert(canonical_path.to_path_buf(), snapshot);
    }

    fn view_state_for_path(&self, canonical_path: &Path) -> Option<&EditorViewportSnapshot> {
        self.view_state_by_canonical_path.get(canonical_path)
    }
}

struct EditorTabSession {
    view_id: u64,
    authority: Rc<RefCell<BufferAuthority>>,
    last_seen_revision: RevisionId,
    snapshot: Snapshot,
    document_lines: Vec<String>,
    line_graphemes: Vec<usize>,
    text_input: aureline_input::text_input::TextInputSession,
    viewport: EditorViewport,
    compositor: ViewportCompositor,
    needs_text_repaint: bool,
}

impl EditorTabSession {
    fn new(view_id: u64, authority: Rc<RefCell<BufferAuthority>>) -> Self {
        let (snapshot, revision) = {
            let mut auth = authority.borrow_mut();
            let snapshot = auth.buffer.snapshot();
            let revision = auth.buffer.revision_id();
            (snapshot, revision)
        };
        let mut session = Self {
            view_id,
            authority,
            last_seen_revision: revision,
            snapshot,
            document_lines: Vec::new(),
            line_graphemes: Vec::new(),
            text_input: aureline_input::text_input::TextInputSession::new(),
            viewport: EditorViewport::new(),
            compositor: ViewportCompositor::default(),
            needs_text_repaint: true,
        };
        session.refresh_document_cache();
        session
    }

    fn render_info(&self) -> TabRenderInfo {
        let auth = self.authority.borrow();
        let large_file_state = if auth.large_file_doc.is_some() {
            Some(LargeFileSurfaceState::Constrained)
        } else if auth.large_file_override.is_some() {
            Some(LargeFileSurfaceState::Override)
        } else {
            None
        };
        TabRenderInfo {
            label: auth.label.clone(),
            dirty: auth.is_dirty(),
            read_only: auth.read_only,
            generated: auth.generated,
            managed: auth.managed,
            projection: auth.projection,
            snapshot_facing: auth.snapshot_facing,
            large_file_state,
        }
    }

    fn ensure_fresh_snapshot(&mut self) {
        let revision = self.authority.borrow().revision_id();
        if revision == self.last_seen_revision {
            return;
        }
        let snapshot = self.authority.borrow_mut().buffer.snapshot();
        self.snapshot = snapshot;
        self.last_seen_revision = revision;
        self.refresh_document_cache();
        self.viewport.clamp_to_document(&self.line_graphemes);
        self.needs_text_repaint = true;
    }

    fn refresh_snapshot_and_cache(&mut self) {
        let (snapshot, revision) = {
            let mut auth = self.authority.borrow_mut();
            let snapshot = auth.buffer.snapshot();
            let revision = auth.buffer.revision_id();
            (snapshot, revision)
        };
        self.snapshot = snapshot;
        self.last_seen_revision = revision;
        self.refresh_document_cache();
        self.viewport.clamp_to_document(&self.line_graphemes);
    }

    fn apply_viewport_snapshot(&mut self, snapshot: &EditorViewportSnapshot) {
        self.viewport.set_caret(snapshot.caret);
        self.viewport
            .set_selection_anchor(snapshot.selection_anchor);
        self.viewport
            .set_ime_composition(snapshot.ime_composition.clone());

        self.viewport.clear_secondary_carets();
        for secondary in &snapshot.secondary_selections {
            self.viewport.add_secondary_caret(secondary.caret);
        }
        {
            let selections = self.viewport.selections_mut();
            for secondary in &snapshot.secondary_selections {
                let Some(anchor) = secondary.selection_anchor else {
                    continue;
                };
                if let Some(selection) = selections
                    .secondary_mut()
                    .iter_mut()
                    .find(|row| row.caret() == secondary.caret)
                {
                    selection.set_anchor(Some(anchor));
                }
            }
        }

        let max_scroll_line = self.max_scroll_line();
        let current_scroll = self.viewport.scroll_line();
        if snapshot.scroll_line != current_scroll {
            let delta = snapshot.scroll_line as i32 - current_scroll as i32;
            let _ = self.viewport.scroll_by_lines(delta, max_scroll_line);
        }
        self.viewport.clamp_to_document(&self.line_graphemes);
        self.needs_text_repaint = true;
    }

    fn refresh_document_cache(&mut self) {
        match self.snapshot.as_str() {
            Some(_) => {
                self.document_lines = (0..self.snapshot.line_count())
                    .map(|line| self.snapshot.line_str(line).unwrap_or("").to_string())
                    .collect();
            }
            None => {
                self.document_lines = vec!["<non-UTF8 buffer: preview unavailable>".to_string()];
            }
        }
        if self.document_lines.is_empty() {
            self.document_lines.push(String::new());
        }
        self.line_graphemes = match self.snapshot.as_str() {
            Some(_) => (0..self.snapshot.line_count())
                .map(|line| self.snapshot.grapheme_count_in_line(line).unwrap_or(0))
                .collect(),
            None => self
                .document_lines
                .iter()
                .map(|line| {
                    unicode_segmentation::UnicodeSegmentation::graphemes(line.as_str(), true)
                        .count()
                })
                .collect(),
        };
    }

    fn max_scroll_line(&self) -> usize {
        self.document_lines.len().saturating_sub(1)
    }

    fn apply_action(
        &mut self,
        action: &EditorAction,
        viewport_rect: PixelRect,
    ) -> Option<aureline_editor::ViewportDamage> {
        self.ensure_fresh_snapshot();
        let max_scroll_line = self.max_scroll_line();

        if self.authority.borrow().read_only == ReadOnlyState::Constrained
            && matches!(
                action,
                EditorAction::InsertText { .. }
                    | EditorAction::DeleteBackward
                    | EditorAction::DeleteForward
            )
        {
            return None;
        }

        match action {
            EditorAction::InsertText { text } => {
                let scope = if self.viewport.caret_count() > 1
                    && self.viewport.ime_composition().is_some()
                {
                    aureline_editor::TextEditScope::PrimaryOnly
                } else {
                    aureline_editor::TextEditScope::AllCarets
                };

                let outcome = {
                    let mut authority = self.authority.borrow_mut();
                    self.viewport.selections_mut().apply_insert_text(
                        &mut authority.buffer,
                        &self.snapshot,
                        text,
                        aureline_editor::undo::originator::USER_KEYSTROKE,
                        scope,
                    )
                };

                let Ok(Some(outcome)) = outcome else {
                    return None;
                };

                self.snapshot = outcome.snapshot;
                self.last_seen_revision = outcome.revision;
                self.refresh_document_cache();
                self.viewport.set_ime_composition(None);
                self.needs_text_repaint = true;
            }
            EditorAction::DeleteBackward => {
                let outcome = {
                    let mut authority = self.authority.borrow_mut();
                    self.viewport.selections_mut().apply_delete_backward(
                        &mut authority.buffer,
                        &self.snapshot,
                        aureline_editor::undo::originator::USER_KEYSTROKE,
                        aureline_editor::TextEditScope::AllCarets,
                    )
                };

                let Ok(Some(outcome)) = outcome else {
                    return None;
                };

                self.snapshot = outcome.snapshot;
                self.last_seen_revision = outcome.revision;
                self.refresh_document_cache();
                self.viewport.set_ime_composition(None);
                self.needs_text_repaint = true;
            }
            EditorAction::DeleteForward => {
                let outcome = {
                    let mut authority = self.authority.borrow_mut();
                    self.viewport.selections_mut().apply_delete_forward(
                        &mut authority.buffer,
                        &self.snapshot,
                        aureline_editor::undo::originator::USER_KEYSTROKE,
                        aureline_editor::TextEditScope::AllCarets,
                    )
                };

                let Ok(Some(outcome)) = outcome else {
                    return None;
                };

                self.snapshot = outcome.snapshot;
                self.last_seen_revision = outcome.revision;
                self.refresh_document_cache();
                self.viewport.set_ime_composition(None);
                self.needs_text_repaint = true;
            }
            EditorAction::MoveCaret {
                movement,
                extend_selection,
            } => match movement {
                CaretMove::WordLeft | CaretMove::WordRight => {
                    let before = self.viewport.caret();

                    if !*extend_selection {
                        self.viewport.clear_selection();
                    } else if self.viewport.selection_anchor().is_none() {
                        self.viewport.set_selection_anchor(Some(before));
                    }

                    let direction = match movement {
                        CaretMove::WordLeft => aureline_editor::text_nav::WordMotion::Left,
                        CaretMove::WordRight => aureline_editor::text_nav::WordMotion::Right,
                        _ => unreachable!("movement already narrowed"),
                    };

                    let Some(next) = aureline_editor::text_nav::move_point_by_word(
                        &self.snapshot,
                        before,
                        direction,
                    ) else {
                        return None;
                    };

                    if next == before {
                        return None;
                    }

                    self.viewport.set_caret(next);
                }
                _ => {
                    if !self
                        .viewport
                        .move_caret(*movement, &self.line_graphemes, *extend_selection)
                    {
                        return None;
                    }
                }
            },
            EditorAction::ChangeSelection { delta } => {
                self.viewport
                    .apply_selection_delta(*delta, &self.line_graphemes);
            }
            EditorAction::UpdateComposition { composition } => {
                self.viewport.set_ime_composition(Some(composition.clone()));
            }
            EditorAction::ClearComposition => {
                self.viewport.set_ime_composition(None);
            }
            EditorAction::ScrollLines { .. } | EditorAction::ScaleChange => {}
        }

        {
            let mut auth = self.authority.borrow_mut();
            let _ = auth
                .find_replace
                .sync_for_view(&self.snapshot, self.viewport.caret());
        }

        let damage = self
            .viewport
            .apply_action(action, viewport_rect, max_scroll_line)?;

        Some(damage)
    }
}

struct EditorGroupSession {
    tabs: HashMap<EditorTabId, EditorTabSession>,
}

impl EditorGroupSession {
    fn new() -> Self {
        Self {
            tabs: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum SaveTabAttempt {
    Saved(SaveResult),
    NoTarget,
    ReviewRequired {
        record: SaveReviewSheetRecord,
        outcome: aureline_vfs::SaveOutcome,
    },
}

struct EditorWorkspaceRuntimeState {
    text_runtime: EditorTextRuntime,
    explorer: ExplorerViewRuntime,
    terminal_pane: TerminalPaneRuntimeState,
    groups: HashMap<PaneId, EditorGroupSession>,
    buffers: BufferAuthorityStore,
    save_coordinator: StagedSaveCoordinator,
    mutation_journal: MutationJournalStore,
    local_history: LocalHistoryStore,
    crash_journal: aureline_recovery::crash_journal::CrashJournalStore,
    crash_journal_workspace_ref: String,
}

#[derive(Clone)]
struct CrashJournalSnapshotCandidate {
    view_id: u64,
    snapshot_bytes: Vec<u8>,
    authority: Rc<RefCell<BufferAuthority>>,
}

impl EditorWorkspaceRuntimeState {
    fn new() -> Self {
        Self::with_log_root(PathBuf::from(".logs"))
    }

    fn with_log_root(log_root: PathBuf) -> Self {
        let storage = HistoryStorageRoot::new(log_root.join("history"));
        let recovery_root = log_root.join("recovery");
        Self {
            text_runtime: EditorTextRuntime::with_system_fonts(),
            explorer: ExplorerViewRuntime::new(),
            terminal_pane: TerminalPaneRuntimeState::new(),
            groups: HashMap::new(),
            buffers: BufferAuthorityStore::new(),
            save_coordinator: StagedSaveCoordinator::new(),
            mutation_journal: MutationJournalStore::new(storage.clone()),
            local_history: LocalHistoryStore::new(storage),
            crash_journal: aureline_recovery::crash_journal::CrashJournalStore::new(recovery_root),
            crash_journal_workspace_ref: "workspace:none".to_string(),
        }
    }

    #[cfg(test)]
    fn with_buffer_store(buffers: BufferAuthorityStore) -> Self {
        let storage = HistoryStorageRoot::new(PathBuf::from(".logs").join("history"));
        let recovery_root = PathBuf::from(".logs").join("recovery");
        Self {
            text_runtime: EditorTextRuntime::with_system_fonts(),
            explorer: ExplorerViewRuntime::new(),
            terminal_pane: TerminalPaneRuntimeState::new(),
            groups: HashMap::new(),
            buffers,
            save_coordinator: StagedSaveCoordinator::new(),
            mutation_journal: MutationJournalStore::new(storage.clone()),
            local_history: LocalHistoryStore::new(storage),
            crash_journal: aureline_recovery::crash_journal::CrashJournalStore::new(recovery_root),
            crash_journal_workspace_ref: "workspace:none".to_string(),
        }
    }

    #[cfg(test)]
    fn take_buffer_store(&mut self) -> BufferAuthorityStore {
        std::mem::replace(&mut self.buffers, BufferAuthorityStore::new())
    }

    fn set_workspace_recovery_ref(&mut self, workspace_ref: impl Into<String>) {
        self.crash_journal_workspace_ref = workspace_ref.into();
    }

    fn ensure_group(&mut self, group: PaneId) -> &mut EditorGroupSession {
        self.groups
            .entry(group)
            .or_insert_with(EditorGroupSession::new)
    }

    fn ensure_tab_session(
        &mut self,
        group: PaneId,
        tab: EditorTabId,
        authority: Rc<RefCell<BufferAuthority>>,
    ) -> &mut EditorTabSession {
        let view_id = self.buffers.mint_view_id();
        let group_session = self.ensure_group(group);
        group_session
            .tabs
            .entry(tab)
            .or_insert_with(|| EditorTabSession::new(view_id, authority))
    }

    fn open_placeholder(&mut self, group: PaneId, tab: EditorTabId) {
        let authority = self.buffers.placeholder_authority(
            "Welcome",
            "Welcome to Aureline.\n\n\
             This is a prototype editor viewport.\n\
             - Scroll: mouse wheel\n\
             - Move caret: arrow keys\n\
             - Extend selection: Shift+Arrow\n\
             - Add caret: Alt+Click\n\
             - Undo/redo: Cmd/Ctrl+Z / Cmd/Ctrl+Shift+Z\n\
             - Type: regular keys\n",
        );
        self.ensure_tab_session(group, tab, authority);
    }

    fn open_restore_placeholder(
        &mut self,
        group: PaneId,
        tab: EditorTabId,
        label: impl Into<String>,
        detail: impl AsRef<str>,
    ) {
        let label = label.into();
        let text = format!("{}\n\n{}\n", label, detail.as_ref());
        let authority = self.buffers.placeholder_authority(label, &text);
        self.ensure_tab_session(group, tab, authority);
    }

    fn open_file(&mut self, group: PaneId, tab: EditorTabId, path: &Path) -> Result<(), String> {
        let authority = self.buffers.open_file_authority(path)?;
        let view_state = authority
            .borrow()
            .file_path
            .as_ref()
            .and_then(|canonical| self.buffers.view_state_for_path(canonical))
            .cloned();
        let session = self.ensure_tab_session(group, tab, authority);
        if let Some(view_state) = view_state {
            session.apply_viewport_snapshot(&view_state);
        }
        Ok(())
    }

    fn replace_tab_contents(
        &mut self,
        group: PaneId,
        tab: EditorTabId,
        text: &str,
        originator: impl Into<String>,
    ) -> Result<(), String> {
        let snapshot_candidate = {
            let Some(session) = self.tab_session_mut(group, tab) else {
                return Err("tab not found".to_string());
            };
            if session.authority.borrow().read_only != ReadOnlyState::Writable {
                return Err("tab is read-only".to_string());
            }

            let produced = {
                let mut authority = session.authority.borrow_mut();
                let len = authority.buffer.len();
                authority
                    .buffer
                    .replace(0..len, text, originator)
                    .map_err(|err| format!("headless edit failed — {err}"))?;
                authority.buffer.snapshot()
            };

            session.snapshot = produced;
            session.last_seen_revision = session.authority.borrow().revision_id();
            session.refresh_document_cache();
            session.viewport.clamp_to_document(&session.line_graphemes);
            session.needs_text_repaint = true;

            CrashJournalSnapshotCandidate {
                view_id: session.view_id,
                snapshot_bytes: session.snapshot.as_bytes().to_vec(),
                authority: session.authority.clone(),
            }
        };
        self.record_crash_journal_snapshot(snapshot_candidate);
        Ok(())
    }

    fn open_recovered_dirty_buffer(
        &mut self,
        group: PaneId,
        tab: EditorTabId,
        replay: &RestoreDirtyBufferReplay,
        file_path: Option<&Path>,
    ) -> Result<(), String> {
        let label = replay
            .presentation_hint
            .clone()
            .unwrap_or_else(|| "Recovered buffer".to_string());
        let authority = match file_path {
            Some(path) => match self.buffers.open_file_authority(path) {
                Ok(authority) => authority,
                Err(_) => self.buffers.placeholder_authority(label.clone(), ""),
            },
            None => self.buffers.placeholder_authority(label.clone(), ""),
        };

        {
            let mut auth = authority.borrow_mut();
            replay_recovered_bytes_into_authority(&mut auth, &replay.bytes)?;
        }

        let session = self.ensure_tab_session(group, tab, authority);
        session.refresh_snapshot_and_cache();
        session.needs_text_repaint = true;
        Ok(())
    }

    fn open_anyway(&mut self, group: PaneId, tab: EditorTabId) -> Result<(), String> {
        let Some(tab_session) = self
            .groups
            .get_mut(&group)
            .and_then(|g| g.tabs.get_mut(&tab))
        else {
            return Err("tab not found".to_string());
        };
        let canonical = tab_session
            .authority
            .borrow()
            .file_path
            .clone()
            .ok_or_else(|| "tab is not file-backed".to_string())?;
        if tab_session.authority.borrow().large_file_doc.is_none() {
            return Err("tab is not in large-file mode".to_string());
        }
        let uri = VfsUri::file_url_for_path(&canonical)
            .ok_or_else(|| format!("vfs uri build failed for {canonical:?}"))?;
        let local_root = LocalFilesystemRoot::host_root("ws-shell_proto", "root-local");
        let policy = shell_large_file_policy();
        let viewer_config = shell_large_file_viewer_config();
        let outcome = open_document(
            &local_root,
            &uri,
            &policy,
            viewer_config,
            DocumentOpenDisposition::ForceNormal,
        )
        .map_err(|err| err.to_string())?;
        let DocumentOpenOutcome::Normal(doc) = outcome else {
            return Err("expected normal document after override".to_string());
        };

        let mut counters = HookCounters::default();
        let save_target_token =
            open_save_target(&local_root, &uri, mono_timestamp_now(), &mut counters)
                .map_err(|err| err.to_string())?;

        {
            let mut auth = tab_session.authority.borrow_mut();
            let raw_bytes = doc.snapshot.as_bytes().to_vec();
            let open_outcome =
                detect_and_decode_for_buffer(&raw_bytes, &save_target_token.permission_snapshot);
            let (buffer, saved_revision, read_only) = match open_outcome.buffer_utf8_bytes {
                Some(decoded) => {
                    let buffer = Buffer::from_bytes(&decoded);
                    let saved_revision = buffer.revision_id();
                    (buffer, saved_revision, read_only_state_for_path(&canonical))
                }
                None => {
                    let buffer = Buffer::from_str(
                        "<decode recovery: bytes could not be decoded as supported text>",
                    );
                    let saved_revision = buffer.revision_id();
                    (buffer, saved_revision, ReadOnlyState::Constrained)
                }
            };
            auth.buffer = buffer;
            auth.saved_revision = saved_revision;
            auth.vfs_identity = Some(doc.identity);
            auth.save_target_token = Some(save_target_token);
            auth.source_fidelity = Some(open_outcome.record);
            auth.read_only = read_only;
            auth.large_file_doc = None;
            auth.large_file_override = doc.large_file_override;
            auth.find_replace = FindReplaceState::new();
        }

        tab_session.refresh_snapshot_and_cache();
        tab_session.needs_text_repaint = true;
        Ok(())
    }

    fn clone_tab_view(
        &mut self,
        source_group: PaneId,
        source_tab: EditorTabId,
        target_group: PaneId,
        target_tab: EditorTabId,
    ) -> bool {
        let Some((authority, view_state)) = self
            .groups
            .get(&source_group)
            .and_then(|group| group.tabs.get(&source_tab))
            .map(|tab| (tab.authority.clone(), tab.viewport.snapshot()))
        else {
            return false;
        };
        let session = self.ensure_tab_session(target_group, target_tab, authority);
        session.apply_viewport_snapshot(&view_state);
        true
    }

    fn save_tab(&mut self, group: PaneId, tab: EditorTabId) -> Result<SaveTabAttempt, String> {
        let authority_handle = {
            let Some(session) = self
                .groups
                .get_mut(&group)
                .and_then(|g| g.tabs.get_mut(&tab))
            else {
                return Err("tab not found".to_string());
            };
            session.ensure_fresh_snapshot();
            session.authority.clone()
        };

        let mut authority = authority_handle.borrow_mut();
        if authority.read_only != ReadOnlyState::Writable {
            return Err("tab is read-only".to_string());
        }
        let Some(path) = authority.file_path.clone() else {
            authority.mark_saved();
            return Ok(SaveTabAttempt::NoTarget);
        };

        let presentation_uri = VfsUri::file_url_for_path(&path)
            .ok_or_else(|| format!("vfs uri build failed for {path:?}"))?;
        let mut root = LocalFilesystemRoot::host_root("ws-shell_proto", "root-local");

        let token = match authority.save_target_token.clone() {
            Some(token) => token,
            None => {
                let mut counters = HookCounters::default();
                let token = open_save_target(
                    &root,
                    &presentation_uri,
                    mono_timestamp_now(),
                    &mut counters,
                )
                .map_err(|err| err.to_string())?;
                authority.save_target_token = Some(token.clone());
                token
            }
        };

        let snapshot = authority.buffer.snapshot();
        let source_fidelity = authority
            .source_fidelity
            .clone()
            .ok_or_else(|| "missing source-fidelity record for save".to_owned())?;

        let checkpoint_ref = if authority.is_dirty() {
            let captured_at = mono_timestamp_now();
            Some(self.record_save_checkpoint(
                &mut authority,
                &token,
                &source_fidelity,
                &snapshot,
                &captured_at,
            )?)
        } else {
            None
        };
        let request = StagedSaveRequest {
            token: token.clone(),
            new_content: snapshot.as_bytes().to_vec(),
            source_fidelity: source_fidelity.clone(),
            save_participant_group_id: None,
            checkpoint_ref: checkpoint_ref.clone(),
            committed_at: mono_timestamp_now(),
        };

        let mut participants: Vec<Box<dyn aureline_workspace::save::SaveParticipant>> = Vec::new();
        let result = self
            .save_coordinator
            .save(&mut root, request, participants.as_mut_slice());

        if result.committed() {
            authority.mark_saved();
            authority.save_target_token = Some(result.next_token.clone());
            Ok(SaveTabAttempt::Saved(result))
        } else {
            let outcome = result.manifest.outcome;
            if matches!(
                outcome,
                aureline_vfs::SaveOutcome::ExternalChangeDetected
                    | aureline_vfs::SaveOutcome::SaveConflict
                    | aureline_vfs::SaveOutcome::WrongTargetPrevented
                    | aureline_vfs::SaveOutcome::WatcherUncertainty
                    | aureline_vfs::SaveOutcome::ReviewRequiredBeforeSave
                    | aureline_vfs::SaveOutcome::ReviewRequiredBeforeRename
            ) {
                let record = materialize_save_review_sheet_record(
                    &root,
                    &token,
                    &source_fidelity,
                    result.packet_id.clone(),
                    outcome,
                    mono_timestamp_now(),
                    snapshot.as_bytes(),
                    false,
                );
                write_save_review_sheet_log(&record);
                return Ok(SaveTabAttempt::ReviewRequired { record, outcome });
            }

            let hint = match outcome {
                aureline_vfs::SaveOutcome::ExternalChangeDetected => {
                    Some("file changed on disk; compare or reload before overwriting")
                }
                aureline_vfs::SaveOutcome::SaveConflict => {
                    Some("save target revision changed; revalidate and retry")
                }
                aureline_vfs::SaveOutcome::WrongTargetPrevented => {
                    Some("save target drifted; reopen the file or use Save As")
                }
                aureline_vfs::SaveOutcome::WatcherUncertainty => {
                    Some("watcher state is uncertain; refresh and compare before saving")
                }
                _ => None,
            };

            let detail = result.manifest.failure_detail.clone().unwrap_or_default();
            let mut message = format!("save refused ({})", outcome.as_str());
            if let Some(hint) = hint {
                message.push_str(" — ");
                message.push_str(hint);
            }
            if !detail.is_empty() {
                message.push_str(" — ");
                message.push_str(&detail);
            }
            Err(message)
        }
    }

    fn close_group(&mut self, group: PaneId) {
        let Some(group_session) = self.groups.remove(&group) else {
            return;
        };
        for session in group_session.tabs.into_values() {
            let file_path = session.authority.borrow().file_path.clone();
            let Some(canonical) = file_path else {
                continue;
            };
            self.buffers
                .record_view_state(&canonical, session.viewport.snapshot());
        }
    }

    fn close_tab(&mut self, group: PaneId, tab: EditorTabId) {
        let session = self
            .groups
            .get_mut(&group)
            .and_then(|session| session.tabs.remove(&tab));
        let Some(session) = session else {
            return;
        };
        let file_path = session.authority.borrow().file_path.clone();
        let Some(canonical) = file_path else {
            return;
        };
        self.buffers
            .record_view_state(&canonical, session.viewport.snapshot());
    }

    fn has_tab_session(&self, group: PaneId, tab: EditorTabId) -> bool {
        self.groups
            .get(&group)
            .is_some_and(|session| session.tabs.contains_key(&tab))
    }

    fn tab_session_mut(
        &mut self,
        group: PaneId,
        tab: EditorTabId,
    ) -> Option<&mut EditorTabSession> {
        self.groups.get_mut(&group)?.tabs.get_mut(&tab)
    }

    fn tab_render_info(&self, group: PaneId, tab: EditorTabId) -> Option<TabRenderInfo> {
        let group = self.groups.get(&group)?;
        let tab = group.tabs.get(&tab)?;
        Some(tab.render_info())
    }

    fn active_source_fidelity(
        &self,
        group: PaneId,
        tab: Option<EditorTabId>,
    ) -> Option<SourceFidelityRecord> {
        let tab = tab?;
        let group = self.groups.get(&group)?;
        let tab = group.tabs.get(&tab)?;
        tab.authority.borrow().source_fidelity.clone()
    }

    fn apply_action(
        &mut self,
        group: PaneId,
        tab: EditorTabId,
        action: &EditorAction,
        viewport_rect: PixelRect,
    ) -> Option<aureline_editor::ViewportDamage> {
        let (damage, snapshot_candidate) = {
            let group_session = self.groups.get_mut(&group)?;
            let tab_session = group_session.tabs.get_mut(&tab)?;

            let before_revision = tab_session.last_seen_revision;
            let damage = tab_session.apply_action(action, viewport_rect);
            let revision_changed = tab_session.last_seen_revision != before_revision;

            let snapshot_candidate = revision_changed.then(|| CrashJournalSnapshotCandidate {
                view_id: tab_session.view_id,
                snapshot_bytes: tab_session.snapshot.as_bytes().to_vec(),
                authority: tab_session.authority.clone(),
            });

            (damage, snapshot_candidate)
        };

        if let Some(snapshot_candidate) = snapshot_candidate {
            self.record_crash_journal_snapshot(snapshot_candidate);
        }

        damage
    }

    fn record_crash_journal_snapshot(&mut self, snapshot: CrashJournalSnapshotCandidate) {
        let emitted_at = mono_timestamp_now();
        let bytes = snapshot.snapshot_bytes;

        let mut authority = snapshot.authority.borrow_mut();
        if !authority.is_dirty() {
            return;
        }

        let revision = authority.buffer.revision_id();
        if authority.last_recorded_crash_journal_revision == Some(revision) {
            return;
        }
        authority.last_recorded_crash_journal_revision = Some(revision);

        let (logical_document_id, object_ref, object_class) =
            if let Some(token) = authority.save_target_token.as_ref() {
                (
                    aureline_history::checkpoints::logical_document_id(&token.identity),
                    token
                        .identity
                        .logical_workspace_identity
                        .logical_uri
                        .to_string(),
                    aureline_recovery::crash_journal::ObjectClass::CanonicalFile,
                )
            } else if let Some(identity) = authority.vfs_identity.as_ref() {
                (
                    aureline_history::checkpoints::logical_document_id(identity),
                    identity.logical_workspace_identity.logical_uri.to_string(),
                    aureline_recovery::crash_journal::ObjectClass::VirtualBuffer,
                )
            } else {
                (
                    format!("ld:buffer:{}", snapshot.view_id),
                    format!("buffer:{}", snapshot.view_id),
                    aureline_recovery::crash_journal::ObjectClass::VirtualBuffer,
                )
            };

        let journal_id = format!("journal:{}", self.crash_journal_workspace_ref);
        let input = aureline_recovery::crash_journal::CrashJournalCaptureInput {
            journal_id,
            workspace_ref: self.crash_journal_workspace_ref.clone(),
            logical_document_id,
            object_ref,
            object_class,
            presentation_hint: Some(authority.label.clone()),
            emitted_at,
            bytes,
        };

        let _ = self.crash_journal.capture_minimal_full_snapshot(input);
    }

    fn compose_group(
        &mut self,
        group: PaneId,
        tab: EditorTabId,
        window_buffer: &mut [u32],
        window_width: u32,
        window_height: u32,
        viewport_rect: PixelRect,
        clip: Option<PixelRect>,
        paint_style: &ViewportPaintStyle,
    ) {
        let Some(group_session) = self.groups.get_mut(&group) else {
            return;
        };
        let Some(session) = group_session.tabs.get_mut(&tab) else {
            return;
        };
        session.ensure_fresh_snapshot();
        let size_changed = session.viewport.layout().viewport_width_px != viewport_rect.width
            || session.viewport.layout().viewport_height_px != viewport_rect.height;

        if session.needs_text_repaint || size_changed {
            session.compositor.repaint_text_layer(
                &mut session.viewport,
                &session.document_lines,
                &mut self.text_runtime,
                paint_style,
                (viewport_rect.width, viewport_rect.height),
            );
            session.needs_text_repaint = false;
        } else {
            session.compositor.scroll_text_layer(
                &mut session.viewport,
                &session.document_lines,
                &mut self.text_runtime,
                paint_style,
                (viewport_rect.width, viewport_rect.height),
            );
        }
        let auth = session.authority.borrow();
        let highlights = auth.find_replace.highlight_overlays();
        session.compositor.compose_into_window(
            window_buffer,
            window_width,
            window_height,
            viewport_rect,
            &session.viewport,
            highlights,
            paint_style,
            clip,
        );
    }

    fn record_save_checkpoint(
        &mut self,
        authority: &mut BufferAuthority,
        token: &SaveTargetToken,
        source_fidelity: &SourceFidelityRecord,
        snapshot: &Snapshot,
        captured_at: &str,
    ) -> Result<String, String> {
        let Some(journal_entry) = authority.buffer.peek_undo() else {
            return Err(
                "dirty buffer missing journal entry for local-history checkpoint".to_owned(),
            );
        };

        let entry_id = self.local_history.mint_entry_id();
        let filesystem_identity =
            aureline_history::checkpoints::filesystem_identity_record(&token.identity);
        let logical_document_id =
            aureline_history::checkpoints::logical_document_id(&token.identity);

        let mutation_id = if authority.last_recorded_mutation_transaction_id
            == Some(journal_entry.transaction_id())
        {
            authority
                .last_recorded_mutation_id
                .clone()
                .ok_or_else(|| "buffer mutation journal state missing mutation id".to_owned())?
        } else {
            let mutation_id = self.record_buffer_mutation_with_checkpoint(
                &filesystem_identity,
                &logical_document_id,
                journal_entry,
                &entry_id,
                captured_at,
            )?;
            authority.last_recorded_mutation_transaction_id = Some(journal_entry.transaction_id());
            authority.last_recorded_mutation_id = Some(mutation_id.clone());
            mutation_id
        };

        let staged_bytes = encode_for_save(source_fidelity, snapshot.as_bytes())
            .map_err(|detail| format!("local-history capture failed: {detail}"))?;
        let body_object_ref = self
            .local_history
            .write_body_object(&staged_bytes)
            .map_err(|err| err.to_string())?;

        let capture_descriptor = aureline_history::checkpoints::CaptureDescriptor {
            capture_mode: aureline_history::checkpoints::CaptureMode::ContentAddressedSnapshot,
            omission_reason: aureline_history::checkpoints::CaptureOmissionReasonClass::NotOmitted,
            body_available: true,
            body_object_refs: vec![body_object_ref],
            reference_digest: None,
            bytes_estimated: Some(staged_bytes.len() as u64),
            omission_note: None,
        };

        let mutation_journal_link = aureline_history::checkpoints::MutationJournalLink {
            linked_kind:
                aureline_history::checkpoints::MutationJournalLinkKind::MutationJournalEntry,
            linked_id: mutation_id.clone(),
            actor_class: Some(
                aureline_history::checkpoints::MutationJournalLinkActorClass::from(
                    self.actor_class_for_originator(journal_entry.originator()),
                ),
            ),
            source_class: Some(aureline_history::SourceClass::HumanLocal),
            reversal_class: Some(
                aureline_history::checkpoints::MutationJournalLinkReversalClass::from(
                    self.reversal_class_for_posture(journal_entry.compensation_posture()),
                ),
            ),
            redaction_class: Some(aureline_history::RedactionClass::CodeAdjacent),
        };

        let logical_document_identity = aureline_history::checkpoints::LogicalDocumentIdentity {
            logical_document_id,
            current_filesystem_identity: filesystem_identity,
            canonical_identity_drift: Some(
                aureline_history::checkpoints::CanonicalIdentityDrift::NoDrift,
            ),
            rename_move_history: Vec::new(),
        };

        let entry = aureline_history::LocalHistoryEntryRecord::new(
            entry_id.clone(),
            aureline_history::checkpoints::SnapshotClass::EditSaveCheckpoint,
            captured_at.to_owned(),
            logical_document_identity,
            capture_descriptor,
            mutation_journal_link,
            aureline_history::RetentionScopeClass::RetainedByPolicyWindow,
            Some(format!(
                "Edit/save checkpoint captured for {}",
                token.identity.presentation_path.display_label
            )),
        );

        self.local_history
            .write_entry(&entry)
            .map_err(|err| err.to_string())?;

        Ok(entry_id)
    }

    fn record_buffer_mutation_with_checkpoint(
        &mut self,
        filesystem_identity: &aureline_history::checkpoints::FilesystemIdentityRecord,
        logical_document_id: &str,
        journal_entry: aureline_buffer::JournalEntry<'_>,
        local_history_entry_id: &str,
        captured_at: &str,
    ) -> Result<String, String> {
        let mutation_id = self.mutation_journal.mint_mutation_id();
        let command_id = self.command_id_for_originator(journal_entry.originator());
        let actor_class = self.actor_class_for_originator(journal_entry.originator());
        let actor_ref = aureline_history::ActorRef {
            display_name: env::var("USER").unwrap_or_else(|_| "local user".to_string()),
            stable_id: None,
            role: Some("author".to_string()),
        };
        let scope_ref = aureline_history::ScopeRef {
            class: aureline_history::ScopeClass::Buffer,
            id: format!("buf:{logical_document_id}"),
        };
        let target_refs = vec![aureline_history::TargetRef {
            target_kind: aureline_history::TargetKind::Buffer,
            filesystem_identity: Some(filesystem_identity.clone()),
            logical_ref: Some(logical_document_id.to_owned()),
            affected_range: None,
        }];
        let reversal_class = self.reversal_class_for_posture(journal_entry.compensation_posture());
        let mut side_effect = aureline_history::SideEffectSummary::new(format!(
            "Committed {} ({})",
            journal_entry.class_id(),
            command_id
        ));
        side_effect.bytes_written =
            Some((journal_entry.inserted_bytes() + journal_entry.removed_bytes()) as u64);
        side_effect.files_touched = Some(0);

        let checkpoint_refs = vec![aureline_history::CheckpointRef {
            checkpoint_kind: aureline_history::CheckpointKind::LocalHistorySnapshot,
            checkpoint_id: local_history_entry_id.to_owned(),
            durability_class: Some(aureline_history::CheckpointDurabilityClass::Durable),
        }];

        let entry = aureline_history::MutationJournalEntryRecord::new(
            mutation_id.clone(),
            command_id,
            actor_class,
            aureline_history::SourceClass::HumanLocal,
            actor_ref,
            scope_ref,
            target_refs,
            captured_at.to_owned(),
            captured_at.to_owned(),
            journal_entry.class_id().to_owned(),
            reversal_class,
            aureline_history::RedactionClass::CodeAdjacent,
            aureline_history::DurableVsDisposable::DurableUserAuthored,
            side_effect,
            checkpoint_refs,
        );

        self.mutation_journal
            .write_entry(&entry)
            .map_err(|err| err.to_string())?;

        Ok(mutation_id)
    }

    fn command_id_for_originator(&self, originator: &str) -> String {
        match originator {
            aureline_editor::undo::originator::USER_KEYSTROKE => "editor.type".to_owned(),
            aureline_editor::undo::originator::PASTE => "editor.paste".to_owned(),
            other => other.strip_prefix("command:").unwrap_or(other).to_owned(),
        }
    }

    fn actor_class_for_originator(&self, originator: &str) -> aureline_history::ActorClass {
        match originator {
            aureline_editor::undo::originator::USER_KEYSTROKE => {
                aureline_history::ActorClass::UserKeystroke
            }
            aureline_editor::undo::originator::PASTE => aureline_history::ActorClass::UserCommand,
            other if other.starts_with("command:") => aureline_history::ActorClass::UserCommand,
            _ => aureline_history::ActorClass::UserCommand,
        }
    }

    fn reversal_class_for_posture(
        &self,
        posture: aureline_buffer::CompensationPosture,
    ) -> aureline_history::ReversalClass {
        match posture {
            aureline_buffer::CompensationPosture::Compensatable => {
                aureline_history::ReversalClass::ExactUndo
            }
            aureline_buffer::CompensationPosture::OnlyRevertible => {
                aureline_history::ReversalClass::CompensatingUndo
            }
        }
    }
}

fn canonical_path_key(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

fn read_only_state_for_path(path: &Path) -> ReadOnlyState {
    match std::fs::OpenOptions::new().write(true).open(path) {
        Ok(_) => ReadOnlyState::Writable,
        Err(_) => ReadOnlyState::Filesystem,
    }
}

const LARGE_FILE_PREVIEW_MAX_BYTES: u64 = 16 * 1024;

fn shell_large_file_policy() -> ClassificationPolicy {
    let mut policy = ClassificationPolicy::default();
    if let Ok(raw) = std::env::var("AURELINE_LARGE_FILE_THRESHOLD_BYTES") {
        if let Ok(value) = raw.parse::<u64>() {
            policy.large_file_size_threshold = value;
        }
    }
    policy
}

fn shell_large_file_viewer_config() -> LargeFileViewerConfig {
    LargeFileViewerConfig::default()
}

fn large_file_presentation_buffer(
    doc: &LargeFileDocument,
    notice: aureline_editor::LargeFileModeNotice,
    preview: Option<String>,
) -> String {
    let decision = doc.viewer.decision();
    let mut out = String::new();
    out.push_str(&notice.title);
    out.push('\n');
    out.push('\n');

    if let Some(trigger) = notice.trigger.as_deref() {
        out.push_str("Trigger: ");
        out.push_str(trigger);
        out.push('\n');
    }
    out.push_str("Reason: ");
    out.push_str(&notice.reason);
    out.push('\n');
    out.push_str("Bytes on disk: ");
    out.push_str(&decision.bytes_on_disk.to_string());
    out.push('\n');
    out.push('\n');

    out.push_str("Reduced capabilities:\n");
    for capability in notice.reduced_capabilities {
        out.push_str("- ");
        out.push_str(&capability);
        out.push('\n');
    }
    out.push('\n');

    out.push_str("Escalation:\n- ");
    out.push_str(&notice.escalation_label);
    out.push_str(": ");
    out.push_str(&notice.escalation_detail);
    out.push('\n');
    out.push_str("- Action: press Ctrl/Cmd+Shift+O to open in the normal buffer path.\n");
    out.push('\n');

    out.push_str(&format!(
        "Preview (first {} bytes, UTF-8 only):\n",
        LARGE_FILE_PREVIEW_MAX_BYTES
    ));
    match preview {
        Some(text) => out.push_str(&text),
        None => out.push_str("<non-UTF8 or binary content: preview unavailable>\n"),
    }
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

#[cfg(test)]
mod clipboard_tests {
    use super::ClipboardState;

    #[test]
    fn disabled_clipboard_refuses_set_text() {
        let mut clipboard = ClipboardState::new(false);
        let err = clipboard
            .set_text("hello")
            .expect_err("disabled clipboard should fail");
        assert_eq!(err, "clipboard disabled");
    }

    #[test]
    fn disabled_clipboard_refuses_get_text() {
        let mut clipboard = ClipboardState::new(false);
        let err = clipboard
            .get_text()
            .expect_err("disabled clipboard should fail");
        assert_eq!(err, "clipboard disabled");
    }
}

const TERMINAL_BUFFER_MAX_BYTES: usize = 128 * 1024;

#[derive(Debug, Clone)]
struct TerminalPaneRuntimeState {
    host: PtyHost,
    active_workspace_id: Option<String>,
    workspace_root: Option<PathBuf>,
    active_session_id: Option<PtySessionId>,
    output_buffers: HashMap<PtySessionId, String>,
    contexts_by_session: HashMap<PtySessionId, ExecutionContext>,
    host_boundary_wedges: HashMap<String, HostBoundaryCueWedge>,
}

impl TerminalPaneRuntimeState {
    fn new() -> Self {
        Self {
            host: PtyHost::new(),
            active_workspace_id: None,
            workspace_root: None,
            active_session_id: None,
            output_buffers: HashMap::new(),
            contexts_by_session: HashMap::new(),
            host_boundary_wedges: HashMap::new(),
        }
    }

    fn open_workspace(
        &mut self,
        workspace_id: String,
        workspace_root: PathBuf,
        trust_state: TrustState,
        observed_at: &str,
    ) {
        if self.active_workspace_id.as_deref() != Some(workspace_id.as_str()) {
            self.close_active_workspace(observed_at, Some("workspace_changed"));
        }
        self.active_workspace_id = Some(workspace_id);
        self.workspace_root = Some(workspace_root);
        let _ = self.ensure_session_for_active_workspace(trust_state, observed_at);
    }

    fn close_active_workspace(&mut self, observed_at: &str, reason_code: Option<&str>) {
        let Some(workspace_id) = self.active_workspace_id.clone() else {
            return;
        };
        let ids: Vec<PtySessionId> = self
            .host
            .sessions()
            .filter(|session| session.header().workspace_id == workspace_id)
            .map(|session| session.session_id().clone())
            .collect();
        for id in ids {
            let _ = self.host.close_session(&id, observed_at, reason_code);
            self.output_buffers.remove(&id);
            self.contexts_by_session.remove(&id);
        }
        if let Some(mut wedge) = self.host_boundary_wedges.remove(&workspace_id) {
            let _ = wedge.record_closed(observed_at, reason_code);
        }
        self.active_session_id = None;
        self.active_workspace_id = None;
        self.workspace_root = None;
    }

    fn ensure_session_for_active_workspace(
        &mut self,
        trust_state: TrustState,
        observed_at: &str,
    ) -> Option<PtySessionId> {
        let workspace_id = self.active_workspace_id.clone()?;
        if let Some(id) = self.active_session_id.clone() {
            if self
                .host
                .session(&id)
                .is_some_and(|session| session.lifecycle_state().is_interactive())
            {
                return Some(id);
            }
        }
        if let Some(id) = self.interactive_session_for_workspace(&workspace_id) {
            self.active_session_id = Some(id.clone());
            return Some(id);
        }
        Some(self.open_session_for_active_workspace(trust_state, observed_at, None))
    }

    #[cfg(test)]
    fn open_command_session_for_test(
        &mut self,
        workspace_id: &str,
        workspace_root: PathBuf,
        command: aureline_terminal::PtyCommand,
        observed_at: &str,
    ) -> PtySessionId {
        self.close_active_workspace(observed_at, Some("test_workspace_changed"));
        self.active_workspace_id = Some(workspace_id.to_owned());
        self.workspace_root = Some(workspace_root);
        self.open_session_for_active_workspace(TrustState::Trusted, observed_at, Some(command))
    }

    fn write_active_input(
        &mut self,
        bytes: &[u8],
        trust_state: TrustState,
        observed_at: &str,
    ) -> Result<(), String> {
        let Some(session_id) = self.ensure_session_for_active_workspace(trust_state, observed_at)
        else {
            return Err("terminal unavailable: open a workspace first".to_string());
        };
        self.host
            .write_input(&session_id, bytes, observed_at)
            .map_err(|err| err.to_string())?;
        let _ = self.drain_outputs(observed_at);
        Ok(())
    }

    fn drain_outputs(&mut self, observed_at: &str) -> bool {
        let ids: Vec<PtySessionId> = self
            .host
            .sessions()
            .filter(|session| session.has_live_pty())
            .map(|session| session.session_id().clone())
            .collect();
        let mut changed = false;
        for id in ids {
            if let Ok(drain) = self.host.drain_output(&id, observed_at) {
                changed |= self.append_output(&id, &drain.bytes);
            }
        }
        changed
    }

    fn has_live_sessions(&self) -> bool {
        self.host.sessions().any(|session| {
            session.header().workspace_id == self.active_workspace_id.as_deref().unwrap_or("")
                && session.has_live_pty()
                && session.lifecycle_state().is_interactive()
        })
    }

    fn snapshot(&self) -> Option<TerminalPaneSnapshot> {
        self.active_workspace_id
            .as_deref()
            .map(|workspace_id| TerminalPaneSnapshot::project(workspace_id, &self.host))
    }

    fn active_output_text(&self) -> &str {
        self.active_session_id
            .as_ref()
            .and_then(|id| self.output_buffers.get(id))
            .map(String::as_str)
            .unwrap_or("")
    }

    fn active_host_boundary_card(&self) -> Option<HostBoundaryCueCardRecord> {
        let workspace_id = self.active_workspace_id.as_deref()?;
        self.host_boundary_wedges
            .get(workspace_id)
            .map(HostBoundaryCueWedge::card)
    }

    fn active_workspace_id(&self) -> Option<&str> {
        self.active_workspace_id.as_deref()
    }

    fn interactive_session_for_workspace(&self, workspace_id: &str) -> Option<PtySessionId> {
        self.host
            .sessions()
            .find(|session| {
                session.header().workspace_id == workspace_id
                    && session.lifecycle_state().is_interactive()
            })
            .map(|session| session.session_id().clone())
    }

    fn open_session_for_active_workspace(
        &mut self,
        trust_state: TrustState,
        observed_at: &str,
        command: Option<aureline_terminal::PtyCommand>,
    ) -> PtySessionId {
        let workspace_id = self
            .active_workspace_id
            .clone()
            .unwrap_or_else(|| "workspace:none".to_string());
        let workspace_root = self
            .workspace_root
            .clone()
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));
        let context = resolve_terminal_execution_context(
            &workspace_id,
            &workspace_root,
            trust_state,
            observed_at,
        );
        let display_title = default_terminal_display_title();
        let request = OpenSessionRequest {
            workspace_id: &workspace_id,
            host_class: host_class_for_execution_context(&context),
            display_title: &display_title,
            cwd_hint: context.target_identity.working_directory.as_deref(),
            execution_context_ref: context.execution_context_id(),
            trust_state: context.policy_and_trust.trust_state,
            observed_at,
        };
        let session_id = match command {
            Some(command) => self.host.open_command_session(request, command),
            None => self.host.open_session(request),
        };
        self.contexts_by_session
            .insert(session_id.clone(), context.clone());
        self.output_buffers.entry(session_id.clone()).or_default();
        self.active_session_id = Some(session_id.clone());

        if let Some(session) = self.host.session(&session_id).cloned() {
            let mut wedge = HostBoundaryCueWedge::new(workspace_id.clone());
            let _ = wedge.open_initial(&context, &session, observed_at);
            self.host_boundary_wedges.insert(workspace_id, wedge);
        }

        session_id
    }

    fn append_output(&mut self, session_id: &PtySessionId, bytes: &[u8]) -> bool {
        if bytes.is_empty() {
            return false;
        }
        let raw = String::from_utf8_lossy(bytes);
        let text = strip_terminal_control_sequences(&raw);
        if text.is_empty() {
            return false;
        }
        let buffer = self.output_buffers.entry(session_id.clone()).or_default();
        buffer.push_str(&text);
        if buffer.len() > TERMINAL_BUFFER_MAX_BYTES {
            let keep_from = buffer.len().saturating_sub(TERMINAL_BUFFER_MAX_BYTES);
            let split_at = buffer
                .char_indices()
                .find(|(idx, _)| *idx >= keep_from)
                .map(|(idx, _)| idx)
                .unwrap_or(keep_from);
            buffer.drain(..split_at);
        }
        true
    }
}

fn resolve_terminal_execution_context(
    workspace_id: &str,
    workspace_root: &Path,
    trust_state: TrustState,
    observed_at: &str,
) -> ExecutionContext {
    let mut resolver = ExecutionContextResolver::new(ExecutionContextResolverConfig {
        workspace_id: workspace_id.to_owned(),
        profile_id: Some("profile:local".to_string()),
        identity_mode: IdentityMode::AccountFreeLocal,
        policy_epoch: 1,
        workspace_default_target_class: TargetClass::LocalHost,
        workspace_default_working_directory: Some(workspace_root.display().to_string()),
        workspace_default_scope_class: ScopeClass::CurrentRoot,
        local_host_canonical_id: format!(
            "localhost:{}-{}",
            std::env::consts::OS,
            std::env::consts::ARCH
        ),
        environment_capsule_ref: EnvironmentCapsuleRef {
            capsule_id: format!("caps:{workspace_id}:terminal"),
            capsule_hash: "sha256:terminal-pane-seed".to_string(),
            resolved_schema_version: "1".to_string(),
            drift_state: CapsuleDriftState::InSync,
        },
        resolver_version: "native-shell-terminal-pane-1".to_string(),
    });
    resolver.resolve(ExecutionContextRequest::local_terminal_seed(
        "cmd:terminal.toggle",
        trust_state,
        observed_at,
    ))
}

fn host_class_for_execution_context(context: &ExecutionContext) -> HostClass {
    match context.target_identity.target_class {
        TargetClass::LocalHost => HostClass::HostDesktop,
        TargetClass::ContainerLocal | TargetClass::Devcontainer => HostClass::LocalContainer,
        _ => HostClass::RemoteAgentPrimary,
    }
}

fn default_terminal_display_title() -> String {
    std::env::var("SHELL")
        .ok()
        .and_then(|shell| {
            Path::new(&shell)
                .file_name()
                .map(|name| name.to_string_lossy().into_owned())
        })
        .filter(|label| !label.trim().is_empty())
        .unwrap_or_else(|| "terminal".to_string())
}

fn strip_terminal_control_sequences(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' {
            match chars.peek().copied() {
                Some('[') => {
                    let _ = chars.next();
                    for next in chars.by_ref() {
                        if ('@'..='~').contains(&next) {
                            break;
                        }
                    }
                }
                Some(']') => {
                    let _ = chars.next();
                    let mut prior_was_escape = false;
                    for next in chars.by_ref() {
                        if next == '\u{7}' || (prior_was_escape && next == '\\') {
                            break;
                        }
                        prior_was_escape = next == '\u{1b}';
                    }
                }
                Some(_) => {
                    let _ = chars.next();
                }
                None => {}
            }
            continue;
        }
        if ch == '\r' {
            out.push('\n');
        } else if ch == '\n' || ch == '\t' || !ch.is_control() {
            out.push(ch);
        }
    }
    out
}

fn terminal_input_bytes_for_key_event(
    code: KeyCode,
    text: Option<&str>,
    modifiers: HeldModifiers,
    platform: PlatformClass,
) -> Option<Vec<u8>> {
    if is_terminal_toggle_chord(code, modifiers, platform) {
        return None;
    }
    match code {
        KeyCode::Enter => Some(b"\r".to_vec()),
        KeyCode::Backspace => Some(vec![0x7f]),
        KeyCode::Delete => Some(b"\x1b[3~".to_vec()),
        KeyCode::Tab if !modifiers.ctrl_or_logo() => Some(b"\t".to_vec()),
        KeyCode::Escape => Some(vec![0x1b]),
        KeyCode::ArrowUp if !modifiers.ctrl_or_logo() => Some(b"\x1b[A".to_vec()),
        KeyCode::ArrowDown if !modifiers.ctrl_or_logo() => Some(b"\x1b[B".to_vec()),
        KeyCode::ArrowRight if !modifiers.ctrl_or_logo() => Some(b"\x1b[C".to_vec()),
        KeyCode::ArrowLeft if !modifiers.ctrl_or_logo() => Some(b"\x1b[D".to_vec()),
        code if modifiers.ctrl && !modifiers.alt && !modifiers.logo && !modifiers.shift => {
            control_byte_for_keycode(code).map(|byte| vec![byte])
        }
        _ if !modifiers.ctrl && !modifiers.logo => text
            .filter(|value| !value.is_empty())
            .map(|value| value.as_bytes().to_vec()),
        _ => None,
    }
}

fn control_byte_for_keycode(code: KeyCode) -> Option<u8> {
    let byte = match code {
        KeyCode::KeyA => 0x01,
        KeyCode::KeyB => 0x02,
        KeyCode::KeyC => 0x03,
        KeyCode::KeyD => 0x04,
        KeyCode::KeyE => 0x05,
        KeyCode::KeyF => 0x06,
        KeyCode::KeyG => 0x07,
        KeyCode::KeyH => 0x08,
        KeyCode::KeyI => 0x09,
        KeyCode::KeyJ => 0x0a,
        KeyCode::KeyK => 0x0b,
        KeyCode::KeyL => 0x0c,
        KeyCode::KeyM => 0x0d,
        KeyCode::KeyN => 0x0e,
        KeyCode::KeyO => 0x0f,
        KeyCode::KeyP => 0x10,
        KeyCode::KeyQ => 0x11,
        KeyCode::KeyR => 0x12,
        KeyCode::KeyS => 0x13,
        KeyCode::KeyT => 0x14,
        KeyCode::KeyU => 0x15,
        KeyCode::KeyV => 0x16,
        KeyCode::KeyW => 0x17,
        KeyCode::KeyX => 0x18,
        KeyCode::KeyY => 0x19,
        KeyCode::KeyZ => 0x1a,
        KeyCode::BracketLeft => 0x1b,
        KeyCode::Backslash => 0x1c,
        KeyCode::BracketRight => 0x1d,
        _ => return None,
    };
    Some(byte)
}

fn is_terminal_toggle_chord(
    code: KeyCode,
    modifiers: HeldModifiers,
    platform: PlatformClass,
) -> bool {
    if code != KeyCode::Backquote || modifiers.alt || modifiers.shift {
        return false;
    }
    match platform {
        PlatformClass::Macos => modifiers.logo && !modifiers.ctrl,
        _ => modifiers.ctrl && !modifiers.logo,
    }
}

#[cfg(test)]
mod terminal_routing_tests {
    use super::*;

    #[cfg(unix)]
    use std::thread;

    #[cfg(unix)]
    #[test]
    fn bottom_panel_text_input_reaches_active_pty() {
        let workspace = tempfile::tempdir().expect("workspace tempdir");
        let mut runtime = TerminalPaneRuntimeState::new();
        runtime.open_command_session_for_test(
            "ws-test",
            workspace.path().to_path_buf(),
            aureline_terminal::PtyCommand::new("/bin/sh"),
            "mono:0",
        );
        let modifiers = HeldModifiers::default();
        for ch in "echo routed".chars() {
            let text = ch.to_string();
            let bytes = terminal_input_bytes_for_key_event(
                KeyCode::Space,
                Some(&text),
                modifiers,
                PlatformClass::Linux,
            )
            .expect("printable key routes");
            runtime
                .write_active_input(&bytes, TrustState::Trusted, "mono:input")
                .expect("write printable key");
        }
        let enter = terminal_input_bytes_for_key_event(
            KeyCode::Enter,
            None,
            modifiers,
            PlatformClass::Linux,
        )
        .expect("enter routes");
        runtime
            .write_active_input(&enter, TrustState::Trusted, "mono:enter")
            .expect("write enter");

        let mut observed = String::new();
        for _ in 0..40 {
            let _ = runtime.drain_outputs("mono:poll");
            observed = runtime.active_output_text().to_string();
            if observed.contains("routed") {
                break;
            }
            thread::sleep(Duration::from_millis(25));
        }

        runtime.close_active_workspace("mono:close", Some("test_complete"));
        assert!(
            observed.contains("routed"),
            "expected terminal output to contain routed command output, got: {observed:?}"
        );
    }

    #[test]
    fn platform_terminal_toggle_chords_are_not_sent_to_pty() {
        let modifiers = HeldModifiers {
            ctrl: true,
            alt: false,
            shift: false,
            logo: false,
        };
        assert!(terminal_input_bytes_for_key_event(
            KeyCode::Backquote,
            None,
            modifiers,
            PlatformClass::Linux,
        )
        .is_none());

        let mac_modifiers = HeldModifiers {
            ctrl: false,
            alt: false,
            shift: false,
            logo: true,
        };
        assert!(terminal_input_bytes_for_key_event(
            KeyCode::Backquote,
            None,
            mac_modifiers,
            PlatformClass::Macos,
        )
        .is_none());
    }
}

const EXPLORER_ROOT_ID: &str = "root-local";
const EXPLORER_ROW_HIT_HEIGHT_PX: u32 = 24;
const EXPLORER_HEADER_HIT_HEIGHT_PX: u32 = 48;
const EXPLORER_HIT_INSET_PX: u32 = 8;

#[derive(Debug, Clone)]
struct ExplorerViewRuntime {
    tree: ExplorerTree,
    workspace_id: Option<String>,
    workspace_root: Option<PathBuf>,
    root_node_id: Option<ExplorerNodeId>,
    root_kind: WorkspaceRootKind,
    loaded_dirs: BTreeSet<ExplorerNodeId>,
    path_by_node_id: BTreeMap<ExplorerNodeId, PathBuf>,
    node_id_by_path: BTreeMap<PathBuf, ExplorerNodeId>,
    watcher_health: WatcherHealth,
    last_error: Option<String>,
}

impl ExplorerViewRuntime {
    fn new() -> Self {
        Self {
            tree: ExplorerTree::new(),
            workspace_id: None,
            workspace_root: None,
            root_node_id: None,
            root_kind: WorkspaceRootKind::LocalFolder,
            loaded_dirs: BTreeSet::new(),
            path_by_node_id: BTreeMap::new(),
            node_id_by_path: BTreeMap::new(),
            watcher_health: WatcherHealth::Warming,
            last_error: None,
        }
    }

    fn open_workspace(&mut self, root: PathBuf, workspace_id: String) -> Result<(), String> {
        let root = root.canonicalize().unwrap_or(root);
        let root_kind = if root.join(".git").is_dir() {
            WorkspaceRootKind::LocalRepoRoot
        } else {
            WorkspaceRootKind::LocalFolder
        };
        let display_label = root
            .file_name()
            .and_then(|name| name.to_str())
            .filter(|name| !name.trim().is_empty())
            .map(ToOwned::to_owned)
            .unwrap_or_else(|| root.display().to_string());

        let mut tree = ExplorerTree::new();
        let root_node = ExplorerNode::root_mount(
            &workspace_id,
            EXPLORER_ROOT_ID,
            root_kind,
            display_label,
            NodeReadinessClass::Loaded,
        );
        let root_node_id = root_node.node_id.clone();
        tree.insert(root_node).map_err(|err| err.to_string())?;
        tree.expand(&root_node_id).map_err(|err| err.to_string())?;

        self.tree = tree;
        self.workspace_id = Some(workspace_id);
        self.workspace_root = Some(root.clone());
        self.root_node_id = Some(root_node_id.clone());
        self.root_kind = root_kind;
        self.loaded_dirs.clear();
        self.path_by_node_id.clear();
        self.node_id_by_path.clear();
        self.path_by_node_id
            .insert(root_node_id.clone(), root.clone());
        self.node_id_by_path.insert(root, root_node_id.clone());
        self.loaded_dirs.insert(root_node_id.clone());
        self.watcher_health = WatcherHealth::Warming;
        self.last_error = None;

        self.scan_children(&root_node_id)?;
        self.select_first_meaningful_row();
        Ok(())
    }

    fn clear_workspace(&mut self) {
        *self = Self::new();
    }

    fn visible_rows(&self) -> Vec<ExplorerViewportRow> {
        self.tree.visible_rows()
    }

    fn watcher_health(&self) -> WatcherHealth {
        self.watcher_health
    }

    fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    fn selected_file_path(&self) -> Option<PathBuf> {
        let id = self.tree.selected()?;
        let node = self.tree.node(id)?;
        if !node.kind.opens_in_editor() {
            return None;
        }
        self.path_by_node_id.get(id).cloned()
    }

    fn activate_selected_directory(&mut self) -> bool {
        let Some(id) = self.tree.selected().cloned() else {
            return false;
        };
        let Some(node) = self.tree.node(&id) else {
            return false;
        };
        if !node.kind.may_have_children() {
            return false;
        }
        if self.tree.is_expanded(&id) && !node.is_persistent_mount() {
            return self.tree.collapse(&id).is_ok();
        }
        self.expand_node(&id)
    }

    fn expand_node(&mut self, id: &ExplorerNodeId) -> bool {
        let Some(node) = self.tree.node(id) else {
            return false;
        };
        if !node.kind.may_have_children() {
            return false;
        }
        let was_loaded = self.loaded_dirs.contains(id);
        let was_expanded = self.tree.is_expanded(id);
        if !was_loaded {
            if let Err(err) = self.scan_children(id) {
                self.last_error = Some(err);
                return false;
            }
        }
        self.tree.expand(id).is_ok() && (!was_loaded || !was_expanded)
    }

    fn collapse_or_select_parent(&mut self) -> bool {
        let Some(id) = self.tree.selected().cloned() else {
            return false;
        };
        let Some(node) = self.tree.node(&id).cloned() else {
            return false;
        };
        if node.kind.may_have_children()
            && self.tree.is_expanded(&id)
            && !node.is_persistent_mount()
        {
            return self.tree.collapse(&id).is_ok();
        }
        let Some(parent) = node.parent_id else {
            return false;
        };
        self.tree.select(&parent).is_ok()
    }

    fn select_next(&mut self) -> bool {
        self.select_relative(1)
    }

    fn select_prev(&mut self) -> bool {
        self.select_relative(-1)
    }

    fn select_relative(&mut self, delta: i32) -> bool {
        let rows = self.visible_rows();
        if rows.is_empty() {
            return false;
        }
        let current = self
            .tree
            .selected()
            .and_then(|id| rows.iter().position(|row| &row.node_id == id))
            .unwrap_or(0);
        let last = rows.len().saturating_sub(1) as i32;
        let next = (current as i32 + delta).clamp(0, last) as usize;
        if next == current {
            return false;
        }
        self.tree.select(&rows[next].node_id).is_ok()
    }

    fn select_row(&mut self, row_index: usize) -> bool {
        let rows = self.visible_rows();
        let Some(row) = rows.get(row_index) else {
            return false;
        };
        self.tree.select(&row.node_id).is_ok()
    }

    fn toggle_row(&mut self, row_index: usize) -> bool {
        let rows = self.visible_rows();
        let Some(row) = rows.get(row_index) else {
            return false;
        };
        let id = row.node_id.clone();
        if self.tree.select(&id).is_err() {
            return false;
        }
        let Some(node) = self.tree.node(&id) else {
            return true;
        };
        if !node.kind.may_have_children() {
            return true;
        }
        self.activate_selected_directory()
    }

    fn apply_watcher_events(&mut self, events: Vec<WatcherEvent>) -> bool {
        let mut changed = false;
        for event in events {
            match event {
                WatcherEvent::Health(frame) => {
                    if self.watcher_health != frame.watcher_health {
                        self.watcher_health = frame.watcher_health;
                        changed = true;
                    }
                }
                WatcherEvent::Change(change) => {
                    if change.root_id != EXPLORER_ROOT_ID {
                        continue;
                    }
                    changed |= self.apply_change(change.kind);
                }
            }
        }
        changed
    }

    fn apply_change(&mut self, kind: VfsChangeKind) -> bool {
        match kind {
            VfsChangeKind::Created { uri } => {
                let Some(path) = uri.file_path() else {
                    return self.rescan_loaded_dirs();
                };
                self.add_path_if_parent_loaded(path)
            }
            VfsChangeKind::Deleted { uri } => {
                let Some(path) = uri.file_path() else {
                    return self.rescan_loaded_dirs();
                };
                self.remove_path(path)
            }
            VfsChangeKind::Renamed { from, to } => {
                let removed = from.file_path().is_some_and(|path| self.remove_path(path));
                let added = to
                    .file_path()
                    .is_some_and(|path| self.add_path_if_parent_loaded(path));
                removed || added
            }
            VfsChangeKind::Modified { .. } => false,
            VfsChangeKind::Rescan => self.rescan_loaded_dirs(),
        }
    }

    fn rescan_loaded_dirs(&mut self) -> bool {
        let loaded_paths: Vec<PathBuf> = self
            .loaded_dirs
            .iter()
            .filter_map(|id| self.path_by_node_id.get(id).cloned())
            .collect();
        if loaded_paths.is_empty() {
            return false;
        }
        let selected_path = self
            .tree
            .selected()
            .and_then(|id| self.path_by_node_id.get(id).cloned());

        let Some(root) = self.workspace_root.clone() else {
            return false;
        };
        let Some(workspace_id) = self.workspace_id.clone() else {
            return false;
        };
        if self.open_workspace(root, workspace_id).is_err() {
            return false;
        }
        for path in loaded_paths {
            if let Some(id) = self.node_id_by_path.get(&path).cloned() {
                let _ = self.expand_node(&id);
            }
        }
        if let Some(path) = selected_path {
            if let Some(id) = self.node_id_by_path.get(&path).cloned() {
                let _ = self.tree.select(&id);
            }
        }
        true
    }

    fn scan_children(&mut self, parent_id: &ExplorerNodeId) -> Result<(), String> {
        let parent_path = self
            .path_by_node_id
            .get(parent_id)
            .cloned()
            .ok_or_else(|| "explorer parent path missing".to_string())?;
        let parent_node = self
            .tree
            .node(parent_id)
            .cloned()
            .ok_or_else(|| "explorer parent node missing".to_string())?;
        if !parent_node.kind.may_have_children() {
            return Ok(());
        }

        let removed = self
            .tree
            .clear_children(parent_id)
            .map_err(|err| err.to_string())?;
        for id in removed {
            self.drop_path_mappings_for_subtree(&id);
        }

        let mut entries = Vec::new();
        let read_dir = std::fs::read_dir(&parent_path)
            .map_err(|err| format!("explorer scan failed for {}: {err}", parent_path.display()))?;
        for entry in read_dir.flatten() {
            let path = entry.path();
            let Ok(file_type) = entry.file_type() else {
                continue;
            };
            if file_type.is_dir() {
                if let Some(name) = path.file_name().and_then(|name| name.to_str()) {
                    if crate::palette::query_session::is_workspace_file_index_ignored_dir(name) {
                        continue;
                    }
                }
            } else if !file_type.is_file() {
                continue;
            }
            entries.push((path, file_type.is_dir(), file_type.is_file()));
        }
        entries.sort_by(|(left_path, left_dir, _), (right_path, right_dir, _)| {
            right_dir
                .cmp(left_dir)
                .then_with(|| explorer_path_label(left_path).cmp(&explorer_path_label(right_path)))
        });

        for (path, is_dir, is_file) in entries {
            if let Some(node) = self.node_for_path(parent_id, &parent_node, path, is_dir, is_file) {
                let id = node.node_id.clone();
                let path = self.normalize_path_for_map(self.path_from_node(&node));
                self.tree.insert(node).map_err(|err| err.to_string())?;
                self.path_by_node_id.insert(id.clone(), path.clone());
                self.node_id_by_path.insert(path, id);
            }
        }
        self.loaded_dirs.insert(parent_id.clone());
        let _ = self
            .tree
            .set_readiness(parent_id, NodeReadinessClass::Loaded);
        Ok(())
    }

    fn add_path_if_parent_loaded(&mut self, path: PathBuf) -> bool {
        let path = self.normalize_path_for_map(path);
        if self.node_id_by_path.contains_key(&path) {
            return false;
        }
        let Some(parent_path) = path
            .parent()
            .map(|p| self.normalize_path_for_map(p.to_path_buf()))
        else {
            return false;
        };
        let Some(parent_id) = self.node_id_by_path.get(&parent_path).cloned() else {
            return false;
        };
        if !self.loaded_dirs.contains(&parent_id) {
            return false;
        }
        let Ok(metadata) = std::fs::metadata(&path) else {
            return false;
        };
        let is_dir = metadata.is_dir();
        let is_file = metadata.is_file();
        if !is_dir && !is_file {
            return false;
        }
        if self.path_has_ignored_directory(&path, is_dir) {
            return false;
        }
        let Some(parent_node) = self.tree.node(&parent_id).cloned() else {
            return false;
        };
        let Some(node) =
            self.node_for_path(&parent_id, &parent_node, path.clone(), is_dir, is_file)
        else {
            return false;
        };
        let id = node.node_id.clone();
        if self.tree.insert(node).is_err() {
            return false;
        }
        self.path_by_node_id.insert(id.clone(), path.clone());
        self.node_id_by_path.insert(path, id);
        true
    }

    fn remove_path(&mut self, path: PathBuf) -> bool {
        let path = self.normalize_path_for_map(path);
        let Some(id) = self.node_id_by_path.get(&path).cloned() else {
            return false;
        };
        let descendants = self.descendant_ids_for_path(&path);
        if self.tree.remove_subtree(&id).is_err() {
            return false;
        }
        for descendant in descendants {
            if let Some(mapped_path) = self.path_by_node_id.remove(&descendant) {
                self.node_id_by_path.remove(&mapped_path);
            }
            self.loaded_dirs.remove(&descendant);
        }
        if self.tree.selected().is_none() {
            self.select_first_meaningful_row();
        }
        true
    }

    fn node_for_path(
        &self,
        parent_id: &ExplorerNodeId,
        parent_node: &ExplorerNode,
        path: PathBuf,
        is_dir: bool,
        is_file: bool,
    ) -> Option<ExplorerNode> {
        let root = self.workspace_root.as_ref()?;
        let workspace_id = self.workspace_id.as_ref()?;
        let relative_path = path.strip_prefix(root).ok()?;
        let relative = relative_path.to_string_lossy().replace('\\', "/");
        if relative.is_empty() {
            return None;
        }
        let logical_uri = join_explorer_logical(&self.root_logical_uri()?, &relative);
        let display_label = explorer_path_label(&path);
        let canonical_uri = VfsUri::file_url_for_path(&path)
            .or_else(|| VfsUri::file_url_for_path_lossy(&path))
            .map(|uri| uri.into_string())
            .unwrap_or_else(|| logical_uri.clone());
        let generated_artifact_hint = if is_dir {
            None
        } else {
            GeneratedArtifactHint::detect_for(&relative, Some(workspace_id), Some(EXPLORER_ROOT_ID))
        };
        let kind = if is_dir {
            ExplorerNodeKind::Directory
        } else if is_file && generated_artifact_hint.is_some() {
            ExplorerNodeKind::GeneratedArtifact
        } else if is_file {
            ExplorerNodeKind::File
        } else {
            return None;
        };
        Some(ExplorerNode {
            node_id: ExplorerNodeId::from_logical(workspace_id, EXPLORER_ROOT_ID, &logical_uri),
            workspace_id: workspace_id.clone(),
            root_id: EXPLORER_ROOT_ID.to_string(),
            root_kind: self.root_kind,
            kind,
            depth: parent_node.depth.saturating_add(1),
            display_label,
            presentation_uri: canonical_uri.clone(),
            canonical_uri,
            logical_uri,
            root_badge: parent_node.root_badge.clone(),
            parent_id: Some(parent_id.clone()),
            readiness: if is_dir {
                NodeReadinessClass::ManifestKnown
            } else {
                NodeReadinessClass::Loaded
            },
            generated_artifact_hint,
            special_file_hint: None,
        })
    }

    fn root_logical_uri(&self) -> Option<String> {
        let id = self.root_node_id.as_ref()?;
        self.tree.node(id).map(|node| node.logical_uri.clone())
    }

    fn path_from_node(&self, node: &ExplorerNode) -> PathBuf {
        self.workspace_root
            .as_ref()
            .and_then(|root| {
                let root_logical = self.root_logical_uri()?;
                let relative = node.logical_uri.strip_prefix(&root_logical)?;
                Some(root.join(relative))
            })
            .unwrap_or_else(|| PathBuf::from(&node.presentation_uri))
    }

    fn normalize_path_for_map(&self, path: PathBuf) -> PathBuf {
        path.canonicalize().unwrap_or(path)
    }

    fn path_has_ignored_directory(&self, path: &Path, path_is_dir: bool) -> bool {
        let Some(root) = self.workspace_root.as_ref() else {
            return true;
        };
        let Ok(relative) = path.strip_prefix(root) else {
            return true;
        };
        let ignored_component_count = if path_is_dir {
            relative.components().count()
        } else {
            relative.components().count().saturating_sub(1)
        };
        relative
            .components()
            .take(ignored_component_count)
            .any(|component| {
                let label = component.as_os_str().to_string_lossy();
                crate::palette::query_session::is_workspace_file_index_ignored_dir(&label)
            })
    }

    fn descendant_ids_for_path(&self, path: &Path) -> Vec<ExplorerNodeId> {
        self.node_id_by_path
            .iter()
            .filter_map(|(candidate_path, id)| {
                candidate_path.starts_with(path).then_some(id.clone())
            })
            .collect()
    }

    fn drop_path_mappings_for_subtree(&mut self, id: &ExplorerNodeId) {
        let Some(path) = self.path_by_node_id.get(id).cloned() else {
            return;
        };
        for descendant in self.descendant_ids_for_path(&path) {
            if let Some(mapped_path) = self.path_by_node_id.remove(&descendant) {
                self.node_id_by_path.remove(&mapped_path);
            }
            self.loaded_dirs.remove(&descendant);
        }
    }

    fn select_first_meaningful_row(&mut self) {
        let rows = self.visible_rows();
        let selected = rows
            .iter()
            .find(|row| row.depth > 0)
            .or_else(|| rows.first());
        if let Some(row) = selected {
            let _ = self.tree.select(&row.node_id);
        }
    }
}

fn join_explorer_logical(root_logical_uri: &str, relative: &str) -> String {
    let relative = relative.trim_start_matches('/');
    if root_logical_uri.ends_with('/') {
        format!("{root_logical_uri}{relative}")
    } else {
        format!("{root_logical_uri}/{relative}")
    }
}

fn explorer_path_label(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.trim().is_empty())
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| path.display().to_string())
}

#[cfg(test)]
mod explorer_view_runtime_tests {
    use super::*;

    use std::fs;

    use aureline_vfs::VfsChangeEvent;

    fn fixture_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/repos/m1/nested_source_tree")
            .canonicalize()
            .expect("nested source fixture should exist")
    }

    fn labels(explorer: &ExplorerViewRuntime) -> Vec<String> {
        explorer
            .visible_rows()
            .into_iter()
            .map(|row| row.display_label)
            .collect()
    }

    fn selected_label(explorer: &ExplorerViewRuntime) -> Option<String> {
        let selected = explorer.tree.selected()?;
        explorer
            .tree
            .node(selected)
            .map(|node| node.display_label.clone())
    }

    #[test]
    fn scans_fixture_root_one_level_and_expands_directories_lazily() {
        let root = fixture_root();
        let mut explorer = ExplorerViewRuntime::new();
        explorer
            .open_workspace(root.clone(), "workspace:test".to_string())
            .expect("fixture workspace should scan");

        let root_labels = labels(&explorer);
        assert!(root_labels.iter().any(|label| label == "app"));
        assert!(root_labels.iter().any(|label| label == "src"));
        assert!(root_labels.iter().any(|label| label == "README.md"));
        assert!(!root_labels.iter().any(|label| label == "home.md"));

        let app_id = explorer
            .node_id_by_path
            .get(&root.join("app").canonicalize().expect("app dir"))
            .cloned()
            .expect("app node should be indexed");
        assert!(explorer.expand_node(&app_id));
        let app_labels = labels(&explorer);
        assert!(app_labels.iter().any(|label| label == "pages"));
        assert!(app_labels.iter().any(|label| label == "routes.ts"));
        assert!(!app_labels.iter().any(|label| label == "home.md"));

        let pages_id = explorer
            .node_id_by_path
            .get(&root.join("app/pages").canonicalize().expect("pages dir"))
            .cloned()
            .expect("pages node should be indexed after expanding app");
        assert!(explorer.expand_node(&pages_id));
        let pages_labels = labels(&explorer);
        assert!(pages_labels.iter().any(|label| label == "home.md"));
        assert!(pages_labels.iter().any(|label| label == "about.md"));
    }

    #[test]
    fn selection_navigation_reaches_files_for_editor_activation() {
        let root = fixture_root();
        let mut explorer = ExplorerViewRuntime::new();
        explorer
            .open_workspace(root.clone(), "workspace:test".to_string())
            .expect("fixture workspace should scan");

        assert_eq!(selected_label(&explorer).as_deref(), Some("app"));
        assert!(explorer.select_next());
        assert_eq!(selected_label(&explorer).as_deref(), Some("src"));
        assert!(explorer.select_next());
        assert_eq!(selected_label(&explorer).as_deref(), Some("README.md"));
        assert_eq!(explorer.selected_file_path(), Some(root.join("README.md")));
    }

    #[test]
    fn watcher_created_file_event_updates_loaded_parent_without_rescan() {
        let temp = tempfile::tempdir().expect("tempdir");
        let root = temp.path().join("workspace");
        fs::create_dir(&root).expect("workspace dir");
        fs::write(root.join("README.md"), "fixture\n").expect("seed file");

        let mut explorer = ExplorerViewRuntime::new();
        explorer
            .open_workspace(root.clone(), "workspace:test".to_string())
            .expect("temp workspace should scan");

        let created_path = root.join("created.rs");
        fs::write(&created_path, "fn main() {}\n").expect("created file");
        let uri = VfsUri::file_url_for_path(&created_path)
            .or_else(|| VfsUri::file_url_for_path_lossy(&created_path))
            .expect("file URI");

        let changed = explorer.apply_watcher_events(vec![WatcherEvent::Change(VfsChangeEvent {
            root_id: EXPLORER_ROOT_ID.to_string(),
            kind: VfsChangeKind::Created { uri },
        })]);

        assert!(changed);
        assert!(labels(&explorer).iter().any(|label| label == "created.rs"));
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
    labs_enabled: bool,
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
            labs_enabled: load_labs_wedge_inspector_enabled(),
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

    fn toggle_policy_blocked(&mut self) {
        self.policy_blocked_in_context = !self.policy_blocked_in_context;
    }
}

fn load_labs_wedge_inspector_enabled() -> bool {
    if let Ok(value) = env::var("AURELINE_LABS_WEDGE_INSPECTOR") {
        if let Some(enabled) = parse_bool_token(&value) {
            return enabled;
        }
    }
    if let Ok(value) = env::var("AURELINE_LABS_ENABLED") {
        if let Some(enabled) = parse_bool_token(&value) {
            return enabled;
        }
    }

    [
        PathBuf::from(".aureline").join("settings.json"),
        PathBuf::from(".aureline").join("settings.jsonc"),
        PathBuf::from(".logs").join("settings").join("labs.json"),
    ]
    .into_iter()
    .any(|path| {
        std::fs::read_to_string(path)
            .ok()
            .and_then(|raw| setting_bool(&raw, "shell.labs.wedge_inspector_enabled"))
            .unwrap_or(false)
    })
}

fn parse_bool_token(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" | "enabled" => Some(true),
        "0" | "false" | "no" | "off" | "disabled" => Some(false),
        _ => None,
    }
}

fn setting_bool(raw: &str, setting_id: &str) -> Option<bool> {
    if let Ok(value) = serde_json::from_str::<serde_json::Value>(raw) {
        return value
            .get(setting_id)
            .and_then(serde_json::Value::as_bool)
            .or_else(|| {
                value
                    .get("labs")
                    .and_then(|labs| labs.get("wedge_inspector_enabled"))
                    .and_then(serde_json::Value::as_bool)
            });
    }

    let compact: String = raw.chars().filter(|ch| !ch.is_whitespace()).collect();
    let flat_key = format!("\"{setting_id}\":true");
    if compact.contains(&flat_key) {
        return Some(true);
    }
    let nested_key = "\"wedge_inspector_enabled\":true";
    if compact.contains(nested_key) {
        return Some(true);
    }
    None
}

#[derive(Debug, Clone)]
struct WorkspaceLifecycleRuntimeState {
    machine: Option<WorkspaceLifecycleMachine>,
    last_logged: Option<WorkspaceLifecycleLogKey>,
    snapshot_path: PathBuf,
    transitions_path: PathBuf,
    last_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct WorkspaceLifecycleLogKey {
    workspace_id: String,
    lifecycle_state: WorkspaceLifecycleState,
    trust_state: TrustState,
    watcher_health: Option<WatcherHealth>,
    hot_index_ready: bool,
    command_graph_ready: bool,
    last_transition_reason_code: Option<String>,
}

impl WorkspaceLifecycleRuntimeState {
    fn new() -> Self {
        let base = PathBuf::from(".logs").join("workspace");
        Self {
            machine: None,
            last_logged: None,
            snapshot_path: base.join("workspace_lifecycle_snapshot.json"),
            transitions_path: base.join("workspace_lifecycle_transitions.jsonl"),
            last_error: None,
        }
    }

    fn open_local_folder(&mut self, workspace_id: String, trust_state: TrustState) {
        let observed_at = mono_timestamp_now();
        let mut machine = WorkspaceLifecycleMachine::discovered(workspace_id, observed_at.clone());
        machine.open_workspace(observed_at.clone());
        machine.resolve_trust(trust_state, observed_at.clone());
        machine.mark_shell_interactive(observed_at);
        machine.update_readiness_gates(
            None,
            None,
            Some(true),
            mono_timestamp_now(),
            Some("command_graph_ready".to_string()),
        );
        self.machine = Some(machine);
        self.last_logged = None;
        self.flush_logs_if_changed();
    }

    fn resolve_trust(&mut self, trust_state: TrustState) -> bool {
        let Some(machine) = self.machine.as_mut() else {
            return false;
        };
        let before_state = machine.state();
        let before_trust = machine.trust_state();
        machine.resolve_trust(trust_state, mono_timestamp_now());
        let changed = machine.state() != before_state || machine.trust_state() != before_trust;
        self.flush_logs_if_changed();
        changed
    }

    fn update_from_file_index(
        &mut self,
        watcher_health: Option<WatcherHealth>,
        hot_index_ready: Option<bool>,
    ) -> bool {
        let Some(machine) = self.machine.as_mut() else {
            return false;
        };
        let before_state = machine.state();
        let before_watcher = machine.watcher_health();
        let before_hot_index_ready = machine.hot_index_ready();
        machine.update_readiness_gates(
            watcher_health,
            hot_index_ready,
            None,
            mono_timestamp_now(),
            None,
        );
        let changed = machine.state() != before_state
            || machine.watcher_health() != before_watcher
            || machine.hot_index_ready() != before_hot_index_ready;
        self.flush_logs_if_changed();
        changed
    }

    fn flush_logs_if_changed(&mut self) {
        let Some(machine) = self.machine.as_mut() else {
            return;
        };
        let snapshot = machine.snapshot();
        let key = WorkspaceLifecycleLogKey {
            workspace_id: snapshot.workspace_id.clone(),
            lifecycle_state: snapshot.lifecycle_state,
            trust_state: snapshot.trust_state,
            watcher_health: machine.watcher_health(),
            hot_index_ready: snapshot.hot_index_ready,
            command_graph_ready: snapshot.command_graph_ready,
            last_transition_reason_code: snapshot.last_transition_reason_code.clone(),
        };
        if self.last_logged.as_ref() == Some(&key) {
            return;
        }
        self.last_logged = Some(key);

        if let Some(parent) = self.snapshot_path.parent() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                self.last_error = Some(format!("workspace lifecycle log dir create failed: {err}"));
                return;
            }
        }

        match serde_json::to_string_pretty(&snapshot) {
            Ok(payload) => {
                if let Err(err) = std::fs::write(&self.snapshot_path, payload) {
                    self.last_error =
                        Some(format!("workspace lifecycle snapshot write failed: {err}"));
                }
            }
            Err(err) => {
                self.last_error = Some(format!(
                    "workspace lifecycle snapshot serialize failed: {err}"
                ));
            }
        }

        let frames = machine.drain_transition_frames();
        if !frames.is_empty() {
            let mut out = String::new();
            for frame in frames {
                match serde_json::to_string(&frame) {
                    Ok(line) => {
                        out.push_str(&line);
                        out.push('\n');
                    }
                    Err(err) => {
                        self.last_error = Some(format!(
                            "workspace lifecycle transition serialize failed: {err}"
                        ));
                        return;
                    }
                }
            }
            if let Err(err) = std::fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&self.transitions_path)
                .and_then(|mut file| std::io::Write::write_all(&mut file, out.as_bytes()))
            {
                self.last_error = Some(format!(
                    "workspace lifecycle transitions append failed: {err}"
                ));
            }
        }
    }

    fn status_badge_token(&self) -> Option<&'static str> {
        Some(self.machine.as_ref()?.state().as_str())
    }

    fn watcher_health_token(&self) -> Option<&'static str> {
        Some(self.machine.as_ref()?.watcher_health()?.as_str())
    }

    fn hot_index_ready(&self) -> Option<bool> {
        Some(self.machine.as_ref()?.hot_index_ready())
    }
}

#[derive(Debug, Clone)]
struct RecentWorkRuntimeState {
    store_path: PathBuf,
    registry: RecentWorkRegistry,
    active_recent_work_id: Option<String>,
    active_workspace_label: Option<String>,
    suspended_frames: HashMap<String, DesktopFrame>,
    last_error: Option<String>,
}

impl RecentWorkRuntimeState {
    fn load() -> Self {
        let store_path = RecentWorkRegistry::default_store_path();
        match RecentWorkRegistry::load_or_default(&store_path) {
            Ok(mut registry) => {
                for entry in &mut registry.entries {
                    normalize_recent_work_entry_recovery_actions(entry);
                }
                Self {
                    store_path,
                    registry,
                    active_recent_work_id: None,
                    active_workspace_label: None,
                    suspended_frames: HashMap::new(),
                    last_error: None,
                }
            }
            Err(err) => Self {
                store_path,
                registry: RecentWorkRegistry {
                    record_kind: RecentWorkRegistryRecordKind::RecentWorkRegistryRecord,
                    recent_work_registry_schema_version: 1,
                    updated_at: "mono:0000:00:00:00.0000".to_string(),
                    entries: Vec::new(),
                },
                active_recent_work_id: None,
                active_workspace_label: None,
                suspended_frames: HashMap::new(),
                last_error: Some(err.to_string()),
            },
        }
    }

    fn active_workspace_label(&self) -> Option<&str> {
        self.active_workspace_label.as_deref()
    }

    fn find_entry(&self, recent_work_id: &str) -> Option<&RecentWorkEntryRecord> {
        self.registry
            .entries
            .iter()
            .find(|row| row.recent_work_id == recent_work_id)
    }

    fn save(&self) -> Result<(), RecentWorkRegistryError> {
        self.registry.save(&self.store_path)
    }

    fn note_local_folder_opened(&mut self, opened_path: &Path, trust_state: TrustState) {
        let label = opened_path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| opened_path.display().to_string());
        let subtitle = opened_path.display().to_string();
        let identity_key = opened_path.to_string_lossy();
        let recent_work_id = recent_work_id_for(TargetKind::LocalFolder, &identity_key);
        let opened_at = mono_timestamp_now();

        let entry = RecentWorkEntryRecord {
            record_kind: RecentWorkEntryRecordKind::RecentWorkEntryRecord,
            entry_and_restore_schema_version: 1,
            recent_work_id: recent_work_id.clone(),
            presentation_label: label.clone(),
            presentation_subtitle: Some(subtitle),
            target_kind: TargetKind::LocalFolder,
            target_state: RecentWorkTargetState::Reachable,
            portability_class: PortabilityClass::LocalOnly,
            trust_state,
            restore_availability: RestoreAvailability::None,
            safe_recovery_actions: vec![
                SafeRecoveryAction::Open,
                SafeRecoveryAction::Pin,
                SafeRecoveryAction::RemoveFromRecents,
                SafeRecoveryAction::RevealInExplorer,
            ],
            pinned: false,
            last_opened_at: opened_at.clone(),
            filesystem_identity_ref: None,
            remote_target_descriptor_ref: None,
            artifact_descriptor_ref: None,
            recovery_checkpoint_refs: None,
        };
        let mut entry = entry;
        normalize_recent_work_entry_recovery_actions(&mut entry);

        self.registry.updated_at = opened_at;
        self.registry.upsert(entry);
        self.active_recent_work_id = Some(recent_work_id);
        self.active_workspace_label = Some(label);
    }

    fn suspend_active_frame(&mut self, frame: &DesktopFrame) {
        let Some(active_id) = self.active_recent_work_id.as_deref() else {
            return;
        };
        self.suspended_frames
            .insert(active_id.to_string(), frame.clone());
    }

    fn activate_recent_work(
        &mut self,
        frame: &mut DesktopFrame,
        window_size: (u32, u32),
        entry: &RecentWorkEntryRecord,
    ) {
        if let Some(snapshot) = self.suspended_frames.get(&entry.recent_work_id).cloned() {
            *frame = snapshot;
            frame.relayout(window_size.0, window_size.1);
        } else {
            *frame = DesktopFrame::new(window_size.0, window_size.1);
        }
        self.active_recent_work_id = Some(entry.recent_work_id.clone());
        self.active_workspace_label = Some(entry.presentation_label.clone());
    }
}

const NOTIFICATION_TOAST_FADE: Duration = Duration::from_millis(260);
const NOTIFICATION_TOAST_MAX_VISIBLE: usize = 4;
const NOTIFICATION_BANNER_MAX_VISIBLE: usize = 3;

#[derive(Debug, Clone)]
struct NotificationSurfaceRuntimeState {
    router: NotificationRouter,
    quiet_hours: QuietHoursPosture,
    toasts: Vec<LiveToastNotification>,
    banners: Vec<LiveBannerNotification>,
    last_error: Option<String>,
}

#[derive(Debug, Clone)]
struct LiveToastNotification {
    row: NotificationSurfaceRow,
    opened_at: Instant,
    expires_at: Instant,
}

#[derive(Debug, Clone)]
struct LiveBannerNotification {
    row: NotificationSurfaceRow,
}

impl NotificationSurfaceRuntimeState {
    fn from_env() -> Self {
        Self::new(quiet_hours_posture_from_env())
    }

    fn new(quiet_hours: QuietHoursPosture) -> Self {
        Self {
            router: NotificationRouter::new(),
            quiet_hours,
            toasts: Vec::new(),
            banners: Vec::new(),
            last_error: None,
        }
    }

    fn route(
        &mut self,
        mut envelope: NotificationEnvelope,
        now: Instant,
    ) -> Result<RoutedNotification, NotificationRoutingError> {
        self.quiet_hours.apply_to_envelope(&mut envelope);
        let routed = self.router.route(&envelope)?;
        self.apply_routed_surfaces(&routed, now);
        Ok(routed)
    }

    fn apply_routed_surfaces(&mut self, routed: &RoutedNotification, now: Instant) {
        if let Some(row) = NotificationSurfaceRow::project_toast(routed) {
            if surface_route_is_visible(routed, FanoutSurfaceClass::Toast) {
                self.upsert_toast(row, now);
            } else if surface_route_is_dedupe(routed, FanoutSurfaceClass::Toast) {
                self.refresh_toast_row(row);
            }
        }

        if let Some(row) = NotificationSurfaceRow::project_contextual_banner(routed) {
            if surface_route_is_visible(routed, FanoutSurfaceClass::ContextualBanner) {
                self.upsert_banner(row);
            } else if surface_route_is_dedupe(routed, FanoutSurfaceClass::ContextualBanner) {
                self.refresh_banner_row(row);
            }
        }
    }

    fn upsert_toast(&mut self, row: NotificationSurfaceRow, now: Instant) {
        let timeout = toast_timeout_for(row.severity_class);
        if let Some(existing) = self
            .toasts
            .iter_mut()
            .find(|toast| toast.row.canonical_event_id == row.canonical_event_id)
        {
            existing.row = row;
            existing.expires_at = now + timeout;
            return;
        }

        self.toasts.push(LiveToastNotification {
            row,
            opened_at: now,
            expires_at: now + timeout,
        });
        if self.toasts.len() > NOTIFICATION_TOAST_MAX_VISIBLE {
            let overflow = self.toasts.len() - NOTIFICATION_TOAST_MAX_VISIBLE;
            self.toasts.drain(0..overflow);
        }
    }

    fn refresh_toast_row(&mut self, row: NotificationSurfaceRow) {
        if let Some(existing) = self
            .toasts
            .iter_mut()
            .find(|toast| toast.row.canonical_event_id == row.canonical_event_id)
        {
            existing.row = row;
        }
    }

    fn upsert_banner(&mut self, row: NotificationSurfaceRow) {
        if let Some(existing) = self
            .banners
            .iter_mut()
            .find(|banner| banner.row.canonical_event_id == row.canonical_event_id)
        {
            existing.row = row;
            return;
        }

        self.banners.push(LiveBannerNotification { row });
        if self.banners.len() > NOTIFICATION_BANNER_MAX_VISIBLE {
            let overflow = self.banners.len() - NOTIFICATION_BANNER_MAX_VISIBLE;
            self.banners.drain(0..overflow);
        }
    }

    fn refresh_banner_row(&mut self, row: NotificationSurfaceRow) {
        if let Some(existing) = self
            .banners
            .iter_mut()
            .find(|banner| banner.row.canonical_event_id == row.canonical_event_id)
        {
            existing.row = row;
        }
    }

    fn tick(&mut self, now: Instant) -> bool {
        let before = self.toasts.len();
        self.toasts.retain(|toast| now < toast.expires_at);
        before != self.toasts.len()
    }

    fn has_active_toasts(&self) -> bool {
        !self.toasts.is_empty()
    }

    fn dismiss_at(&mut self, frame: &DesktopFrame, scale_factor: f64, x: u32, y: u32) -> bool {
        if let Some(index) =
            notification_toast_hit_index(frame, scale_factor, self.toasts.len(), x, y)
        {
            if index < self.toasts.len() {
                self.toasts.remove(index);
                return true;
            }
        }

        if notification_banner_rect(frame, scale_factor)
            .is_some_and(|rect| point_in_rect(x, y, rect))
            && !self.banners.is_empty()
        {
            self.banners.pop();
            return true;
        }

        false
    }
}

fn quiet_hours_posture_from_env() -> QuietHoursPosture {
    let Ok(value) = env::var("AURELINE_QUIET_HOURS") else {
        return QuietHoursPosture::none();
    };
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" | "quiet" | "quiet_hours" => {
            QuietHoursPosture::quiet_hours_user()
        }
        "dnd" | "do_not_disturb" => QuietHoursPosture::do_not_disturb(),
        "focus" | "focus_mode" => QuietHoursPosture::focus_mode(),
        "presentation" | "presenting" => QuietHoursPosture::presentation(),
        "privacy" | "privacy_mode" => QuietHoursPosture::privacy_mode(),
        "admin" | "admin_suppression" => QuietHoursPosture::admin_suppression(),
        _ => QuietHoursPosture::none(),
    }
}

fn surface_route_is_visible(routed: &RoutedNotification, surface: FanoutSurfaceClass) -> bool {
    routed
        .surface_routes
        .iter()
        .find(|route| route.fanout_surface_class == surface)
        .is_some_and(|route| route.is_visible())
}

fn surface_route_is_dedupe(routed: &RoutedNotification, surface: FanoutSurfaceClass) -> bool {
    routed
        .surface_routes
        .iter()
        .find(|route| route.fanout_surface_class == surface)
        .is_some_and(|route| {
            matches!(
                route.receipt_state,
                FanoutReceiptState::DedupedCanonicalEvent | FanoutReceiptState::DedupedGroupedBurst
            )
        })
}

fn toast_timeout_for(severity: SeverityClass) -> Duration {
    match severity {
        SeverityClass::Success => Duration::from_millis(3200),
        SeverityClass::Info => Duration::from_millis(4200),
        SeverityClass::Warning | SeverityClass::Degraded => Duration::from_millis(6200),
        SeverityClass::Error | SeverityClass::Blocking | SeverityClass::Critical => {
            Duration::from_millis(8200)
        }
    }
}

fn toast_alpha(toast: &LiveToastNotification, now: Instant) -> f32 {
    if now >= toast.expires_at {
        return 0.0;
    }
    let fade_starts = toast
        .expires_at
        .checked_sub(NOTIFICATION_TOAST_FADE)
        .unwrap_or(toast.opened_at);
    if now < fade_starts {
        return 1.0;
    }
    let remaining = toast.expires_at.saturating_duration_since(now);
    clamp_unit(remaining.as_secs_f32() / NOTIFICATION_TOAST_FADE.as_secs_f32())
}

#[derive(Debug, Clone)]
struct ActivityCenterRuntimeState {
    store_path: PathBuf,
    runtime: ActivityCenterRuntime,
    notifications: NotificationSurfaceRuntimeState,
    file_index_ready_by_key: HashMap<String, bool>,
    last_error: Option<String>,
}

impl ActivityCenterRuntimeState {
    fn load(recent_work_store_path: &Path) -> Self {
        let root = recent_work_store_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_else(|| PathBuf::from(".logs").join("recent_work"));
        let store_path = root.join("activity_center_rows.json");
        match ActivityCenterRuntime::file_backed(&store_path) {
            Ok(runtime) => Self {
                store_path,
                runtime,
                notifications: NotificationSurfaceRuntimeState::from_env(),
                file_index_ready_by_key: HashMap::new(),
                last_error: None,
            },
            Err(err) => Self {
                store_path,
                runtime: ActivityCenterRuntime::in_memory(),
                notifications: NotificationSurfaceRuntimeState::from_env(),
                file_index_ready_by_key: HashMap::new(),
                last_error: Some(format!("activity center load failed: {err}")),
            },
        }
    }

    #[cfg(test)]
    fn in_memory_with_quiet_hours(quiet_hours: QuietHoursPosture) -> Self {
        Self {
            store_path: PathBuf::from("activity_center_rows.json"),
            runtime: ActivityCenterRuntime::in_memory(),
            notifications: NotificationSurfaceRuntimeState::new(quiet_hours),
            file_index_ready_by_key: HashMap::new(),
            last_error: None,
        }
    }

    fn snapshot(&self) -> ActivityCenterSnapshot {
        self.runtime.snapshot()
    }

    fn tick_notifications(&mut self, now: Instant) -> bool {
        self.notifications.tick(now)
    }

    fn notifications_need_animation(&self) -> bool {
        self.notifications.has_active_toasts()
    }

    fn dismiss_notification_at(
        &mut self,
        frame: &DesktopFrame,
        scale_factor: f64,
        x: u32,
        y: u32,
    ) -> bool {
        self.notifications.dismiss_at(frame, scale_factor, x, y)
    }

    fn persist_clean_shutdown(&mut self) {
        if let Err(err) = self.runtime.persist_now() {
            self.last_error = Some(format!("activity center shutdown persist failed: {err}"));
        }
    }

    fn note_workspace_opened(
        &mut self,
        recent_work_id: Option<&str>,
        workspace_label: &str,
        path: &Path,
    ) {
        let identity = recent_work_id.map(str::to_owned).unwrap_or_else(|| {
            format!(
                "local-folder-{:016x}",
                fnv1a_64(&path.display().to_string())
            )
        });
        let object_target_ref = format!("obj:workspace:{identity}");
        let event_id = format!("ux:event:workspace-open:{identity}");
        let source_event_ref = format!("shell:workspace-open:{identity}");
        let detail = format!("Opened {}", path.display());

        self.record_job(
            ActivityJobRecord {
                event_id: &event_id,
                envelope_id: &format!("ux:notif-env:workspace-open:{identity}:running"),
                source_subsystem: SourceSubsystem::Shell,
                source_event_ref: &source_event_ref,
                object_target_ref: &object_target_ref,
                summary_label: &format!("Workspace opening: {workspace_label}"),
                severity_class: SeverityClass::Info,
                action_command_id: "cmd:activity.focus_origin",
                action_label: "Focus workspace",
                attention_surface: None,
                minted_at: now_rfc3339(),
            },
            DurableJobObservation::in_flight(ActivityRowLifecycleClass::Running, None),
        );
        self.record_job(
            ActivityJobRecord {
                event_id: &event_id,
                envelope_id: &format!("ux:notif-env:workspace-open:{identity}:completed"),
                source_subsystem: SourceSubsystem::Shell,
                source_event_ref: &source_event_ref,
                object_target_ref: &object_target_ref,
                summary_label: &format!("Workspace opened: {workspace_label}"),
                severity_class: SeverityClass::Success,
                action_command_id: "cmd:activity.focus_origin",
                action_label: "Focus workspace",
                attention_surface: Some(FanoutSurfaceClass::Toast),
                minted_at: now_rfc3339(),
            },
            DurableJobObservation::completed(detail, Some(object_target_ref.clone())),
        );
    }

    fn note_quick_open_file_index(
        &mut self,
        recent_work_id: Option<&str>,
        root: &Path,
        ready: bool,
    ) -> bool {
        let identity = recent_work_id
            .map(str::to_owned)
            .unwrap_or_else(|| format!("workspace-{:016x}", fnv1a_64(&root.display().to_string())));
        let key = format!("quick-open-index:{identity}");
        if self.file_index_ready_by_key.get(&key).copied() == Some(ready) {
            return false;
        }
        self.file_index_ready_by_key.insert(key, ready);

        let root_label = root
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| root.display().to_string());
        let event_id = format!("ux:event:quick-open-index:{identity}");
        let object_target_ref = format!("obj:quick-open-index:{identity}");
        let source_event_ref = format!("palette:file-index:{identity}");

        if ready {
            self.record_job(
                ActivityJobRecord {
                    event_id: &event_id,
                    envelope_id: &format!("ux:notif-env:quick-open-index:{identity}:completed"),
                    source_subsystem: SourceSubsystem::Indexer,
                    source_event_ref: &source_event_ref,
                    object_target_ref: &object_target_ref,
                    summary_label: &format!("Quick open index ready: {root_label}"),
                    severity_class: SeverityClass::Success,
                    action_command_id: "cmd:activity.focus_origin",
                    action_label: "Focus workspace",
                    attention_surface: None,
                    minted_at: now_rfc3339(),
                },
                DurableJobObservation::completed(
                    format!("File index completed for {}", root.display()),
                    Some(object_target_ref.clone()),
                ),
            );
        } else {
            self.record_job(
                ActivityJobRecord {
                    event_id: &event_id,
                    envelope_id: &format!("ux:notif-env:quick-open-index:{identity}:running"),
                    source_subsystem: SourceSubsystem::Indexer,
                    source_event_ref: &source_event_ref,
                    object_target_ref: &object_target_ref,
                    summary_label: &format!("Quick open index warming: {root_label}"),
                    severity_class: SeverityClass::Info,
                    action_command_id: "cmd:activity.focus_origin",
                    action_label: "Focus workspace",
                    attention_surface: None,
                    minted_at: now_rfc3339(),
                },
                DurableJobObservation::in_flight(ActivityRowLifecycleClass::Running, None),
            );
        }
        true
    }

    fn note_clone_running(
        &mut self,
        operation_id: &str,
        remote_url: &str,
        destination_path: &Path,
        progress_label: Option<String>,
    ) {
        let progress = progress_label.map(|label| ActivityRowProgress::new(label, 0, 0));
        self.record_clone_transition(
            operation_id,
            remote_url,
            destination_path,
            "running",
            SeverityClass::Info,
            DurableJobObservation::in_flight(ActivityRowLifecycleClass::Running, progress),
            None,
        );
    }

    fn note_clone_completed(
        &mut self,
        operation_id: &str,
        remote_url: &str,
        destination_path: &Path,
    ) {
        self.record_clone_transition(
            operation_id,
            remote_url,
            destination_path,
            "completed",
            SeverityClass::Success,
            DurableJobObservation::completed(
                format!("Cloned into {}", destination_path.display()),
                Some(format!("obj:clone:{operation_id}")),
            ),
            Some(FanoutSurfaceClass::Toast),
        );
    }

    fn note_clone_failed(
        &mut self,
        operation_id: &str,
        remote_url: &str,
        destination_path: &Path,
        error: &CloneError,
    ) {
        self.record_clone_transition(
            operation_id,
            remote_url,
            destination_path,
            "failed",
            SeverityClass::Error,
            DurableJobObservation::failed(
                ActivityRowRetryability::Available,
                format!("{}: {}", error.class.as_str(), error.message),
                Some(format!("obj:clone:{operation_id}")),
            ),
            Some(FanoutSurfaceClass::ContextualBanner),
        );
    }

    fn note_import_review_recorded(&mut self, review: &ImportReviewRecord) {
        let event_id = format!("ux:event:import-review:{}", review.import_review_id);
        let object_target_ref = format!("obj:import-review:{}", review.import_review_id);
        let source_event_ref = format!("workspace-import:{}", review.import_review_id);
        let summary_label = format!(
            "Import review recorded: {}",
            review.classification.display_label()
        );
        let detail = format!(
            "{} for {} into {}",
            review.discovered_item_count_label(),
            review.source_path,
            review.destination_workspace_target
        );
        let severity_class = match review.decision_class {
            ImportReviewDecisionClass::ApplyAfterPreview => SeverityClass::Success,
            ImportReviewDecisionClass::Decline | ImportReviewDecisionClass::Defer => {
                SeverityClass::Warning
            }
        };
        self.record_job(
            ActivityJobRecord {
                event_id: &event_id,
                envelope_id: &format!("ux:notif-env:import-review:{}", review.import_review_id),
                source_subsystem: SourceSubsystem::Shell,
                source_event_ref: &source_event_ref,
                object_target_ref: &object_target_ref,
                summary_label: &summary_label,
                severity_class,
                action_command_id: "cmd:workspace.import_profile",
                action_label: "Review import",
                attention_surface: Some(FanoutSurfaceClass::Toast),
                minted_at: now_rfc3339(),
            },
            DurableJobObservation::completed(detail, Some(object_target_ref.clone())),
        );
    }

    fn record_clone_transition(
        &mut self,
        operation_id: &str,
        _remote_url: &str,
        destination_path: &Path,
        phase: &str,
        severity_class: SeverityClass,
        observation: DurableJobObservation,
        attention_surface: Option<FanoutSurfaceClass>,
    ) {
        let destination_label = destination_path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_else(|| destination_path.display().to_string());
        let event_id = format!("ux:event:clone-repository:{operation_id}");
        let object_target_ref = format!("obj:clone:{operation_id}");
        let source_event_ref = format!("workspace-clone:{operation_id}");
        let summary_label = match phase {
            "completed" => format!("Clone completed: {destination_label}"),
            "failed" => format!("Clone failed: {destination_label}"),
            _ => format!("Clone running: {destination_label}"),
        };
        self.record_job(
            ActivityJobRecord {
                event_id: &event_id,
                envelope_id: &format!("ux:notif-env:clone-repository:{operation_id}:{phase}"),
                source_subsystem: SourceSubsystem::Shell,
                source_event_ref: &source_event_ref,
                object_target_ref: &object_target_ref,
                summary_label: &summary_label,
                severity_class,
                action_command_id: "cmd:activity.focus_origin",
                action_label: "Focus clone",
                attention_surface,
                minted_at: now_rfc3339(),
            },
            observation,
        );
    }

    fn next_save_operation_id(&self, label: &str) -> String {
        let token = now_rfc3339();
        format!(
            "save-{:016x}-{}",
            fnv1a_64(label),
            sanitize_activity_id_component(&token)
        )
    }

    fn note_save_running(&mut self, operation_id: &str, label: &str) {
        self.record_save_transition(
            operation_id,
            label,
            "running",
            SeverityClass::Info,
            DurableJobObservation::in_flight(ActivityRowLifecycleClass::Running, None),
        );
    }

    fn note_save_completed(
        &mut self,
        operation_id: &str,
        label: &str,
        evidence_ref: Option<String>,
    ) {
        self.record_save_transition(
            operation_id,
            label,
            "completed",
            SeverityClass::Success,
            DurableJobObservation::completed(format!("Saved {label}"), evidence_ref),
        );
    }

    fn note_save_failed(&mut self, operation_id: &str, label: &str, detail: impl Into<String>) {
        self.record_save_transition(
            operation_id,
            label,
            "failed",
            SeverityClass::Error,
            DurableJobObservation::failed(ActivityRowRetryability::Available, detail.into(), None),
        );
    }

    fn record_save_transition(
        &mut self,
        operation_id: &str,
        label: &str,
        phase: &str,
        severity_class: SeverityClass,
        observation: DurableJobObservation,
    ) {
        let event_id = format!("ux:event:save-fsync:{operation_id}");
        let object_target_ref = format!("obj:save-fsync:{operation_id}");
        let source_event_ref = format!("vfs-save:{operation_id}");
        let summary_label = match phase {
            "completed" => format!("Save completed: {label}"),
            "failed" => format!("Save failed: {label}"),
            _ => format!("Save fsync running: {label}"),
        };
        self.record_job(
            ActivityJobRecord {
                event_id: &event_id,
                envelope_id: &format!("ux:notif-env:save-fsync:{operation_id}:{phase}"),
                source_subsystem: SourceSubsystem::VfsSave,
                source_event_ref: &source_event_ref,
                object_target_ref: &object_target_ref,
                summary_label: &summary_label,
                severity_class,
                action_command_id: "cmd:activity.focus_origin",
                action_label: "Focus editor",
                attention_surface: match phase {
                    "completed" => Some(FanoutSurfaceClass::Toast),
                    "failed" => Some(FanoutSurfaceClass::ContextualBanner),
                    _ => None,
                },
                minted_at: now_rfc3339(),
            },
            observation,
        );
    }

    fn note_trust_state_changed(&mut self, previous: &str, current: &str) {
        let minted_at = now_rfc3339();
        let transition_id =
            sanitize_activity_id_component(&format!("{previous}-{current}-{minted_at}"));
        let event_id = format!("ux:event:workspace-trust:{transition_id}");
        let object_target_ref = "obj:workspace-trust:current";
        let source_event_ref = format!("workspace-trust:{transition_id}");
        let summary_label = format!("Workspace trust changed: {}", trust_state_label(current));
        let detail = format!(
            "Workspace trust changed from {} to {}.",
            trust_state_label(previous),
            trust_state_label(current)
        );
        self.record_job(
            ActivityJobRecord {
                event_id: &event_id,
                envelope_id: &format!("ux:notif-env:workspace-trust:{transition_id}"),
                source_subsystem: SourceSubsystem::WorkspaceTrust,
                source_event_ref: &source_event_ref,
                object_target_ref,
                summary_label: &summary_label,
                severity_class: if current == "trusted" {
                    SeverityClass::Info
                } else {
                    SeverityClass::Warning
                },
                action_command_id: "cmd:activity.focus_origin",
                action_label: "Open trust details",
                attention_surface: Some(FanoutSurfaceClass::ContextualBanner),
                minted_at,
            },
            DurableJobObservation::completed(detail, Some(object_target_ref.to_string())),
        );
    }

    fn note_settings_write_denied(&mut self, outcome: &WriteAttemptOutcome) {
        let Some(reason) = outcome.denial_reason.as_ref() else {
            return;
        };
        let reason_label = settings_denial_reason_label(reason);
        let setting_component = sanitize_activity_id_component(&outcome.setting_id);
        let event_id = format!("ux:event:settings-write-denied:{setting_component}");
        let object_target_ref = format!("obj:settings:{}", outcome.setting_id);
        let source_event_ref = format!("settings-write:{}", outcome.setting_id);
        let summary_label = format!(
            "Settings write denied: {} ({})",
            outcome.setting_id, reason_label
        );
        let detail = outcome
            .effective_after
            .as_ref()
            .map(|effective| {
                format!(
                    "{} remains {} from {} ({})",
                    effective.setting_id,
                    effective.value.preview(),
                    effective.winning_scope.as_str(),
                    effective.source_label
                )
            })
            .unwrap_or_else(|| "No effective setting value was changed.".to_string());

        self.record_job(
            ActivityJobRecord {
                event_id: &event_id,
                envelope_id: &format!("ux:notif-env:settings-write-denied:{setting_component}"),
                source_subsystem: SourceSubsystem::PolicyResolver,
                source_event_ref: &source_event_ref,
                object_target_ref: &object_target_ref,
                summary_label: &summary_label,
                severity_class: SeverityClass::Warning,
                action_command_id: "cmd:settings.open",
                action_label: "Open settings",
                attention_surface: Some(FanoutSurfaceClass::ContextualBanner),
                minted_at: now_rfc3339(),
            },
            DurableJobObservation::failed(ActivityRowRetryability::DeniedByContext, detail, None),
        );
    }

    fn record_job(&mut self, job: ActivityJobRecord<'_>, observation: DurableJobObservation) {
        let envelope = activity_job_envelope(job);
        match self.notifications.route(envelope, Instant::now()) {
            Ok(routed) => {
                if let Err(err) = self
                    .runtime
                    .record_routed_observation(&routed, &observation)
                {
                    self.last_error = Some(format!("activity center record failed: {err}"));
                }
            }
            Err(err) => {
                self.notifications.last_error = Some(format!("notification route failed: {err}"));
                self.last_error = Some(format!("notification route failed: {err}"));
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ActivityJobRecord<'a> {
    event_id: &'a str,
    envelope_id: &'a str,
    source_subsystem: SourceSubsystem,
    source_event_ref: &'a str,
    object_target_ref: &'a str,
    summary_label: &'a str,
    severity_class: SeverityClass,
    action_command_id: &'a str,
    action_label: &'a str,
    attention_surface: Option<FanoutSurfaceClass>,
    minted_at: String,
}

fn activity_job_envelope(job: ActivityJobRecord<'_>) -> NotificationEnvelope {
    let mut recommended_surfaces = vec![
        FanoutSurfaceClass::DurableJobRow,
        FanoutSurfaceClass::StatusItem,
    ];
    if let Some(surface) = job.attention_surface {
        if !recommended_surfaces.contains(&surface) {
            recommended_surfaces.push(surface);
        }
    }

    NotificationEnvelope {
        record_kind: "notification_envelope_record".to_string(),
        notification_envelope_schema_version: NOTIFICATION_ENVELOPE_SCHEMA_VERSION,
        notification_envelope_id: job.envelope_id.to_string(),
        canonical_event_id: job.event_id.to_string(),
        event_lineage_id_ref: format!("ux:lineage:{}", job.event_id),
        source_subsystem: job.source_subsystem,
        source_event_ref: job.source_event_ref.to_string(),
        actor_identity_ref: "id:actor:system:shell".to_string(),
        canonical_object_target_ref: job.object_target_ref.to_string(),
        severity_class: job.severity_class,
        privacy_class: PrivacyClass::WorkspaceSensitive,
        privacy_payload_class: PrivacyPayloadClass::LockScreenSafeGeneric,
        redaction_class: RedactionClass::OperatorOnlyRestricted,
        dedupe_key_scheme: DedupeKeyScheme::CanonicalEventId,
        dedupe_key_ref: job.event_id.to_string(),
        grouped_burst_id_ref: None,
        recommended_surfaces,
        summary_label: job.summary_label.to_string(),
        reopen_target: ReopenTarget {
            reopen_target_ref: format!("ux:reopen:{}", job.event_id),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            exact_target_identity_ref: Some(job.object_target_ref.to_string()),
            placeholder_announcement_label: None,
            revalidation_required_reason_label: None,
        },
        actions: vec![StableAction {
            action_id: format!("ux:action:open:{}", job.event_id),
            label: job.action_label.to_string(),
            command_id: job.action_command_id.to_string(),
            target_identity_ref: job.object_target_ref.to_string(),
            reopen_target_kind: ReopenTargetKind::DurableActivityRow,
            is_destructive: false,
        }],
        suppression_state: SuppressionState {
            active_modes_at_mint: vec![QuietHoursMode::ModeNone],
            suppression_reasons: vec![],
            suppressed: false,
        },
        fanout_receipts: vec![],
        minted_at: job.minted_at,
    }
}

fn sanitize_activity_id_component(value: &str) -> String {
    value
        .chars()
        .map(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' => ch.to_ascii_lowercase(),
            _ => '-',
        })
        .collect()
}

fn expand_tilde(value: &str) -> PathBuf {
    if value == "~" {
        return env::var_os("HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from(value));
    }
    if let Some(rest) = value.strip_prefix("~/") {
        if let Some(home) = env::var_os("HOME") {
            return PathBuf::from(home).join(rest);
        }
    }
    PathBuf::from(value)
}

const UI_THEME_SETTING_ID: &str = "ui.theme";
const UI_DENSITY_SETTING_ID: &str = "ui.density";
const UI_MOTION_SETTING_ID: &str = "ui.motion";
const UI_THEME_LIGHT: &str = "light";
const UI_THEME_DARK: &str = "dark";
const UI_THEME_SYSTEM: &str = "system";
const UI_DENSITY_COMPACT: &str = "compact";
const UI_DENSITY_COMFORTABLE: &str = "comfortable";
const UI_DENSITY_SPACIOUS: &str = "spacious";
const UI_MOTION_FULL: &str = "full";
const UI_MOTION_REDUCED: &str = "reduced";
const UI_MOTION_NONE: &str = "none";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThemeBaseMode {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
struct AppearanceRuntimeState {
    store_path: PathBuf,
    policy_path: PathBuf,
    settings_path: PathBuf,
    session: AppearanceSessionRecord,
    policy: LiveFollowSystemPolicyRecord,
    resolver: EffectiveSettingsResolver,
    last_error: Option<String>,
}

impl AppearanceRuntimeState {
    fn load() -> Self {
        let state_root = appearance_state_root();
        let store_path = state_root
            .join("appearance")
            .join("appearance_session.json");
        let policy_path = state_root
            .join("appearance")
            .join("live_follow_system_policy.json");
        let settings_path = state_root
            .join("settings")
            .join("appearance_effective_settings.json");

        let mut last_error: Option<String> = None;
        let mut needs_persist = false;

        let session = match std::fs::read_to_string(&store_path) {
            Ok(payload) => serde_json::from_str(&payload).unwrap_or_else(|err| {
                last_error = Some(format!("appearance session parse failed: {err}"));
                needs_persist = true;
                AppearanceSessionRecord::first_party_default(now_rfc3339())
            }),
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    last_error = Some(format!("appearance session read failed: {err}"));
                } else {
                    needs_persist = true;
                }
                AppearanceSessionRecord::first_party_default(now_rfc3339())
            }
        };

        let policy = match std::fs::read_to_string(&policy_path) {
            Ok(payload) => serde_json::from_str(&payload).unwrap_or_else(|err| {
                last_error = Some(format!("appearance policy parse failed: {err}"));
                needs_persist = true;
                LiveFollowSystemPolicyRecord::first_party_default(
                    session.appearance_session_id.clone(),
                    now_rfc3339(),
                )
            }),
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    last_error = Some(format!("appearance policy read failed: {err}"));
                } else {
                    needs_persist = true;
                }
                LiveFollowSystemPolicyRecord::first_party_default(
                    session.appearance_session_id.clone(),
                    now_rfc3339(),
                )
            }
        };

        let mut resolver = appearance_settings_resolver_with_defaults();
        let settings_loaded = match std::fs::read_to_string(&settings_path) {
            Ok(payload) => match serde_json::from_str::<serde_json::Value>(&payload) {
                Ok(value) => match resolver.import_state(&value) {
                    Ok(()) => true,
                    Err(err) => {
                        last_error = Some(format!("appearance settings import failed: {err}"));
                        needs_persist = true;
                        false
                    }
                },
                Err(err) => {
                    last_error = Some(format!("appearance settings parse failed: {err}"));
                    needs_persist = true;
                    false
                }
            },
            Err(err) => {
                if err.kind() != std::io::ErrorKind::NotFound {
                    last_error = Some(format!("appearance settings read failed: {err}"));
                } else {
                    needs_persist = true;
                }
                false
            }
        };

        if !settings_loaded {
            install_session_overlay_if_needed(&mut resolver, &session);
        }

        let mut state = Self {
            store_path,
            policy_path,
            settings_path,
            session,
            policy,
            resolver,
            last_error,
        };
        state.sync_session_from_effective_settings();

        if needs_persist {
            state.persist();
        }

        state
    }

    fn theme_class(&self) -> ThemeClass {
        let theme = self.ui_theme_token();
        theme_class_for_settings(
            theme.as_str(),
            self.session.contrast_mode,
            self.session.theme_class(),
        )
    }

    fn density_class(&self) -> DensityClass {
        let density = self.ui_density_token();
        density_class_for_settings(density.as_str())
    }

    fn reduced_motion_posture(&self) -> AccessibilityPostureClass {
        let motion = self.ui_motion_token();
        motion_posture_for_settings(motion.as_str())
    }

    fn apply_cli_overrides(
        &mut self,
        theme_class: Option<ThemeClass>,
        density_class: Option<DensityClass>,
        reduced_motion_posture: Option<AccessibilityPostureClass>,
        ui_theme: Option<&str>,
        ui_density: Option<&str>,
        ui_motion: Option<&str>,
    ) {
        if theme_class.is_none()
            && density_class.is_none()
            && reduced_motion_posture.is_none()
            && ui_theme.is_none()
            && ui_density.is_none()
            && ui_motion.is_none()
        {
            return;
        }

        let minted_at = now_rfc3339();
        if let Some(theme) = theme_class {
            self.session.apply_theme_class(theme, minted_at.clone());
            let _ = self.write_ui_setting(
                UI_THEME_SETTING_ID,
                ui_theme_for_theme_class(theme),
                SettingScope::SessionOverride,
            );
        }
        if let Some(density) = density_class {
            self.session.apply_density_class(density, minted_at.clone());
            let _ = self.write_ui_setting(
                UI_DENSITY_SETTING_ID,
                ui_density_for_density_class(density),
                SettingScope::SessionOverride,
            );
        }
        if let Some(posture) = reduced_motion_posture {
            self.session.apply_reduced_motion_posture(
                posture,
                ReducedMotionSource::UserSetting,
                minted_at,
            );
            let _ = self.write_ui_setting(
                UI_MOTION_SETTING_ID,
                ui_motion_for_posture(posture),
                SettingScope::SessionOverride,
            );
        }
        if let Some(theme) = ui_theme {
            let _ =
                self.write_ui_setting(UI_THEME_SETTING_ID, theme, SettingScope::SessionOverride);
        }
        if let Some(density) = ui_density {
            let _ = self.write_ui_setting(
                UI_DENSITY_SETTING_ID,
                density,
                SettingScope::SessionOverride,
            );
        }
        if let Some(motion) = ui_motion {
            let _ =
                self.write_ui_setting(UI_MOTION_SETTING_ID, motion, SettingScope::SessionOverride);
        }
    }

    fn toggle_light_dark(&mut self) -> WriteAttemptOutcome {
        let theme = self.ui_theme_token();
        let next = match theme.as_str() {
            UI_THEME_LIGHT => UI_THEME_DARK,
            _ => UI_THEME_LIGHT,
        };
        self.write_ui_setting(UI_THEME_SETTING_ID, next, SettingScope::UserGlobal)
    }

    fn toggle_high_contrast(&mut self) {
        self.sync_session_from_effective_settings();
        let minted_at = now_rfc3339();
        self.session.toggle_high_contrast(minted_at);
        self.persist();
    }

    fn cycle_density_class(&mut self) -> WriteAttemptOutcome {
        let density = self.ui_density_token();
        let next = match density.as_str() {
            UI_DENSITY_COMPACT => UI_DENSITY_COMFORTABLE,
            UI_DENSITY_COMFORTABLE => UI_DENSITY_SPACIOUS,
            _ => UI_DENSITY_COMPACT,
        };
        self.write_ui_setting(UI_DENSITY_SETTING_ID, next, SettingScope::UserGlobal)
    }

    fn cycle_reduced_motion_posture(&mut self) -> WriteAttemptOutcome {
        let motion = self.ui_motion_token();
        let next = match motion.as_str() {
            UI_MOTION_FULL => UI_MOTION_REDUCED,
            UI_MOTION_REDUCED => UI_MOTION_NONE,
            _ => UI_MOTION_FULL,
        };
        self.write_ui_setting(UI_MOTION_SETTING_ID, next, SettingScope::UserGlobal)
    }

    fn write_ui_setting(
        &mut self,
        setting_id: &str,
        value: &str,
        target_scope: SettingScope,
    ) -> WriteAttemptOutcome {
        let outcome = self.resolver.attempt_write(
            setting_id,
            target_scope,
            SettingValue::String(value.to_string()),
        );
        if outcome.verdict == WriteIntent::Allowed {
            self.sync_session_from_effective_settings();
            self.persist();
        }
        outcome
    }

    fn resolved_ui_setting(&self, setting_id: &str) -> Option<EffectiveValue> {
        self.resolver.resolve(setting_id).ok()
    }

    fn settings_rows(&self) -> Vec<SettingsOverlayRow> {
        [
            (UI_THEME_SETTING_ID, "Theme"),
            (UI_DENSITY_SETTING_ID, "Density"),
            (UI_MOTION_SETTING_ID, "Motion"),
        ]
        .into_iter()
        .filter_map(|(setting_id, label)| {
            let effective = self.resolved_ui_setting(setting_id)?;
            Some(SettingsOverlayRow::from_effective(label, effective))
        })
        .collect()
    }

    #[cfg(test)]
    fn lock_ui_setting_for_test(&mut self, setting_id: &str, value: &str) {
        let mut policy = self
            .resolver
            .overlays()
            .find(|overlay| overlay.scope == SettingScope::AdminPolicyNarrowing)
            .cloned()
            .unwrap_or_else(|| {
                ScopeOverlay::new(
                    SettingScope::AdminPolicyNarrowing,
                    "Test managed settings policy",
                )
            });
        policy.set_policy_constraint(
            setting_id,
            aureline_settings::PolicyConstraint::SingleValue {
                value: SettingValue::String(value.to_string()),
            },
        );
        self.resolver
            .set_overlay(policy)
            .expect("test policy lock must validate");
        self.sync_session_from_effective_settings();
    }

    fn ui_theme_token(&self) -> String {
        self.resolved_string_or_default(UI_THEME_SETTING_ID, UI_THEME_DARK)
    }

    fn ui_density_token(&self) -> String {
        self.resolved_string_or_default(UI_DENSITY_SETTING_ID, UI_DENSITY_COMFORTABLE)
    }

    fn ui_motion_token(&self) -> String {
        self.resolved_string_or_default(UI_MOTION_SETTING_ID, UI_MOTION_FULL)
    }

    fn resolved_string_or_default(&self, setting_id: &str, fallback: &str) -> String {
        self.resolver
            .resolve(setting_id)
            .ok()
            .and_then(|effective| match effective.value {
                SettingValue::String(value) => Some(value),
                _ => None,
            })
            .unwrap_or_else(|| fallback.to_string())
    }

    fn sync_session_from_effective_settings(&mut self) {
        let minted_at = now_rfc3339();
        self.session
            .apply_theme_class(self.theme_class(), minted_at.clone());
        self.session
            .apply_density_class(self.density_class(), minted_at.clone());
        self.session.apply_reduced_motion_posture(
            self.reduced_motion_posture(),
            ReducedMotionSource::UserSetting,
            minted_at,
        );
    }

    fn persist(&mut self) {
        if let Some(parent) = self.store_path.parent() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                self.last_error = Some(format!("appearance session dir create failed: {err}"));
                return;
            }
        }

        match serde_json::to_string_pretty(&self.session) {
            Ok(payload) => {
                if let Err(err) = std::fs::write(&self.store_path, payload) {
                    self.last_error = Some(format!("appearance session write failed: {err}"));
                }
            }
            Err(err) => {
                self.last_error = Some(format!("appearance session serialize failed: {err}"));
            }
        }

        match serde_json::to_string_pretty(&self.policy) {
            Ok(payload) => {
                if let Err(err) = std::fs::write(&self.policy_path, payload) {
                    self.last_error = Some(format!("appearance policy write failed: {err}"));
                }
            }
            Err(err) => {
                self.last_error = Some(format!("appearance policy serialize failed: {err}"));
            }
        }

        if let Some(parent) = self.settings_path.parent() {
            if let Err(err) = std::fs::create_dir_all(parent) {
                self.last_error = Some(format!("appearance settings dir create failed: {err}"));
                return;
            }
        }

        match serde_json::to_string_pretty(&self.resolver.export_state()) {
            Ok(payload) => {
                if let Err(err) = std::fs::write(&self.settings_path, payload) {
                    self.last_error = Some(format!("appearance settings write failed: {err}"));
                }
            }
            Err(err) => {
                self.last_error = Some(format!("appearance settings serialize failed: {err}"));
            }
        }
    }
}

fn appearance_state_root() -> PathBuf {
    env::var_os("AURELINE_APPEARANCE_STATE_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(".logs"))
}

fn appearance_settings_resolver_with_defaults() -> EffectiveSettingsResolver {
    let mut resolver = EffectiveSettingsResolver::new(SchemaRegistry::with_seed_catalog());
    let mut defaults = ScopeOverlay::new(SettingScope::BuiltInDefault, "Aureline shell defaults");
    defaults.set_value(
        UI_THEME_SETTING_ID,
        SettingValue::String(UI_THEME_DARK.to_string()),
    );
    defaults.set_value(
        UI_DENSITY_SETTING_ID,
        SettingValue::String(UI_DENSITY_COMFORTABLE.to_string()),
    );
    defaults.set_value(
        UI_MOTION_SETTING_ID,
        SettingValue::String(UI_MOTION_FULL.to_string()),
    );
    resolver
        .set_overlay(defaults)
        .expect("appearance default settings must validate");
    resolver
}

fn install_session_overlay_if_needed(
    resolver: &mut EffectiveSettingsResolver,
    session: &AppearanceSessionRecord,
) {
    let theme = ui_theme_for_theme_class(session.theme_class());
    let density = ui_density_for_density_class(session.density_class());
    let motion = ui_motion_for_posture(session.reduced_motion_posture);
    if theme == UI_THEME_DARK && density == UI_DENSITY_COMFORTABLE && motion == UI_MOTION_FULL {
        return;
    }

    let mut user = ScopeOverlay::new(SettingScope::UserGlobal, "Appearance session");
    user.set_value(UI_THEME_SETTING_ID, SettingValue::String(theme.to_string()));
    user.set_value(
        UI_DENSITY_SETTING_ID,
        SettingValue::String(density.to_string()),
    );
    user.set_value(
        UI_MOTION_SETTING_ID,
        SettingValue::String(motion.to_string()),
    );
    resolver
        .set_overlay(user)
        .expect("appearance session settings must validate");
}

fn ui_theme_for_theme_class(theme: ThemeClass) -> &'static str {
    match theme {
        ThemeClass::LightParity | ThemeClass::HighContrastLight => UI_THEME_LIGHT,
        ThemeClass::DarkReference | ThemeClass::HighContrastDark => UI_THEME_DARK,
    }
}

fn ui_density_for_density_class(density: DensityClass) -> &'static str {
    match density {
        DensityClass::Compact => UI_DENSITY_COMPACT,
        DensityClass::Standard => UI_DENSITY_COMFORTABLE,
        DensityClass::Comfortable => UI_DENSITY_SPACIOUS,
    }
}

fn ui_motion_for_posture(posture: AccessibilityPostureClass) -> &'static str {
    match posture {
        AccessibilityPostureClass::MotionStandard => UI_MOTION_FULL,
        AccessibilityPostureClass::MotionReduced => UI_MOTION_REDUCED,
        AccessibilityPostureClass::MotionLowMotion
        | AccessibilityPostureClass::MotionPowerSaver
        | AccessibilityPostureClass::MotionCriticalHotPath => UI_MOTION_NONE,
    }
}

fn theme_class_for_settings(
    theme: &str,
    contrast_mode: ContrastMode,
    previous: ThemeClass,
) -> ThemeClass {
    let base = match theme {
        UI_THEME_LIGHT => ThemeBaseMode::Light,
        UI_THEME_SYSTEM => match previous {
            ThemeClass::LightParity | ThemeClass::HighContrastLight => ThemeBaseMode::Light,
            ThemeClass::DarkReference | ThemeClass::HighContrastDark => ThemeBaseMode::Dark,
        },
        _ => ThemeBaseMode::Dark,
    };
    let high_contrast = matches!(
        contrast_mode,
        ContrastMode::ContrastHigh | ContrastMode::ContrastForcedColors
    );
    match (base, high_contrast) {
        (ThemeBaseMode::Light, false) => ThemeClass::LightParity,
        (ThemeBaseMode::Dark, false) => ThemeClass::DarkReference,
        (ThemeBaseMode::Light, true) => ThemeClass::HighContrastLight,
        (ThemeBaseMode::Dark, true) => ThemeClass::HighContrastDark,
    }
}

fn density_class_for_settings(density: &str) -> DensityClass {
    match density {
        UI_DENSITY_COMPACT => DensityClass::Compact,
        UI_DENSITY_SPACIOUS => DensityClass::Comfortable,
        _ => DensityClass::Standard,
    }
}

fn motion_posture_for_settings(motion: &str) -> AccessibilityPostureClass {
    match motion {
        UI_MOTION_REDUCED => AccessibilityPostureClass::MotionReduced,
        UI_MOTION_NONE => AccessibilityPostureClass::MotionLowMotion,
        _ => AccessibilityPostureClass::MotionStandard,
    }
}

#[cfg(test)]
mod appearance_settings_runtime_tests {
    use super::*;

    fn appearance_state_for_test(name: &str) -> AppearanceRuntimeState {
        let root = std::env::temp_dir().join(format!(
            "aureline-appearance-settings-test-{}-{}",
            name,
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        ));
        let session = AppearanceSessionRecord::first_party_default(now_rfc3339());
        let policy = LiveFollowSystemPolicyRecord::first_party_default(
            session.appearance_session_id.clone(),
            now_rfc3339(),
        );
        AppearanceRuntimeState {
            store_path: root.join("appearance_session.json"),
            policy_path: root.join("live_follow_system_policy.json"),
            settings_path: root.join("appearance_effective_settings.json"),
            session,
            policy,
            resolver: appearance_settings_resolver_with_defaults(),
            last_error: None,
        }
    }

    #[test]
    fn appearance_reads_theme_from_effective_settings() {
        let mut appearance = appearance_state_for_test("theme");
        let outcome = appearance.write_ui_setting(
            UI_THEME_SETTING_ID,
            UI_THEME_LIGHT,
            SettingScope::UserGlobal,
        );

        assert_eq!(outcome.verdict, WriteIntent::Allowed);
        assert_eq!(appearance.theme_class(), ThemeClass::LightParity);
        let theme = appearance
            .resolved_ui_setting(UI_THEME_SETTING_ID)
            .expect("theme resolves");
        assert_eq!(
            theme.value,
            SettingValue::String(UI_THEME_LIGHT.to_string())
        );
        assert_eq!(theme.winning_scope, SettingScope::UserGlobal);
    }

    #[test]
    fn policy_locked_theme_write_is_denied_without_changing_value() {
        let mut appearance = appearance_state_for_test("policy");
        appearance.lock_ui_setting_for_test(UI_THEME_SETTING_ID, UI_THEME_DARK);

        let outcome = appearance.write_ui_setting(
            UI_THEME_SETTING_ID,
            UI_THEME_LIGHT,
            SettingScope::UserGlobal,
        );

        assert_eq!(outcome.verdict, WriteIntent::Denied);
        assert!(matches!(
            outcome.denial_reason,
            Some(WriteDenialReason::PolicyLocked)
        ));
        assert_eq!(appearance.theme_class(), ThemeClass::DarkReference);
        let effective = appearance
            .resolved_ui_setting(UI_THEME_SETTING_ID)
            .expect("theme resolves");
        assert_eq!(
            effective.value,
            SettingValue::String(UI_THEME_DARK.to_string())
        );
        assert_eq!(effective.lock_state.as_str(), "policy_locked");
    }

    #[test]
    fn settings_overlay_denial_surfaces_status_and_notification() {
        let mut appearance = appearance_state_for_test("overlay-denial");
        appearance.lock_ui_setting_for_test(UI_THEME_SETTING_ID, UI_THEME_DARK);
        let mut activity_center =
            ActivityCenterRuntimeState::in_memory_with_quiet_hours(QuietHoursPosture::none());
        let mut command_runtime = CommandRuntimeState::default();
        let frame = DesktopFrame::new(1280, 720);
        let mut overlay = Some(ShellOverlayState::settings(
            ShellZoneId::MainWorkspace,
            frame.focused_editor_group(),
            &appearance,
        ));

        apply_settings_overlay_decision(
            &mut appearance,
            &mut activity_center,
            &mut command_runtime,
            &mut overlay,
            SettingsOverlayDecision {
                setting_id: UI_THEME_SETTING_ID.to_string(),
                value: UI_THEME_LIGHT.to_string(),
            },
        );

        assert!(command_runtime
            .last_command_label
            .as_deref()
            .is_some_and(|line| line.contains("settings write denied")));
        assert!(!activity_center.snapshot().is_empty());
        let Some(ShellOverlayState {
            kind: ShellOverlayKind::Settings(settings),
            ..
        }) = overlay
        else {
            panic!("settings overlay should remain open");
        };
        assert!(settings
            .status_line
            .as_deref()
            .is_some_and(|line| line.contains("policy_locked")));
        assert!(settings.rows.iter().any(|row| {
            row.setting_id == UI_THEME_SETTING_ID && row.lock_state == "policy_locked"
        }));
    }
}

fn left_sidebar_damage_hint(frame: &DesktopFrame, scale_factor: f64) -> ShellDamageHint {
    frame
        .layout()
        .zone(ShellZoneId::LeftSidebar)
        .map(|rect| ShellDamageHint::Rect {
            layer: CompositionLayerId::TextAndDecoration,
            class: DamageClassId::TextReflowLocal,
            rect: to_physical_rect(rect, scale_factor),
        })
        .unwrap_or(ShellDamageHint::FullWindow)
}

fn explorer_row_index_at(frame: &DesktopFrame, scale_factor: f64, x: u32, y: u32) -> Option<usize> {
    let zone = to_physical_rect(frame.layout().zone(ShellZoneId::LeftSidebar)?, scale_factor);
    let inner = Rect::new(
        zone.x.saturating_add(EXPLORER_HIT_INSET_PX),
        zone.y.saturating_add(EXPLORER_HIT_INSET_PX),
        zone.width
            .saturating_sub(EXPLORER_HIT_INSET_PX.saturating_mul(2)),
        zone.height
            .saturating_sub(EXPLORER_HIT_INSET_PX.saturating_mul(2)),
    );
    if x < inner.x || x >= inner.right() || y < inner.y || y >= inner.bottom() {
        return None;
    }
    let rows_y = inner.y.saturating_add(EXPLORER_HEADER_HIT_HEIGHT_PX);
    if y < rows_y {
        return None;
    }
    Some(((y - rows_y) / EXPLORER_ROW_HIT_HEIGHT_PX) as usize)
}

fn status_bar_slot_at(
    frame: &DesktopFrame,
    scale_factor: f64,
    x: u32,
    y: u32,
) -> Option<&'static str> {
    let zone_rect = frame.layout().zone(ShellZoneId::StatusBar)?;
    frame
        .slot_rects_within_zone(ShellZoneId::StatusBar, zone_rect, 0)
        .into_iter()
        .find_map(|(slot_id, slot_rect)| {
            let physical = to_physical_rect(slot_rect, scale_factor);
            point_in_rect(x, y, physical).then_some(slot_id)
        })
}

fn handle_status_bar_click(
    frame: &mut DesktopFrame,
    overlay: &mut Option<ShellOverlayState>,
    command_runtime: &mut CommandRuntimeState,
    editor_runtime: &EditorWorkspaceRuntimeState,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &WorkspaceLifecycleRuntimeState,
    recent_work: &RecentWorkRuntimeState,
    activity_center: &ActivityCenterRuntimeState,
    title_context_bar: &TitleContextBarStateRecord,
    scale_factor: f64,
    x: u32,
    y: u32,
) -> bool {
    let Some(slot_id) = status_bar_slot_at(frame, scale_factor, x, y) else {
        return false;
    };
    let snapshot = status_bar_snapshot_for_shell(
        frame,
        editor_runtime,
        enablement_runtime,
        workspace_lifecycle,
        recent_work,
        activity_center,
        title_context_bar,
    );

    match status_bar_slot_activation(slot_id) {
        StatusBarSlotActivation::WorkspaceSwitcher => {
            *overlay = Some(ShellOverlayState::workspace_switcher(
                frame.focused_zone(),
                frame.focused_editor_group(),
                &recent_work.registry,
                enablement_runtime.workspace_trust_state.as_str(),
            ));
            frame.focus_zone(ShellZoneId::TransientOverlay);
            command_runtime.note_non_command_action("status target opened workspace switcher");
            true
        }
        StatusBarSlotActivation::TrustReview => {
            *overlay = Some(ShellOverlayState::status_bar_item_detail(
                frame.focused_zone(),
                frame.focused_editor_group(),
                status_bar_detail_lines(
                    "Workspace trust review",
                    &snapshot,
                    &[StatusBarItemKind::Trust, StatusBarItemKind::Profile],
                ),
            ));
            frame.focus_zone(ShellZoneId::TransientOverlay);
            command_runtime.note_non_command_action("status trust opened workspace trust review");
            true
        }
        StatusBarSlotActivation::WorkSummary => {
            frame.focus_zone(ShellZoneId::ActivityRail);
            command_runtime.note_non_command_action("status work summary opened activity center");
            true
        }
        StatusBarSlotActivation::SourceFidelity => {
            *overlay = Some(ShellOverlayState::status_bar_item_detail(
                frame.focused_zone(),
                frame.focused_editor_group(),
                status_bar_detail_lines(
                    "Source fidelity",
                    &snapshot,
                    &[StatusBarItemKind::Encoding],
                ),
            ));
            frame.focus_zone(ShellZoneId::TransientOverlay);
            command_runtime.note_non_command_action("status encoding opened source fidelity");
            true
        }
        StatusBarSlotActivation::RecoveryDetail => {
            let kinds: Vec<StatusBarItemKind> = snapshot
                .items
                .iter()
                .filter(|item| item.is_recovery_critical)
                .map(|item| item.item_kind)
                .collect();
            if kinds.is_empty() {
                frame.focus_zone(ShellZoneId::StatusBar);
                command_runtime.note_non_command_action("status recovery slot focused");
            } else {
                *overlay = Some(ShellOverlayState::status_bar_item_detail(
                    frame.focused_zone(),
                    frame.focused_editor_group(),
                    status_bar_detail_lines("Recovery status", &snapshot, &kinds),
                ));
                frame.focus_zone(ShellZoneId::TransientOverlay);
                command_runtime.note_non_command_action("status recovery opened detail");
            }
            true
        }
        StatusBarSlotActivation::Fallback => {
            frame.focus_zone(ShellZoneId::StatusBar);
            command_runtime.note_non_command_action(format!(
                "status fallback selected: {}",
                shell_slot_label(slot_id)
            ));
            true
        }
    }
}

fn open_selected_explorer_file(
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    frame: &mut DesktopFrame,
    command_runtime: &mut CommandRuntimeState,
    hot_path_metrics: &mut HotPathMetrics,
    clock: &WallClock,
) -> ShellDamageHint {
    let Some(path) = editor_runtime.explorer.selected_file_path() else {
        return ShellDamageHint::None;
    };
    let focused_group = frame.focused_editor_group();
    let tick = clock.now().0;
    if frame.active_tab_id(focused_group).is_some() {
        hot_path_metrics.note_file_switch_to_paint_requested(tick);
    } else {
        hot_path_metrics.note_file_open_to_paint_requested(tick);
    }
    let Some(tab) = frame.open_tab_in_group(focused_group) else {
        return ShellDamageHint::None;
    };
    match editor_runtime.open_file(focused_group, tab, &path) {
        Ok(()) => {
            command_runtime
                .note_non_command_action(format!("opened file from explorer — {}", path.display()));
            ShellDamageHint::FullWindow
        }
        Err(err) => {
            hot_path_metrics
                .close_latest_span_as_error(clock.now().0, format!("open file failed — {err}"));
            command_runtime.note_non_command_action(format!("open file failed — {err}"));
            let _ = frame.close_active_tab(focused_group);
            editor_runtime.close_tab(focused_group, tab);
            ShellDamageHint::FullWindow
        }
    }
}

fn fnv1a_64(value: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in value.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn recent_work_id_for(target_kind: TargetKind, identity_key: &str) -> String {
    let hash = fnv1a_64(identity_key);
    format!("recent:{}:{:016x}", target_kind.as_str(), hash)
}

fn workspace_id_for_local_folder(identity_key: &str) -> String {
    let hash = fnv1a_64(identity_key);
    format!("ws-local-{:016x}", hash)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocalFolderOpenSource {
    CliArgument,
    CloneRepository,
    FolderPicker,
}

impl LocalFolderOpenSource {
    const fn status_label(self) -> &'static str {
        match self {
            Self::CliArgument => "opened folder from CLI",
            Self::CloneRepository => "opened cloned repository",
            Self::FolderPicker => "opened folder",
        }
    }
}

fn open_local_folder_workspace(
    path: &Path,
    source: LocalFolderOpenSource,
    command_runtime: &mut CommandRuntimeState,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    palette: &mut CommandPaletteState,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
) {
    let group = frame.focused_editor_group();
    if let Some(tab) = frame.open_tab() {
        editor_runtime.open_placeholder(group, tab);
    }

    let trust_state =
        trust_state_for_recent_work(enablement_runtime.workspace_trust_state.as_str());
    recent_work.note_local_folder_opened(path, trust_state);
    if let Some(active_id) = recent_work.active_recent_work_id.clone() {
        editor_runtime.set_workspace_recovery_ref(active_id);
    }
    activity_center.note_workspace_opened(
        recent_work.active_recent_work_id.as_deref(),
        recent_work.active_workspace_label().unwrap_or("workspace"),
        path,
    );
    activity_center.note_quick_open_file_index(
        recent_work.active_recent_work_id.as_deref(),
        path,
        false,
    );
    palette.set_workspace_root(path.to_path_buf());
    let identity_key = path.to_string_lossy();
    let workspace_id = workspace_id_for_local_folder(&identity_key);
    if let Err(err) = editor_runtime
        .explorer
        .open_workspace(path.to_path_buf(), workspace_id.clone())
    {
        command_runtime.note_non_command_action(format!("explorer unavailable — {err}"));
    }
    workspace_lifecycle.open_local_folder(workspace_id.clone(), trust_state);
    editor_runtime.terminal_pane.open_workspace(
        workspace_id,
        path.to_path_buf(),
        trust_state,
        &mono_timestamp_now(),
    );
    if let Err(err) = recent_work.save() {
        command_runtime.note_non_command_action(format!("recent work save failed — {err}"));
    } else {
        command_runtime.note_non_command_action(format!(
            "{} — {}",
            source.status_label(),
            path.display()
        ));
    }
}

#[allow(clippy::too_many_arguments)]
fn poll_clone_jobs(
    clone_jobs: &mut CloneJobRuntimeState,
    command_runtime: &mut CommandRuntimeState,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    palette: &mut CommandPaletteState,
    overlay: &mut Option<ShellOverlayState>,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
) -> bool {
    let Some(job) = clone_jobs.active.as_mut() else {
        return false;
    };

    let mut changed = false;
    let mut completed: Option<Result<(), CloneError>> = None;
    while let Ok(message) = job.receiver.try_recv() {
        changed = true;
        match message {
            CloneWorkerMessage::Progress(event) => {
                let label = clone_progress_label(&event);
                activity_center.note_clone_running(
                    &job.operation_id,
                    &job.request.remote_url,
                    &job.request.destination_path,
                    Some(label.clone()),
                );
                set_clone_sheet_status(overlay, label.clone(), true);
                command_runtime.note_non_command_action(format!("clone progress - {label}"));
            }
            CloneWorkerMessage::Completed(result) => {
                completed = Some(result);
                break;
            }
        }
    }

    let Some(result) = completed else {
        return changed;
    };

    let job = clone_jobs
        .active
        .take()
        .expect("clone job exists after completed message");
    match result {
        Ok(()) => {
            activity_center.note_clone_completed(
                &job.operation_id,
                &job.request.remote_url,
                &job.request.destination_path,
            );
            command_runtime.record(invocation_and_result_clone_succeeded(
                &job.session,
                &job.request.destination_path,
            ));
            set_clone_sheet_status(overlay, "clone completed".to_string(), false);
            open_local_folder_workspace(
                &job.request.destination_path,
                LocalFolderOpenSource::CloneRepository,
                command_runtime,
                frame,
                editor_runtime,
                palette,
                enablement_runtime,
                workspace_lifecycle,
                recent_work,
                activity_center,
            );
            if let Some(state) = overlay.as_mut() {
                state.close(frame);
            }
            *overlay = None;
        }
        Err(err) => {
            activity_center.note_clone_failed(
                &job.operation_id,
                &job.request.remote_url,
                &job.request.destination_path,
                &err,
            );
            command_runtime.record(invocation_and_result_clone_failed(&job.session, &err));
            command_runtime.note_non_command_action(format!(
                "clone failed - {} ({})",
                err.class.as_str(),
                err.message
            ));
            set_clone_sheet_status(
                overlay,
                format!("{}: {}", err.class.as_str(), err.message),
                false,
            );
        }
    }
    true
}

fn clone_progress_label(event: &CloneProgressEvent) -> String {
    match event.phase {
        CloneProgressPhase::Starting => "Starting clone".to_string(),
        CloneProgressPhase::Completed => "Clone completed".to_string(),
        CloneProgressPhase::Progress => event.message.clone(),
    }
}

fn mono_timestamp_now() -> String {
    let duration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    format!(
        "mono:unix:{}.{:09}",
        duration.as_secs(),
        duration.subsec_nanos()
    )
}

fn write_restore_proposal_log(
    recovery_root: &Path,
    proposal: &RestoreProposal,
) -> Result<(), String> {
    std::fs::create_dir_all(recovery_root)
        .map_err(|err| format!("create recovery root failed: {err}"))?;
    let path = recovery_root.join("restore_proposal_latest.json");
    let json = serde_json::to_string_pretty(proposal)
        .map_err(|err| format!("serialize restore proposal failed: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("write {} failed: {err}", path.display()))?;
    Ok(())
}

fn write_restore_outcome_log(recovery_root: &Path, outcome: &RestoreOutcome) -> Result<(), String> {
    std::fs::create_dir_all(recovery_root)
        .map_err(|err| format!("create recovery root failed: {err}"))?;
    let path = recovery_root.join("restore_outcome_latest.json");
    let json = serde_json::to_string_pretty(outcome)
        .map_err(|err| format!("serialize restore outcome failed: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("write {} failed: {err}", path.display()))?;
    Ok(())
}

fn trust_state_for_recent_work(value: &str) -> TrustState {
    match value {
        "trusted" => TrustState::Trusted,
        "restricted" => TrustState::Restricted,
        "pending_evaluation" => TrustState::PendingEvaluation,
        _ => TrustState::PendingEvaluation,
    }
}

fn trust_state_label(value: &str) -> &'static str {
    match value {
        "trusted" => "Trusted",
        "restricted" => "Restricted",
        "pending_evaluation" => "Pending evaluation",
        _ => "Unknown",
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatusBarSlotSource {
    Recovery,
    WorkspaceContext,
    ExecutionContext,
    WorkSummary,
    FileMetadata,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StatusBarSlotActivation {
    RecoveryDetail,
    TrustReview,
    WorkspaceSwitcher,
    WorkSummary,
    SourceFidelity,
    Fallback,
}

struct StatusBarSlotPaint {
    label: String,
    attention: bool,
}

#[cfg(test)]
fn status_bar_slot_source(slot_id: &str) -> Option<StatusBarSlotSource> {
    match slot_id {
        "status.slot.recovery.primary" => Some(StatusBarSlotSource::Recovery),
        "status.slot.context.workspace" => Some(StatusBarSlotSource::WorkspaceContext),
        "status.slot.context.execution" => Some(StatusBarSlotSource::ExecutionContext),
        "status.slot.work.summary" => Some(StatusBarSlotSource::WorkSummary),
        "status.slot.metadata.file" => Some(StatusBarSlotSource::FileMetadata),
        _ => None,
    }
}

#[cfg(test)]
fn status_bar_slot_has_documented_fallback(slot_id: &str) -> bool {
    matches!(slot_id, "status.slot.extension.scoped")
}

fn status_bar_slot_activation(slot_id: &str) -> StatusBarSlotActivation {
    match slot_id {
        "status.slot.recovery.primary" => StatusBarSlotActivation::RecoveryDetail,
        "status.slot.context.workspace" => StatusBarSlotActivation::TrustReview,
        "status.slot.context.execution" => StatusBarSlotActivation::WorkspaceSwitcher,
        "status.slot.work.summary" => StatusBarSlotActivation::WorkSummary,
        "status.slot.metadata.file" => StatusBarSlotActivation::SourceFidelity,
        _ => StatusBarSlotActivation::Fallback,
    }
}

fn status_bar_snapshot_for_shell(
    frame: &DesktopFrame,
    editor_runtime: &EditorWorkspaceRuntimeState,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &WorkspaceLifecycleRuntimeState,
    recent_work: &RecentWorkRuntimeState,
    activity_center: &ActivityCenterRuntimeState,
    title_context_bar: &TitleContextBarStateRecord,
) -> StatusBarSnapshot {
    let activity_snapshot = activity_center.snapshot();
    let active_owners = status_bar_active_background_owners(&activity_snapshot);
    let observed_at = status_bar_observed_at(&activity_snapshot);
    let active_fidelity = editor_runtime.active_source_fidelity(
        frame.focused_editor_group(),
        frame.active_tab_id(frame.focused_editor_group()),
    );
    let target_label = status_bar_target_label(recent_work, title_context_bar);
    let workspace_id = workspace_lifecycle
        .machine
        .as_ref()
        .map(|machine| machine.workspace_id())
        .unwrap_or(title_context_bar.workspace_identity.workspace_ref.as_str());
    let execution_context_ref = workspace_lifecycle.machine.as_ref().map(|machine| {
        if machine.workspace_id().is_empty() {
            "execution_context.local_desktop.empty_shell".to_string()
        } else {
            format!("execution_context.local_desktop.{}", machine.workspace_id())
        }
    });
    let profile = &title_context_bar.profile_identity;

    let inputs = StatusBarInputs {
        workspace_id,
        workspace_trust_state: trust_state_for_recent_work(
            enablement_runtime.workspace_trust_state.as_str(),
        ),
        target: TargetSnapshot {
            target_class_token: title_host_class_token(title_context_bar.host_identity.host_class),
            target_label: target_label.as_str(),
            reachability_token: workspace_reachability_token(workspace_lifecycle),
            execution_context_ref: execution_context_ref.as_deref(),
            has_degraded_field: workspace_lifecycle_has_degraded_field(workspace_lifecycle)
                || matches!(
                    title_context_bar.host_identity.host_state,
                    HostStateClass::ReadOnlyDegraded
                        | HostStateClass::Offline
                        | HostStateClass::PolicyBlocked
                        | HostStateClass::MissingDetails
                        | HostStateClass::Mixed
                ),
        },
        profile: ProfileSnapshot {
            profile_label: profile.profile_label.as_str(),
            profile_mode_token: profile_mode_token(profile.profile_mode),
            deployment_profile_token: deployment_profile_token(profile.deployment_profile),
            identity_mode_token: title_identity_mode_token(profile.identity_mode),
        },
        encoding: EncodingSnapshot {
            source_fidelity: active_fidelity.as_ref(),
        },
        background: BackgroundStateSnapshot {
            active_owners: &active_owners,
            aggregate_degraded: status_bar_background_degraded(&activity_snapshot),
            observed_at: observed_at.as_str(),
        },
    };
    StatusBarSnapshot::project(&inputs)
}

fn status_bar_target_label(
    recent_work: &RecentWorkRuntimeState,
    title_context_bar: &TitleContextBarStateRecord,
) -> String {
    recent_work
        .active_workspace_label()
        .filter(|label| !label.trim().is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| title_context_bar.workspace_identity.display_label.clone())
}

fn workspace_reachability_token(
    workspace_lifecycle: &WorkspaceLifecycleRuntimeState,
) -> &'static str {
    match workspace_lifecycle
        .machine
        .as_ref()
        .map(|machine| machine.state())
    {
        Some(WorkspaceLifecycleState::Ready | WorkspaceLifecycleState::PartiallyReady) => {
            "reachable"
        }
        Some(
            WorkspaceLifecycleState::Discovered
            | WorkspaceLifecycleState::TrustEvaluating
            | WorkspaceLifecycleState::Opening,
        ) => "warming",
        Some(WorkspaceLifecycleState::Degraded) => "degraded",
        Some(WorkspaceLifecycleState::Closing | WorkspaceLifecycleState::Closed) => "unreachable",
        None => "reachable",
    }
}

fn workspace_lifecycle_has_degraded_field(
    workspace_lifecycle: &WorkspaceLifecycleRuntimeState,
) -> bool {
    let Some(machine) = workspace_lifecycle.machine.as_ref() else {
        return false;
    };
    machine.state() == WorkspaceLifecycleState::Degraded
        || machine
            .watcher_health()
            .is_some_and(|health| health.as_str() != "healthy")
}

fn title_host_class_token(host_class: crate::chrome::title_context_bar::HostClass) -> &'static str {
    match host_class {
        crate::chrome::title_context_bar::HostClass::Local => "local_host",
        crate::chrome::title_context_bar::HostClass::RemoteHost => "remote_host",
        crate::chrome::title_context_bar::HostClass::ContainerDevcontainer => {
            "container_devcontainer"
        }
        crate::chrome::title_context_bar::HostClass::ManagedWorkspace => "managed_workspace",
        crate::chrome::title_context_bar::HostClass::BrowserRuntimeBridge => {
            "browser_runtime_bridge"
        }
        crate::chrome::title_context_bar::HostClass::ServicePlane => "service_plane",
        crate::chrome::title_context_bar::HostClass::MixedLocalPlusRemote => {
            "mixed_local_plus_remote"
        }
        crate::chrome::title_context_bar::HostClass::UnknownMissingDetails => {
            "unknown_missing_details"
        }
    }
}

fn profile_mode_token(profile_mode: ProfileModeClass) -> &'static str {
    match profile_mode {
        ProfileModeClass::Standard => "standard",
        ProfileModeClass::TemporarySession => "temporary_session",
        ProfileModeClass::SafeMode => "safe_mode",
        ProfileModeClass::ImportedProfile => "imported_profile",
        ProfileModeClass::ManagedPolicyProfile => "managed_policy_profile",
        ProfileModeClass::SupportRecovery => "support_recovery",
    }
}

fn deployment_profile_token(deployment_profile: DeploymentProfileClass) -> &'static str {
    match deployment_profile {
        DeploymentProfileClass::IndividualLocal => "individual_local",
        DeploymentProfileClass::SelfHosted => "self_hosted",
        DeploymentProfileClass::EnterpriseOnline => "enterprise_online",
        DeploymentProfileClass::AirGapped => "air_gapped",
        DeploymentProfileClass::ManagedCloud => "managed_cloud",
    }
}

fn title_identity_mode_token(identity_mode: TitleIdentityMode) -> &'static str {
    match identity_mode {
        TitleIdentityMode::AccountFreeLocal => "account_free_local",
        TitleIdentityMode::SelfHostedOrg => "self_hosted_org",
        TitleIdentityMode::ManagedWorkspace => "managed_workspace",
    }
}

fn status_bar_active_background_owners(snapshot: &ActivityCenterSnapshot) -> Vec<&'static str> {
    let mut owners = BTreeSet::new();
    for row in &snapshot.rows {
        if row.is_terminal {
            continue;
        }
        owners.insert(source_subsystem_token(row.source_subsystem));
    }
    owners.into_iter().collect()
}

fn status_bar_background_degraded(snapshot: &ActivityCenterSnapshot) -> Option<DegradedStateToken> {
    if snapshot.rows.iter().any(|row| {
        !row.is_terminal
            && matches!(
                row.severity_class,
                SeverityClass::Critical | SeverityClass::Blocking | SeverityClass::Error
            )
    }) {
        return Some(DegradedStateToken::PolicyBlocked);
    }
    if snapshot.rows.iter().any(|row| {
        !row.is_terminal
            && matches!(
                row.severity_class,
                SeverityClass::Degraded | SeverityClass::Warning
            )
    }) {
        return Some(DegradedStateToken::Limited);
    }
    None
}

fn status_bar_observed_at(snapshot: &ActivityCenterSnapshot) -> String {
    snapshot
        .rows
        .iter()
        .map(|row| row.last_observed_at.as_str())
        .max()
        .unwrap_or("mono:status_bar:no_activity")
        .to_owned()
}

fn source_subsystem_token(source: SourceSubsystem) -> &'static str {
    match source {
        SourceSubsystem::Editor => "editor",
        SourceSubsystem::Terminal => "terminal",
        SourceSubsystem::ReviewAndDiff => "review_and_diff",
        SourceSubsystem::PaletteAndSearch => "palette_and_search",
        SourceSubsystem::InstallUpdateAttach => "install_update_attach",
        SourceSubsystem::AiApply => "ai_apply",
        SourceSubsystem::Collaboration => "collaboration",
        SourceSubsystem::ProviderBearing => "provider_bearing",
        SourceSubsystem::DocsHelpServiceHealth => "docs_help_service_health",
        SourceSubsystem::SupportExport => "support_export",
        SourceSubsystem::BuildSystem => "build_system",
        SourceSubsystem::TestRunner => "test_runner",
        SourceSubsystem::DebugSession => "debug_session",
        SourceSubsystem::TaskRunner => "task_runner",
        SourceSubsystem::Indexer => "indexer",
        SourceSubsystem::VfsSave => "vfs_save",
        SourceSubsystem::SyncMirror => "sync_mirror",
        SourceSubsystem::NotebookKernel => "notebook_kernel",
        SourceSubsystem::RemoteAgent => "remote_agent",
        SourceSubsystem::ExtensionHost => "extension_host",
        SourceSubsystem::WorkspaceTrust => "workspace_trust",
        SourceSubsystem::PolicyResolver => "policy_resolver",
        SourceSubsystem::AdminPolicy => "admin_policy",
        SourceSubsystem::SecretBroker => "secret_broker",
        SourceSubsystem::RuntimePowerManager => "runtime_power_manager",
        SourceSubsystem::Shell => "shell",
    }
}

fn status_bar_slot_paint(
    slot_id: &str,
    snapshot: &StatusBarSnapshot,
    title_context_bar: &TitleContextBarStateRecord,
) -> Option<StatusBarSlotPaint> {
    let paint = match slot_id {
        "status.slot.recovery.primary" => {
            if let Some(item) = snapshot.items.iter().find(|item| item.is_recovery_critical) {
                StatusBarSlotPaint {
                    label: status_bar_item_label(item),
                    attention: true,
                }
            } else {
                StatusBarSlotPaint {
                    label: title_context_bar
                        .projection_label(SurfaceKind::WorkspaceStatusItem)
                        .unwrap_or("Workspace ready")
                        .to_owned(),
                    attention: false,
                }
            }
        }
        "status.slot.context.workspace" => {
            let trust = snapshot.item(StatusBarItemKind::Trust)?;
            let profile = snapshot.item(StatusBarItemKind::Profile)?;
            StatusBarSlotPaint {
                label: format!(
                    "{}: {} | {}: {}",
                    trust.label,
                    trust.current_value_label,
                    profile.label,
                    profile.current_value_label
                ),
                attention: status_bar_items_attention([trust, profile]),
            }
        }
        "status.slot.context.execution" => {
            let target = snapshot.item(StatusBarItemKind::Target)?;
            StatusBarSlotPaint {
                label: status_bar_item_label(target),
                attention: status_bar_items_attention([target]),
            }
        }
        "status.slot.work.summary" => {
            let work = snapshot.item(StatusBarItemKind::BackgroundState)?;
            StatusBarSlotPaint {
                label: status_bar_item_label(work),
                attention: status_bar_items_attention([work]),
            }
        }
        "status.slot.metadata.file" => {
            let encoding = snapshot.item(StatusBarItemKind::Encoding)?;
            StatusBarSlotPaint {
                label: status_bar_item_label(encoding),
                attention: status_bar_items_attention([encoding]),
            }
        }
        _ => return None,
    };
    Some(paint)
}

fn status_bar_items_attention<'a>(
    items: impl IntoIterator<Item = &'a StatusBarItemRecord>,
) -> bool {
    items
        .into_iter()
        .any(|item| item.is_recovery_critical || item.degraded_token.is_some())
}

fn status_bar_item_label(item: &StatusBarItemRecord) -> String {
    format!("{}: {}", item.label, item.current_value_label)
}

#[derive(Default)]
struct CommandDispatchRuntimeIo<'a> {
    window: Option<&'a winit::window::Window>,
    damage_geometry: Option<&'a ShellDamageGeometryCache>,
    clipboard: Option<&'a mut ClipboardState>,
    appearance: Option<&'a mut AppearanceRuntimeState>,
}

fn focused_editor_target(
    frame: &DesktopFrame,
    editor_runtime: &EditorWorkspaceRuntimeState,
) -> Option<(PaneId, EditorTabId)> {
    if frame.focused_zone() != ShellZoneId::MainWorkspace {
        return None;
    }
    let group = frame.focused_editor_group();
    let tab = frame.active_tab_id(group)?;
    editor_runtime
        .groups
        .get(&group)
        .and_then(|group_session| group_session.tabs.get(&tab))
        .map(|_| (group, tab))
}

fn focused_editor_authority(
    frame: &DesktopFrame,
    editor_runtime: &EditorWorkspaceRuntimeState,
) -> Option<(PaneId, EditorTabId, Rc<RefCell<BufferAuthority>>)> {
    let (group, tab) = focused_editor_target(frame, editor_runtime)?;
    let authority = editor_runtime
        .groups
        .get(&group)
        .and_then(|group_session| group_session.tabs.get(&tab))
        .map(|session| session.authority.clone())?;
    Some((group, tab, authority))
}

fn editor_target_ref(group: PaneId, tab: EditorTabId, suffix: &str) -> String {
    format!("editor:{}:{}:{suffix}", group.value(), tab.0)
}

fn active_editor_ref(
    frame: &DesktopFrame,
    editor_runtime: &EditorWorkspaceRuntimeState,
) -> Option<String> {
    let (group, tab, _) = focused_editor_authority(frame, editor_runtime)?;
    Some(editor_target_ref(group, tab, "active"))
}

fn writable_editor_ref(
    frame: &DesktopFrame,
    editor_runtime: &EditorWorkspaceRuntimeState,
    require_file_backed: bool,
) -> Option<String> {
    let (group, tab, authority) = focused_editor_authority(frame, editor_runtime)?;
    let auth = authority.borrow();
    if auth.read_only != ReadOnlyState::Writable {
        return None;
    }
    if require_file_backed && auth.file_path.is_none() {
        return None;
    }
    Some(editor_target_ref(
        group,
        tab,
        if require_file_backed {
            "writable-file"
        } else {
            "writable"
        },
    ))
}

fn undo_stack_ref(
    frame: &DesktopFrame,
    editor_runtime: &EditorWorkspaceRuntimeState,
    redo: bool,
) -> Option<String> {
    let (group, tab, authority) = focused_editor_authority(frame, editor_runtime)?;
    let auth = authority.borrow();
    let available = if redo {
        aureline_editor::undo::next_redo(&auth.buffer).is_some()
    } else {
        aureline_editor::undo::next_undo(&auth.buffer).is_some()
    };
    available.then(|| editor_target_ref(group, tab, if redo { "redo" } else { "undo" }))
}

fn active_search_ref(
    frame: &DesktopFrame,
    editor_runtime: &EditorWorkspaceRuntimeState,
) -> Option<String> {
    let (group, tab, authority) = focused_editor_authority(frame, editor_runtime)?;
    let auth = authority.borrow();
    let active = auth.find_replace.mode() != FindReplaceMode::Hidden
        && !auth.find_replace.query().trim().is_empty();
    active.then(|| editor_target_ref(group, tab, "search"))
}

fn inferred_editor_argument(
    argument_name: &str,
    resolved_value_ref: Option<String>,
) -> ArgumentProvenanceEntry {
    ArgumentProvenanceEntry {
        argument_name: argument_name.to_string(),
        provenance: "inferred_from_focused_context".to_string(),
        resolved_value_ref,
    }
}

fn argument_provenance_map_for_shell(
    entry: &CommandRegistryEntryRecord,
    frame: &DesktopFrame,
    editor_runtime: &EditorWorkspaceRuntimeState,
) -> Vec<ArgumentProvenanceEntry> {
    match entry.descriptor.command_id.as_str() {
        "cmd:editor.save" => vec![inferred_editor_argument(
            "writable_file_backed_editor_ref",
            writable_editor_ref(frame, editor_runtime, true),
        )],
        "cmd:editor.copy" => vec![inferred_editor_argument(
            "active_editor_ref",
            active_editor_ref(frame, editor_runtime),
        )],
        "cmd:editor.cut" | "cmd:editor.paste" => vec![inferred_editor_argument(
            "writable_editor_ref",
            writable_editor_ref(frame, editor_runtime, false),
        )],
        "cmd:editor.undo" => vec![inferred_editor_argument(
            "undo_stack_ref",
            undo_stack_ref(frame, editor_runtime, false),
        )],
        "cmd:editor.redo" => vec![inferred_editor_argument(
            "redo_stack_ref",
            undo_stack_ref(frame, editor_runtime, true),
        )],
        "cmd:editor.find_next" | "cmd:editor.find_previous" => vec![inferred_editor_argument(
            "active_search_ref",
            active_search_ref(frame, editor_runtime),
        )],
        _ => argument_provenance_map_for(entry),
    }
}

fn status_bar_detail_lines(
    title: &str,
    snapshot: &StatusBarSnapshot,
    kinds: &[StatusBarItemKind],
) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!("{title} - Esc closes"));
    lines.push(format!("workspace: {}", snapshot.workspace_id));
    lines.push(format!("observed_at: {}", snapshot.observed_at));
    for kind in kinds {
        if let Some(item) = snapshot.item(*kind) {
            lines.push(String::new());
            lines.push(status_bar_item_label(item));
            lines.push(format!("command: {}", item.primary_command_id));
            lines.push(format!("opens: {}", item.opens_surface_ref));
            lines.push(format!("truth: {}", item.truth_source_ref));
            if let Some(token) = item.degraded_token.as_deref() {
                lines.push(format!("degraded: {token}"));
            }
            lines.push(item.explanation.clone());
        }
    }
    lines
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
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    palette: &mut CommandPaletteState,
    palette_focus_return: &mut FocusReturnStack<ShellFocusReturnTarget>,
    overlay: &mut Option<ShellOverlayState>,
    command_id: &str,
    origin: DispatchOrigin,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
) -> bool {
    let mut io = CommandDispatchRuntimeIo::default();
    dispatch_command_id_with_io(
        command_runtime,
        registry,
        frame,
        editor_runtime,
        palette,
        palette_focus_return,
        overlay,
        command_id,
        origin,
        enablement_runtime,
        workspace_lifecycle,
        recent_work,
        activity_center,
        &mut io,
    )
}

fn dispatch_command_id_with_io(
    command_runtime: &mut CommandRuntimeState,
    registry: &CommandRegistry,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    palette: &mut CommandPaletteState,
    palette_focus_return: &mut FocusReturnStack<ShellFocusReturnTarget>,
    overlay: &mut Option<ShellOverlayState>,
    command_id: &str,
    origin: DispatchOrigin,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    io: &mut CommandDispatchRuntimeIo<'_>,
) -> bool {
    dispatch_command_id_with_arguments_and_io(
        command_runtime,
        registry,
        frame,
        editor_runtime,
        palette,
        palette_focus_return,
        overlay,
        command_id,
        origin,
        enablement_runtime,
        workspace_lifecycle,
        recent_work,
        activity_center,
        None,
        io,
    )
}

fn dispatch_command_id_with_arguments(
    command_runtime: &mut CommandRuntimeState,
    registry: &CommandRegistry,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    palette: &mut CommandPaletteState,
    palette_focus_return: &mut FocusReturnStack<ShellFocusReturnTarget>,
    overlay: &mut Option<ShellOverlayState>,
    command_id: &str,
    origin: DispatchOrigin,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    argument_provenance_map_override: Option<Vec<ArgumentProvenanceEntry>>,
) -> bool {
    let mut io = CommandDispatchRuntimeIo::default();
    dispatch_command_id_with_arguments_and_io(
        command_runtime,
        registry,
        frame,
        editor_runtime,
        palette,
        palette_focus_return,
        overlay,
        command_id,
        origin,
        enablement_runtime,
        workspace_lifecycle,
        recent_work,
        activity_center,
        argument_provenance_map_override,
        &mut io,
    )
}

fn dispatch_command_id_with_arguments_and_io(
    command_runtime: &mut CommandRuntimeState,
    registry: &CommandRegistry,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    palette: &mut CommandPaletteState,
    palette_focus_return: &mut FocusReturnStack<ShellFocusReturnTarget>,
    overlay: &mut Option<ShellOverlayState>,
    command_id: &str,
    origin: DispatchOrigin,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    argument_provenance_map_override: Option<Vec<ArgumentProvenanceEntry>>,
    io: &mut CommandDispatchRuntimeIo<'_>,
) -> bool {
    let Some(entry) = registry.get(command_id).cloned() else {
        return false;
    };
    dispatch_registry_entry(
        command_runtime,
        registry,
        frame,
        editor_runtime,
        palette,
        palette_focus_return,
        overlay,
        &entry,
        origin,
        enablement_runtime,
        workspace_lifecycle,
        recent_work,
        activity_center,
        argument_provenance_map_override,
        io,
    )
}

fn dispatch_registry_entry(
    command_runtime: &mut CommandRuntimeState,
    registry: &CommandRegistry,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    palette: &mut CommandPaletteState,
    palette_focus_return: &mut FocusReturnStack<ShellFocusReturnTarget>,
    overlay: &mut Option<ShellOverlayState>,
    entry: &CommandRegistryEntryRecord,
    origin: DispatchOrigin,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    argument_provenance_map_override: Option<Vec<ArgumentProvenanceEntry>>,
    io: &mut CommandDispatchRuntimeIo<'_>,
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
        "cmd:workspace.clone_repository" => "apply_after_preview",
        "cmd:workspace.restore_from_checkpoint" => "apply_after_preview",
        "cmd:workspace.import_profile" => "apply_after_preview",
        "cmd:explorer.toggle" => "focus_structural_navigation",
        "cmd:terminal.toggle" => "run_interactive_terminal",
        "cmd:editor.save" => "save_active_editor",
        "cmd:editor.copy" => "copy_editor_selection",
        "cmd:editor.cut" => "cut_editor_selection",
        "cmd:editor.paste" => "paste_into_editor",
        "cmd:editor.undo" => "undo_editor_edit",
        "cmd:editor.redo" => "redo_editor_edit",
        "cmd:editor.find_next" => "navigate_search_match",
        "cmd:editor.find_previous" => "navigate_search_match",
        "cmd:quick_open.toggle" => "open_quick_open",
        "cmd:settings.open" => "open_settings_overlay",
        _ => "query_only_no_mutation",
    };

    let argument_provenance_map = argument_provenance_map_override
        .unwrap_or_else(|| argument_provenance_map_for_shell(entry, frame, editor_runtime));

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

    if entry.descriptor.command_id == "cmd:workspace.clone_repository"
        && clone_request_from_argument_map(&session.argument_provenance_map).is_err()
    {
        let outcome = resolve_entry_flow(EntryFlowRequest {
            entry_verb: EntryVerb::Clone,
            target: EntryFlowTarget::ExplicitTargetKind(TargetKind::RemoteRepository),
            preferred_resulting_mode: Some(ResultingMode::CloneThenReview),
        });
        *overlay = Some(ShellOverlayState::entry_flow_sheet(
            frame.focused_zone(),
            frame.focused_editor_group(),
            outcome,
            entry.descriptor.command_id.clone(),
            origin,
            session.argument_provenance_map.clone(),
            None,
            None,
        ));
        frame.focus_zone(ShellZoneId::TransientOverlay);
        return true;
    }

    if entry.descriptor.command_id == "cmd:workspace.import_profile"
        && import_source_path_from_argument_map(&session.argument_provenance_map).is_none()
    {
        let outcome = resolve_entry_flow(EntryFlowRequest {
            entry_verb: EntryVerb::Import,
            target: EntryFlowTarget::ExplicitTargetKind(TargetKind::CompetitorConfigRoot),
            preferred_resulting_mode: Some(ResultingMode::ExtractThenReview),
        });
        *overlay = Some(ShellOverlayState::entry_flow_sheet(
            frame.focused_zone(),
            frame.focused_editor_group(),
            outcome,
            entry.descriptor.command_id.clone(),
            origin,
            session.argument_provenance_map.clone(),
            Some(DegradedStateToken::Labs),
            Some(
                "Review is available; apply records the import plan without changing settings."
                    .to_string(),
            ),
        ));
        frame.focus_zone(ShellZoneId::TransientOverlay);
        return true;
    }

    let enablement_context = CommandEnablementContext {
        client_scope: "desktop_product".to_string(),
        workspace_trust_state: enablement_runtime.workspace_trust_state.clone(),
        execution_context_available: enablement_runtime.execution_context_available,
        provider_linked: enablement_runtime.provider_linked,
        credential_available: enablement_runtime.credential_available,
        policy_disabled: enablement_runtime.policy_disabled,
        policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
        labs_enabled: enablement_runtime.labs_enabled,
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
        labs_enabled: enablement_runtime.labs_enabled,
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

        let record = materialize_command_diagnostics_sheet_record_with_arguments(
            entry,
            review_runtime,
            session.argument_provenance_map.clone(),
        );
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
            if !palette.is_open() {
                palette_focus_return.record_if_changed(ShellFocusReturnTarget::capture(frame));
                palette.open(registry, cwd);
                frame.focus_zone(ShellZoneId::TransientOverlay);
            }
            let invocation = invocation_and_result_simple_success(&session, "succeeded");
            command_runtime.record(invocation);
            true
        }
        "cmd:settings.open" => {
            if let Some(appearance) = io.appearance.as_deref() {
                *overlay = Some(ShellOverlayState::settings(
                    frame.focused_zone(),
                    frame.focused_editor_group(),
                    appearance,
                ));
                frame.focus_zone(ShellZoneId::TransientOverlay);
                let invocation = invocation_and_result_simple_success(&session, "succeeded");
                command_runtime.record(invocation);
                true
            } else {
                command_runtime.note_non_command_action("settings unavailable");
                let invocation = invocation_and_result_simple_success(&session, "no_op");
                command_runtime.record(invocation);
                true
            }
        }
        "cmd:docs.open_in_browser" => {
            let destination_anchor_ref = session
                .argument_provenance_map
                .iter()
                .find(|row| row.argument_name == "destination_anchor_ref")
                .and_then(|row| row.resolved_value_ref.as_deref())
                .unwrap_or("docs:anchor:docs:open_in_browser_overview");

            let packet_ref = if destination_anchor_ref.starts_with("id:browser-handoff:") {
                destination_anchor_ref.to_string()
            } else {
                "id:browser-handoff:docs-help:project-docs".to_string()
            };

            let Some(url) = resolve_docs_help_handoff_url(&packet_ref) else {
                command_runtime.record(invocation_and_result_docs_open_in_browser_failed(
                    &session,
                    &packet_ref,
                    "handoff_url_unresolved".to_string(),
                ));
                return true;
            };

            match webbrowser::open(&url) {
                Ok(_) => {
                    command_runtime.record(invocation_and_result_docs_open_in_browser_succeeded(
                        &session,
                        &packet_ref,
                    ));
                }
                Err(err) => {
                    command_runtime.record(invocation_and_result_docs_open_in_browser_failed(
                        &session,
                        &packet_ref,
                        err.to_string(),
                    ));
                }
            }
            true
        }
        "cmd:editor.find" | "cmd:editor.replace" => {
            let mode = if entry.descriptor.command_id == "cmd:editor.replace" {
                FindReplaceMode::Replace
            } else {
                FindReplaceMode::Find
            };

            let group = frame.focused_editor_group();
            let Some(tab) = frame.active_tab_id(group) else {
                command_runtime.note_non_command_action("find/replace: no active editor tab");
                command_runtime.record(invocation_and_result_simple_success(&session, "no_op"));
                return true;
            };
            let Some(tab_session) = editor_runtime.tab_session_mut(group, tab) else {
                command_runtime.note_non_command_action("find/replace: missing editor tab session");
                command_runtime.record(invocation_and_result_simple_success(&session, "no_op"));
                return true;
            };
            tab_session.ensure_fresh_snapshot();

            let caret = tab_session.viewport.caret();
            let snapshot = tab_session.snapshot.clone();
            let authority = tab_session.authority.clone();
            {
                let mut auth = authority.borrow_mut();
                auth.find_replace.set_mode(mode);
                let _ = auth.find_replace.sync_for_view(&snapshot, caret);
            }

            *overlay = Some(ShellOverlayState::find_replace(
                frame.focused_zone(),
                frame.focused_editor_group(),
                authority,
                mode,
            ));
            frame.focus_zone(ShellZoneId::TransientOverlay);
            let invocation = invocation_and_result_simple_success(&session, "succeeded");
            command_runtime.record(invocation);
            true
        }
        "cmd:editor.save" => dispatch_editor_save_command(
            command_runtime,
            frame,
            editor_runtime,
            overlay,
            activity_center,
            &session,
        ),
        "cmd:editor.copy" => dispatch_editor_clipboard_command(
            command_runtime,
            frame,
            editor_runtime,
            io,
            &session,
            EditorClipboardCommand::Copy,
        ),
        "cmd:editor.cut" => dispatch_editor_clipboard_command(
            command_runtime,
            frame,
            editor_runtime,
            io,
            &session,
            EditorClipboardCommand::Cut,
        ),
        "cmd:editor.paste" => dispatch_editor_clipboard_command(
            command_runtime,
            frame,
            editor_runtime,
            io,
            &session,
            EditorClipboardCommand::Paste,
        ),
        "cmd:editor.undo" => dispatch_editor_undo_redo_command(
            command_runtime,
            frame,
            editor_runtime,
            &session,
            false,
        ),
        "cmd:editor.redo" => dispatch_editor_undo_redo_command(
            command_runtime,
            frame,
            editor_runtime,
            &session,
            true,
        ),
        "cmd:editor.find_next" => dispatch_find_navigation_command(
            command_runtime,
            frame,
            editor_runtime,
            &session,
            false,
        ),
        "cmd:editor.find_previous" => {
            dispatch_find_navigation_command(command_runtime, frame, editor_runtime, &session, true)
        }
        "cmd:quick_open.toggle" => {
            let cwd = std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."));
            if palette.is_open() {
                palette.close();
                if let Some(target) = palette_focus_return.pop() {
                    target.apply(frame);
                }
            } else {
                palette_focus_return.record_if_changed(ShellFocusReturnTarget::capture(frame));
                palette.open(registry, cwd);
                frame.focus_zone(ShellZoneId::TransientOverlay);
            }
            command_runtime.record(invocation_and_result_simple_success(&session, "succeeded"));
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
        WEDGE_INSPECTOR_COMMAND_ID => {
            let workspace_id = editor_runtime
                .terminal_pane
                .active_workspace_id()
                .or_else(|| recent_work.active_workspace_label())
                .unwrap_or("workspace:wedge_inspector")
                .to_string();
            let inspector = WedgeInspectorOverlay::new(WedgeInspectorInputs {
                host_boundary_card: editor_runtime.terminal_pane.active_host_boundary_card(),
                workspace_lifecycle_state_token: workspace_lifecycle
                    .status_badge_token()
                    .map(str::to_string),
                install_review_card: None,
                observed_at: mono_timestamp_now(),
                workspace_id,
            });
            *overlay = Some(ShellOverlayState::wedge_inspector(
                frame.focused_zone(),
                frame.focused_editor_group(),
                inspector,
            ));
            frame.focus_zone(ShellZoneId::TransientOverlay);
            let invocation = invocation_and_result_simple_success(&session, "succeeded");
            command_runtime.record(invocation);
            true
        }
        "cmd:explorer.toggle" => {
            if frame.layout().zone(ShellZoneId::LeftSidebar).is_some() {
                if frame.focused_zone() == ShellZoneId::LeftSidebar {
                    frame.focus_zone(ShellZoneId::MainWorkspace);
                } else {
                    frame.focus_zone(ShellZoneId::LeftSidebar);
                }
                command_runtime.note_non_command_action("explorer toggled");
                command_runtime.record(invocation_and_result_simple_success(&session, "succeeded"));
            } else {
                command_runtime.note_non_command_action("explorer unavailable at this width");
                command_runtime.record(invocation_and_result_simple_success(&session, "no_op"));
            }
            true
        }
        "cmd:terminal.toggle" => {
            let trust_state =
                trust_state_for_recent_work(enablement_runtime.workspace_trust_state.as_str());
            let session_opened = editor_runtime
                .terminal_pane
                .ensure_session_for_active_workspace(trust_state, &mono_timestamp_now())
                .is_some();
            if session_opened {
                if frame.focused_zone() == ShellZoneId::BottomPanel {
                    frame.focus_zone(ShellZoneId::MainWorkspace);
                } else {
                    frame.focus_zone(ShellZoneId::BottomPanel);
                }
                command_runtime.note_non_command_action("terminal toggled");
                command_runtime.record(invocation_and_result_simple_success(&session, "succeeded"));
            } else {
                command_runtime
                    .note_non_command_action("terminal unavailable: open a workspace first");
                command_runtime.record(invocation_and_result_simple_success(&session, "no_op"));
            }
            true
        }
        "cmd:workspace.open_folder" => {
            let scope_ref = session
                .argument_provenance_map
                .iter()
                .find(|row| row.argument_name == "workspace_scope_ref")
                .and_then(|row| row.resolved_value_ref.as_deref());
            if scope_ref.is_some_and(|scope| scope.contains("workspace_file")) {
                command_runtime.note_non_command_action(
                    "open workspace requested (workspace file selection not implemented)",
                );
                return true;
            }

            let selected_path = match folder_picker::pick_folder() {
                Some(path) => path,
                None => {
                    command_runtime.record(invocation_and_result_open_folder_cancelled(&session));
                    command_runtime.note_non_command_action("open folder cancelled");
                    return true;
                }
            };
            let folder_path = match resolve_open_workspace_path(&selected_path) {
                Ok(path) => path,
                Err(err) => {
                    command_runtime.note_non_command_action(format!("open folder failed — {err}"));
                    return true;
                }
            };
            command_runtime.record(invocation_and_result_open_folder_succeeded(&session));
            open_local_folder_workspace(
                &folder_path,
                LocalFolderOpenSource::FolderPicker,
                command_runtime,
                frame,
                editor_runtime,
                palette,
                enablement_runtime,
                workspace_lifecycle,
                recent_work,
                activity_center,
            );
            true
        }
        "cmd:workspace.clone_repository" => {
            let err = CloneError::new(
                CloneErrorClass::InvalidInput,
                "clone requires the clone flow sheet",
            );
            command_runtime.record(invocation_and_result_clone_failed(&session, &err));
            command_runtime.note_non_command_action("clone requires the clone flow sheet");
            true
        }
        "cmd:workspace.restore_from_checkpoint" => {
            let recovery_root = PathBuf::from(".logs").join("recovery");
            let mut direct_session_store = SessionRestoreStore::new(&recovery_root);
            let changed = apply_restore_from_checkpoint(
                command_runtime,
                frame,
                editor_runtime,
                &mut direct_session_store,
                palette,
                enablement_runtime,
                workspace_lifecycle,
                recent_work,
                activity_center,
                &session,
            );
            if !changed {
                command_runtime
                    .note_non_command_action("restore requested but no restorable state was found");
            }
            true
        }
        "cmd:workspace.import_profile" => {
            let review = import_review_from_argument_map(&session.argument_provenance_map);
            record_import_review_stub(command_runtime, activity_center, &session, &review);
            true
        }
        _ => {
            let invocation = invocation_and_result_unimplemented(&session);
            command_runtime.record(invocation);
            true
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EditorClipboardCommand {
    Copy,
    Cut,
    Paste,
}

fn dispatch_editor_save_command(
    command_runtime: &mut CommandRuntimeState,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    overlay: &mut Option<ShellOverlayState>,
    activity_center: &mut ActivityCenterRuntimeState,
    invocation_session: &CommandInvocationSession,
) -> bool {
    let group = frame.focused_editor_group();
    let Some(tab) = frame.active_tab_id(group) else {
        command_runtime.record(invocation_and_result_simple_success(
            invocation_session,
            "no_op",
        ));
        command_runtime.note_non_command_action("save: no active editor tab");
        return true;
    };
    let save_label = editor_runtime
        .tab_render_info(group, tab)
        .map(|info| info.label)
        .unwrap_or_else(|| "Untitled".to_string());
    let save_operation_id = activity_center.next_save_operation_id(&save_label);
    activity_center.note_save_running(&save_operation_id, &save_label);

    match editor_runtime.save_tab(group, tab) {
        Ok(SaveTabAttempt::Saved(result)) => {
            activity_center.note_save_completed(
                &save_operation_id,
                &save_label,
                Some(result.packet_id.clone()),
            );
            command_runtime.record(invocation_and_result_simple_success(
                invocation_session,
                "succeeded",
            ));
            command_runtime.note_non_command_action(format!(
                "saved ({}) - outcome={} strategy={}",
                result.packet_id,
                result.manifest.outcome.as_str(),
                result.write_strategy.as_str()
            ));
        }
        Ok(SaveTabAttempt::NoTarget) => {
            activity_center.note_save_completed(&save_operation_id, &save_label, None);
            command_runtime.record(invocation_and_result_simple_success(
                invocation_session,
                "no_op",
            ));
            command_runtime.note_non_command_action("save: no file target");
        }
        Ok(SaveTabAttempt::ReviewRequired { record, outcome }) => {
            activity_center.note_save_failed(
                &save_operation_id,
                &save_label,
                format!("Save requires review before commit ({})", outcome.as_str()),
            );
            command_runtime.record(invocation_and_result_simple_success(
                invocation_session,
                "review_required",
            ));
            *overlay = Some(ShellOverlayState::save_review(
                frame.focused_zone(),
                frame.focused_editor_group(),
                group,
                tab,
                record,
                outcome,
            ));
            frame.focus_zone(ShellZoneId::TransientOverlay);
        }
        Err(err) => {
            activity_center.note_save_failed(&save_operation_id, &save_label, err.clone());
            command_runtime.record(invocation_and_result_simple_success(
                invocation_session,
                "failed",
            ));
            command_runtime.note_non_command_action(format!("save failed - {err}"));
        }
    }
    true
}

fn dispatch_editor_undo_redo_command(
    command_runtime: &mut CommandRuntimeState,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    invocation_session: &CommandInvocationSession,
    redo: bool,
) -> bool {
    let group = frame.focused_editor_group();
    let Some(tab) = frame.active_tab_id(group) else {
        command_runtime.record(invocation_and_result_simple_success(
            invocation_session,
            "no_op",
        ));
        return true;
    };
    if !editor_runtime.has_tab_session(group, tab) {
        editor_runtime.open_placeholder(group, tab);
    }
    let Some(tab_session) = editor_runtime.tab_session_mut(group, tab) else {
        command_runtime.record(invocation_and_result_simple_success(
            invocation_session,
            "no_op",
        ));
        return true;
    };
    tab_session.ensure_fresh_snapshot();

    let (changed, summary) = {
        let mut authority = tab_session.authority.borrow_mut();
        let summary = if redo {
            aureline_editor::undo::next_redo(&authority.buffer)
        } else {
            aureline_editor::undo::next_undo(&authority.buffer)
        };
        let outcome = if redo {
            authority.buffer.redo()
        } else {
            authority.buffer.undo()
        };
        (outcome.is_some(), summary)
    };
    if !changed {
        command_runtime.record(invocation_and_result_simple_success(
            invocation_session,
            "no_op",
        ));
        return true;
    }

    tab_session.refresh_snapshot_and_cache();
    tab_session.text_input.force_clear_composition();
    tab_session.viewport.set_ime_composition(None);
    tab_session.needs_text_repaint = true;
    command_runtime.record(invocation_and_result_simple_success(
        invocation_session,
        "succeeded",
    ));
    if let Some(summary) = summary {
        let verb = if redo { "redo" } else { "undo" };
        command_runtime.note_non_command_action(format!(
            "{verb}: {} ({})",
            summary.label_or_class_id(),
            summary.class_id
        ));
    }
    true
}

fn focused_viewport_rect(io: &CommandDispatchRuntimeIo<'_>) -> Option<PixelRect> {
    io.damage_geometry
        .and_then(|damage| damage.focused_editor_viewport)
        .map(|rect| PixelRect::new(rect.x, rect.y, rect.width, rect.height))
}

fn refresh_ime_cursor_after_edit(
    window: Option<&winit::window::Window>,
    viewport: &EditorViewport,
    viewport_rect: Option<PixelRect>,
) {
    if let (Some(window), Some(rect)) = (window, viewport_rect) {
        update_ime_cursor_area_for_viewport(window, viewport, rect);
    }
}

fn dispatch_editor_clipboard_command(
    command_runtime: &mut CommandRuntimeState,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    io: &mut CommandDispatchRuntimeIo<'_>,
    invocation_session: &CommandInvocationSession,
    command: EditorClipboardCommand,
) -> bool {
    let window = io.window;
    let viewport_rect = focused_viewport_rect(io);
    let Some(clipboard) = io.clipboard.as_mut() else {
        command_runtime.record(invocation_and_result_simple_success(
            invocation_session,
            "failed",
        ));
        command_runtime.note_non_command_action("clipboard unavailable");
        return true;
    };

    let group = frame.focused_editor_group();
    let Some(tab) = frame.active_tab_id(group) else {
        command_runtime.record(invocation_and_result_simple_success(
            invocation_session,
            "no_op",
        ));
        return true;
    };
    let Some(tab_session) = editor_runtime.tab_session_mut(group, tab) else {
        command_runtime.record(invocation_and_result_simple_success(
            invocation_session,
            "no_op",
        ));
        return true;
    };
    tab_session.ensure_fresh_snapshot();

    match command {
        EditorClipboardCommand::Copy => match aureline_editor::clipboard::plan_copy_default(
            &tab_session.snapshot,
            tab_session.viewport.selections(),
        ) {
            Ok(payload) => match clipboard.set_text(&payload.text) {
                Ok(()) => {
                    command_runtime.record(invocation_and_result_simple_success(
                        invocation_session,
                        "succeeded",
                    ));
                    let label = match payload.copy_variant_id {
                        aureline_editor::clipboard::CopyVariantId::SelectionRaw => {
                            "copied selection"
                        }
                        aureline_editor::clipboard::CopyVariantId::Line => "copied line",
                    };
                    command_runtime.note_non_command_action(label);
                }
                Err(err) => {
                    command_runtime.record(invocation_and_result_simple_success(
                        invocation_session,
                        "failed",
                    ));
                    command_runtime.note_non_command_action(format!("copy failed - {err}"));
                }
            },
            Err(err) => {
                command_runtime.record(invocation_and_result_simple_success(
                    invocation_session,
                    "failed",
                ));
                command_runtime.note_non_command_action(format!("copy unavailable - {err}"));
            }
        },
        EditorClipboardCommand::Cut => match aureline_editor::clipboard::plan_cut_default(
            &tab_session.snapshot,
            tab_session.viewport.selections(),
        ) {
            Ok(plan) => {
                let aureline_editor::clipboard::CutPayload {
                    payload,
                    delete_ranges,
                } = plan;
                match clipboard.set_text(&payload.text) {
                    Ok(()) => {
                        let outcome = {
                            let mut authority = tab_session.authority.borrow_mut();
                            tab_session
                                .viewport
                                .selections_mut()
                                .apply_delete_byte_ranges(
                                    &mut authority.buffer,
                                    &tab_session.snapshot,
                                    delete_ranges,
                                    aureline_editor::undo::originator::CUT,
                                )
                        };

                        match outcome {
                            Ok(Some(outcome)) => {
                                tab_session.snapshot = outcome.snapshot;
                                tab_session.last_seen_revision = outcome.revision;
                                tab_session.refresh_document_cache();
                                tab_session.viewport.set_ime_composition(None);
                                tab_session.needs_text_repaint = true;
                                refresh_ime_cursor_after_edit(
                                    window,
                                    &tab_session.viewport,
                                    viewport_rect,
                                );
                                command_runtime.record(invocation_and_result_simple_success(
                                    invocation_session,
                                    "succeeded",
                                ));
                                command_runtime.note_non_command_action("cut");
                            }
                            Ok(None) => {
                                command_runtime.record(invocation_and_result_simple_success(
                                    invocation_session,
                                    "no_op",
                                ));
                                command_runtime.note_non_command_action("cut: no-op");
                            }
                            Err(err) => {
                                command_runtime.record(invocation_and_result_simple_success(
                                    invocation_session,
                                    "failed",
                                ));
                                command_runtime
                                    .note_non_command_action(format!("cut failed - {err}"));
                            }
                        }
                    }
                    Err(err) => {
                        command_runtime.record(invocation_and_result_simple_success(
                            invocation_session,
                            "failed",
                        ));
                        command_runtime.note_non_command_action(format!("cut failed - {err}"));
                    }
                }
            }
            Err(err) => {
                command_runtime.record(invocation_and_result_simple_success(
                    invocation_session,
                    "failed",
                ));
                command_runtime.note_non_command_action(format!("cut unavailable - {err}"));
            }
        },
        EditorClipboardCommand::Paste => match clipboard.get_text() {
            Ok(text) => {
                let scope = if tab_session.viewport.caret_count() > 1
                    && tab_session.viewport.ime_composition().is_some()
                {
                    aureline_editor::TextEditScope::PrimaryOnly
                } else {
                    aureline_editor::TextEditScope::AllCarets
                };

                let outcome = {
                    let mut authority = tab_session.authority.borrow_mut();
                    tab_session.viewport.selections_mut().apply_insert_text(
                        &mut authority.buffer,
                        &tab_session.snapshot,
                        &text,
                        aureline_editor::undo::originator::PASTE,
                        scope,
                    )
                };

                match outcome {
                    Ok(Some(outcome)) => {
                        tab_session.snapshot = outcome.snapshot;
                        tab_session.last_seen_revision = outcome.revision;
                        tab_session.refresh_document_cache();
                        tab_session.viewport.set_ime_composition(None);
                        tab_session.needs_text_repaint = true;
                        refresh_ime_cursor_after_edit(window, &tab_session.viewport, viewport_rect);
                        command_runtime.record(invocation_and_result_simple_success(
                            invocation_session,
                            "succeeded",
                        ));
                        command_runtime.note_non_command_action("pasted");
                    }
                    Ok(None) => {
                        command_runtime.record(invocation_and_result_simple_success(
                            invocation_session,
                            "no_op",
                        ));
                        command_runtime.note_non_command_action("paste: no-op");
                    }
                    Err(err) => {
                        command_runtime.record(invocation_and_result_simple_success(
                            invocation_session,
                            "failed",
                        ));
                        command_runtime.note_non_command_action(format!("paste failed - {err}"));
                    }
                }
            }
            Err(err) => {
                command_runtime.record(invocation_and_result_simple_success(
                    invocation_session,
                    "failed",
                ));
                command_runtime.note_non_command_action(format!("paste unavailable - {err}"));
            }
        },
    }
    true
}

fn dispatch_find_navigation_command(
    command_runtime: &mut CommandRuntimeState,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    invocation_session: &CommandInvocationSession,
    previous: bool,
) -> bool {
    let group = frame.focused_editor_group();
    let Some(tab) = frame.active_tab_id(group) else {
        command_runtime.record(invocation_and_result_simple_success(
            invocation_session,
            "no_op",
        ));
        return true;
    };
    let Some(tab_session) = editor_runtime.tab_session_mut(group, tab) else {
        command_runtime.record(invocation_and_result_simple_success(
            invocation_session,
            "no_op",
        ));
        return true;
    };
    tab_session.ensure_fresh_snapshot();
    let caret = tab_session.viewport.caret();
    let snapshot = tab_session.snapshot.clone();

    let selected_range = {
        let mut auth = tab_session.authority.borrow_mut();
        let result = if previous {
            auth.find_replace.select_prev(&snapshot, caret)
        } else {
            auth.find_replace.select_next(&snapshot, caret)
        };
        match result {
            Ok(range) => range,
            Err(err) => {
                command_runtime.record(invocation_and_result_simple_success(
                    invocation_session,
                    "failed",
                ));
                command_runtime.note_non_command_action(format!("find navigation failed - {err}"));
                return true;
            }
        }
    };

    let Some(range) = selected_range else {
        command_runtime.record(invocation_and_result_simple_success(
            invocation_session,
            "no_op",
        ));
        command_runtime.note_non_command_action("find navigation: no matches");
        return true;
    };

    if let Some((line, grapheme)) = snapshot.line_grapheme_for_byte_offset(range.start) {
        let point = aureline_editor::TextPoint { line, grapheme };
        tab_session.viewport.set_caret(point);
        tab_session.viewport.clear_selection();
        tab_session
            .viewport
            .reveal_line(point.line, tab_session.max_scroll_line());
        let mut auth = tab_session.authority.borrow_mut();
        let _ = auth.find_replace.sync_for_view(&snapshot, point);
    }

    command_runtime.record(invocation_and_result_simple_success(
        invocation_session,
        "succeeded",
    ));
    command_runtime.note_non_command_action(if previous {
        "find previous"
    } else {
        "find next"
    });
    true
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

fn invocation_and_result_open_folder_cancelled(
    session: &CommandInvocationSession,
) -> RecordedCommandInvocation {
    invocation_and_result_simple_success(session, "cancelled_by_user")
}

fn invocation_and_result_clone_succeeded(
    session: &CommandInvocationSession,
    destination_path: &Path,
) -> RecordedCommandInvocation {
    let preview_ref = session
        .preview_posture
        .preview_record_ref
        .clone()
        .unwrap_or_else(|| mint_preview_record_ref(&session.canonical_verb));
    let journal_entry_ref = session
        .invocation_session_id
        .replacen("inv:", "journal-entry:", 1);
    let artifact_ref = format!("workspace-root:{}", destination_path.display());

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
        ],
    );

    let result = ResultBodyBlock {
        outcome_code: "succeeded".to_string(),
        warning_codes: Vec::new(),
        error_codes: Vec::new(),
        created_artifact_refs: vec![
            ArtifactRefEntry {
                result_contract_class: "preview_record_emitted_ref".to_string(),
                artifact_ref: preview_ref,
                artifact_role: "preview_record".to_string(),
            },
            ArtifactRefEntry {
                result_contract_class: "journal_entry_appended_ref".to_string(),
                artifact_ref: journal_entry_ref.clone(),
                artifact_role: "side_effect_record".to_string(),
            },
            ArtifactRefEntry {
                result_contract_class: "workspace_opened_ref".to_string(),
                artifact_ref,
                artifact_role: "workspace_root".to_string(),
            },
        ],
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
        "parity-expectation:workspace.clone_repository:result-contract:01".to_string(),
        NoBypassGuards::strict(),
    );

    RecordedCommandInvocation {
        session_packet,
        result_packet,
    }
}

fn invocation_and_result_clone_failed(
    session: &CommandInvocationSession,
    error: &CloneError,
) -> RecordedCommandInvocation {
    let preview_ref = session
        .preview_posture
        .preview_record_ref
        .clone()
        .unwrap_or_else(|| mint_preview_record_ref(&session.canonical_verb));

    let outcome = InvocationOutcomeBlock {
        outcome_class: "failed_with_typed_error".to_string(),
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
        outcome_code: "failed_with_typed_error".to_string(),
        warning_codes: Vec::new(),
        error_codes: vec![error.class.as_str().to_string()],
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
        "parity-expectation:workspace.clone_repository:result-contract:01".to_string(),
        NoBypassGuards::strict(),
    );

    RecordedCommandInvocation {
        session_packet,
        result_packet,
    }
}

fn invocation_and_result_restore_from_checkpoint(
    session: &CommandInvocationSession,
    outcome: &RestoreOutcome,
) -> RecordedCommandInvocation {
    let outcome_ref = session
        .invocation_session_id
        .replacen("inv:", "restore-outcome:", 1);
    let outcome_code = if outcome.is_empty() {
        "no_op"
    } else if outcome.manual_repair_required || !outcome.dirty_buffer_failures.is_empty() {
        "completed_with_warnings"
    } else {
        "succeeded"
    };

    let mut warning_codes = Vec::new();
    if outcome.manual_repair_required {
        warning_codes.push("manual_repair_required".to_string());
    }
    if !outcome.dirty_buffer_failures.is_empty() {
        warning_codes.push("dirty_buffer_replay_partial".to_string());
    }
    if outcome.blocked_side_effectful_count() > 0 {
        warning_codes.push("side_effectful_surfaces_restored_inactive".to_string());
    }

    let outcome_block = InvocationOutcomeBlock {
        outcome_class: outcome_code.to_string(),
        disabled_reason_code: None,
        warnings_summary_refs: warning_codes.clone(),
        partially_applied_artifact_refs: Vec::new(),
        unapplied_artifact_refs: outcome
            .dirty_buffer_failures
            .iter()
            .map(|failure| failure.journal_entry_id.clone())
            .collect(),
    };
    let session_packet = session.invocation_session_packet(
        outcome_block,
        vec![InvocationCreatedArtifactRefEntry {
            result_contract_class: "restore_outcome_ref".to_string(),
            artifact_ref: outcome_ref.clone(),
        }],
        vec![EvidenceRefEntry {
            evidence_ref_class: "restore_outcome_ref".to_string(),
            evidence_id: outcome_ref.clone(),
        }],
    );

    let result = ResultBodyBlock {
        outcome_code: outcome_code.to_string(),
        warning_codes,
        error_codes: Vec::new(),
        created_artifact_refs: vec![ArtifactRefEntry {
            result_contract_class: "restore_outcome_ref".to_string(),
            artifact_ref: outcome_ref.clone(),
            artifact_role: "restore_execution_record".to_string(),
        }],
        notification_refs: Vec::new(),
        activity_refs: Vec::new(),
        rollback_handle_ref: RollbackHandleRefBlock {
            rollback_handle_posture: "not_reversible_by_contract".to_string(),
            rollback_handle_id: None,
        },
        checkpoint_refs: Vec::new(),
        evidence_refs: vec![EvidenceRefEntry {
            evidence_ref_class: "restore_outcome_ref".to_string(),
            evidence_id: outcome_ref,
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
        "parity-expectation:workspace.restore_from_checkpoint:result-contract:01".to_string(),
        NoBypassGuards::strict(),
    );

    RecordedCommandInvocation {
        session_packet,
        result_packet,
    }
}

fn invocation_and_result_restore_from_checkpoint_failed(
    session: &CommandInvocationSession,
    error_detail: String,
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
        error_codes: vec!["restore_execution_failed".to_string(), error_detail],
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
        "parity-expectation:workspace.restore_from_checkpoint:result-contract:01".to_string(),
        NoBypassGuards::strict(),
    );

    RecordedCommandInvocation {
        session_packet,
        result_packet,
    }
}

fn invocation_and_result_docs_open_in_browser_succeeded(
    session: &CommandInvocationSession,
    browser_handoff_packet_ref: &str,
) -> RecordedCommandInvocation {
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
            result_contract_class: "browser_handoff_packet_emitted_ref".to_string(),
            artifact_ref: browser_handoff_packet_ref.to_string(),
        }],
        vec![EvidenceRefEntry {
            evidence_ref_class: "browser_handoff_packet_ref".to_string(),
            evidence_id: browser_handoff_packet_ref.to_string(),
        }],
    );

    let result = ResultBodyBlock {
        outcome_code: "succeeded".to_string(),
        warning_codes: Vec::new(),
        error_codes: Vec::new(),
        created_artifact_refs: vec![ArtifactRefEntry {
            result_contract_class: "browser_handoff_packet_emitted_ref".to_string(),
            artifact_ref: browser_handoff_packet_ref.to_string(),
            artifact_role: "browser_handoff_packet".to_string(),
        }],
        notification_refs: Vec::new(),
        activity_refs: Vec::new(),
        rollback_handle_ref: RollbackHandleRefBlock {
            rollback_handle_posture: "not_applicable_no_mutation".to_string(),
            rollback_handle_id: None,
        },
        checkpoint_refs: Vec::new(),
        evidence_refs: vec![EvidenceRefEntry {
            evidence_ref_class: "browser_handoff_packet_ref".to_string(),
            evidence_id: browser_handoff_packet_ref.to_string(),
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

fn invocation_and_result_docs_open_in_browser_failed(
    session: &CommandInvocationSession,
    browser_handoff_packet_ref: &str,
    error_detail: String,
) -> RecordedCommandInvocation {
    let outcome = InvocationOutcomeBlock {
        outcome_class: "failed_with_typed_error".to_string(),
        disabled_reason_code: None,
        warnings_summary_refs: Vec::new(),
        partially_applied_artifact_refs: Vec::new(),
        unapplied_artifact_refs: Vec::new(),
    };
    let session_packet = session.invocation_session_packet(
        outcome,
        Vec::new(),
        vec![EvidenceRefEntry {
            evidence_ref_class: "browser_handoff_packet_ref".to_string(),
            evidence_id: browser_handoff_packet_ref.to_string(),
        }],
    );

    let result = ResultBodyBlock {
        outcome_code: "failed_with_typed_error".to_string(),
        warning_codes: Vec::new(),
        error_codes: vec!["browser_handoff_launch_failed".to_string(), error_detail],
        created_artifact_refs: Vec::new(),
        notification_refs: Vec::new(),
        activity_refs: Vec::new(),
        rollback_handle_ref: RollbackHandleRefBlock {
            rollback_handle_posture: "not_applicable_no_mutation".to_string(),
            rollback_handle_id: None,
        },
        checkpoint_refs: Vec::new(),
        evidence_refs: vec![EvidenceRefEntry {
            evidence_ref_class: "browser_handoff_packet_ref".to_string(),
            evidence_id: browser_handoff_packet_ref.to_string(),
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
    registry: &CommandRegistry,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    session_restore_store: &mut SessionRestoreStore,
    palette: &mut CommandPaletteState,
    palette_focus_return: &mut FocusReturnStack<ShellFocusReturnTarget>,
    overlay: &mut Option<ShellOverlayState>,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    decision: CommandOverlayDecision,
) {
    match decision {
        CommandOverlayDecision::PreviewApproved { entry, session } => {
            match entry.descriptor.command_id.as_str() {
                "cmd:workspace.import_profile" => {
                    let review = import_review_from_argument_map(&session.argument_provenance_map);
                    record_import_review_stub(command_runtime, activity_center, &session, &review);
                }
                "cmd:workspace.clone_repository" => {
                    let err = CloneError::new(
                        CloneErrorClass::InvalidInput,
                        "clone requires a remote URL and destination path",
                    );
                    command_runtime.record(invocation_and_result_clone_failed(&session, &err));
                    command_runtime.note_non_command_action(
                        "clone requires a remote URL and destination path",
                    );
                }
                "cmd:workspace.restore_from_checkpoint" => {
                    let changed = apply_restore_from_checkpoint_preview_approved(
                        command_runtime,
                        registry,
                        frame,
                        editor_runtime,
                        session_restore_store,
                        palette,
                        palette_focus_return,
                        overlay,
                        enablement_runtime,
                        workspace_lifecycle,
                        recent_work,
                        activity_center,
                        &session,
                    );
                    if !changed {
                        command_runtime.note_non_command_action(
                            "restore approved (no restorable state found)",
                        );
                    }
                }
                _ => {
                    command_runtime.record(invocation_and_result_unimplemented(&session));
                }
            }
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

fn apply_restore_from_checkpoint_preview_approved(
    command_runtime: &mut CommandRuntimeState,
    registry: &CommandRegistry,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    session_restore_store: &mut SessionRestoreStore,
    palette: &mut CommandPaletteState,
    palette_focus_return: &mut FocusReturnStack<ShellFocusReturnTarget>,
    overlay: &mut Option<ShellOverlayState>,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    session: &CommandInvocationSession,
) -> bool {
    let _ = (registry, palette_focus_return, overlay);
    apply_restore_from_checkpoint(
        command_runtime,
        frame,
        editor_runtime,
        session_restore_store,
        palette,
        enablement_runtime,
        workspace_lifecycle,
        recent_work,
        activity_center,
        session,
    )
}

fn apply_restore_from_checkpoint(
    command_runtime: &mut CommandRuntimeState,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    session_restore_store: &mut SessionRestoreStore,
    palette: &mut CommandPaletteState,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    session: &CommandInvocationSession,
) -> bool {
    let recovery_root = PathBuf::from(".logs").join("recovery");
    let crash_journal_reader = CrashJournalStore::new(&recovery_root);
    let proposal = match RestoreProposal::build(session_restore_store, &crash_journal_reader, false)
    {
        Ok(proposal) => proposal,
        Err(err) => {
            command_runtime
                .note_non_command_action(format!("restore proposal unavailable — {err}"));
            command_runtime.record(invocation_and_result_restore_from_checkpoint_failed(
                session,
                err.to_string(),
            ));
            return false;
        }
    };

    if proposal.is_empty() {
        command_runtime.note_non_command_action(
            "restore approved but proposal is empty (no restorable state)",
        );
        let mut runtime = RestoreRuntime::new(session_restore_store, &crash_journal_reader);
        let outcome = proposal.execute(&mut runtime);
        if let Err(err) = write_restore_outcome_log(&recovery_root, &outcome) {
            command_runtime
                .note_non_command_action(format!("restore outcome log unavailable — {err}"));
        }
        command_runtime.record(invocation_and_result_restore_from_checkpoint(
            session, &outcome,
        ));
        return false;
    }

    if let Err(err) = write_restore_proposal_log(&recovery_root, &proposal) {
        command_runtime
            .note_non_command_action(format!("restore proposal log unavailable — {err}"));
    }

    let mut runtime = RestoreRuntime::new(session_restore_store, &crash_journal_reader);
    let outcome = proposal.execute(&mut runtime);
    if let Err(err) = write_restore_outcome_log(&recovery_root, &outcome) {
        command_runtime.note_non_command_action(format!("restore outcome log unavailable — {err}"));
    }
    let applied = apply_restore_outcome_to_shell(
        &outcome,
        frame,
        editor_runtime,
        palette,
        enablement_runtime,
        workspace_lifecycle,
        recent_work,
        activity_center,
        command_runtime,
    );

    command_runtime.note_non_command_action(format!(
        "restore applied (no auto-rerun) — panes={panes}; dirty_buffers={dirty}; blocked_side_effectful={blocked}; failures={failures} — {summary}",
        panes = applied.panes_opened,
        dirty = applied.dirty_buffers_replayed,
        blocked = outcome.blocked_side_effectful_count(),
        failures = outcome.dirty_buffer_failures.len(),
        summary = outcome.summary_line,
    ));
    command_runtime.record(invocation_and_result_restore_from_checkpoint(
        session, &outcome,
    ));
    true
}

#[derive(Debug, Default)]
struct RestoreShellApplySummary {
    panes_opened: usize,
    dirty_buffers_replayed: usize,
}

fn apply_restore_outcome_to_shell(
    outcome: &RestoreOutcome,
    frame: &mut DesktopFrame,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    palette: &mut CommandPaletteState,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    command_runtime: &mut CommandRuntimeState,
) -> RestoreShellApplySummary {
    let mut summary = RestoreShellApplySummary::default();
    let workspace_root = restore_workspace_root_for_outcome(outcome, recent_work, palette);
    if let Some(root) = workspace_root.as_ref() {
        ensure_restore_workspace_runtime(
            root,
            editor_runtime,
            palette,
            enablement_runtime,
            workspace_lifecycle,
            recent_work,
            activity_center,
            command_runtime,
        );
    }

    for pane in &outcome.pane_outcomes {
        if pane.execution_kind == RestorePaneExecutionKind::EvidenceOnly {
            continue;
        }
        if pane.execution_kind == RestorePaneExecutionKind::Reopened
            && outcome
                .dirty_buffer_replays
                .iter()
                .any(|replay| replay.presentation_hint.as_deref() == pane.title_hint.as_deref())
        {
            continue;
        }
        let Some(tab) = frame.open_tab() else {
            continue;
        };
        let group = frame.focused_editor_group();
        let label = pane
            .title_hint
            .clone()
            .unwrap_or_else(|| "Restored pane".to_string());
        editor_runtime.open_restore_placeholder(group, tab, label, &pane.note);
        summary.panes_opened += 1;
    }

    for replay in &outcome.dirty_buffer_replays {
        let file_path =
            restore_file_path_for_object_ref(&replay.object_ref, workspace_root.as_ref());
        let file_path = file_path.as_deref().filter(|path| path.exists());
        let Some(tab) = frame.open_tab() else {
            continue;
        };
        let group = frame.focused_editor_group();
        match editor_runtime.open_recovered_dirty_buffer(group, tab, replay, file_path) {
            Ok(()) => {
                summary.dirty_buffers_replayed += 1;
            }
            Err(err) => {
                command_runtime.note_non_command_action(format!(
                    "dirty-buffer restore failed for {} — {err}",
                    replay.object_ref
                ));
            }
        }
    }

    summary
}

fn restore_workspace_root_for_outcome(
    outcome: &RestoreOutcome,
    recent_work: &RecentWorkRuntimeState,
    palette: &CommandPaletteState,
) -> Option<PathBuf> {
    if let Some(root) = palette.workspace_root() {
        return Some(root.to_path_buf());
    }

    if let Some(active_id) = recent_work.active_recent_work_id.as_deref() {
        if let Some(root) = recent_work
            .find_entry(active_id)
            .and_then(workspace_root_for_recent_work_entry)
        {
            return Some(root);
        }
    }

    if let Some(root) = outcome
        .dirty_buffer_replays
        .iter()
        .find_map(|replay| restore_file_path_for_object_ref(&replay.object_ref, None))
        .and_then(|path| path.parent().map(Path::to_path_buf))
    {
        return Some(root);
    }

    recent_work
        .registry
        .entries
        .iter()
        .filter(|entry| matches!(entry.target_kind, TargetKind::LocalFolder))
        .max_by(|left, right| left.last_opened_at.cmp(&right.last_opened_at))
        .and_then(workspace_root_for_recent_work_entry)
}

fn ensure_restore_workspace_runtime(
    root: &Path,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    palette: &mut CommandPaletteState,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    command_runtime: &mut CommandRuntimeState,
) {
    let entry = recent_work
        .registry
        .entries
        .iter()
        .find(|entry| {
            workspace_root_for_recent_work_entry(entry)
                .as_deref()
                .is_some_and(|candidate| candidate == root)
        })
        .cloned();

    let trust_state = entry
        .as_ref()
        .map(|entry| entry.trust_state)
        .unwrap_or_else(|| {
            trust_state_for_recent_work(enablement_runtime.workspace_trust_state.as_str())
        });

    if let Some(entry) = entry {
        recent_work.active_recent_work_id = Some(entry.recent_work_id.clone());
        recent_work.active_workspace_label = Some(entry.presentation_label.clone());
        editor_runtime.set_workspace_recovery_ref(entry.recent_work_id);
    }

    palette.set_workspace_root(root.to_path_buf());
    let identity_key = root.to_string_lossy();
    let workspace_id = workspace_id_for_local_folder(&identity_key);
    if recent_work.active_recent_work_id.is_none() {
        editor_runtime.set_workspace_recovery_ref(workspace_id.clone());
    }
    if let Err(err) = editor_runtime
        .explorer
        .open_workspace(root.to_path_buf(), workspace_id.clone())
    {
        command_runtime.note_non_command_action(format!("explorer unavailable — {err}"));
    }
    workspace_lifecycle.open_local_folder(workspace_id.clone(), trust_state);
    editor_runtime.terminal_pane.open_workspace(
        workspace_id,
        root.to_path_buf(),
        trust_state,
        &mono_timestamp_now(),
    );
    activity_center.note_workspace_opened(
        recent_work.active_recent_work_id.as_deref(),
        recent_work.active_workspace_label().unwrap_or("workspace"),
        root,
    );
    activity_center.note_quick_open_file_index(
        recent_work.active_recent_work_id.as_deref(),
        root,
        false,
    );
}

fn restore_file_path_for_object_ref(
    object_ref: &str,
    workspace_root: Option<&PathBuf>,
) -> Option<PathBuf> {
    let uri = VfsUri::parse(object_ref.to_string()).ok()?;
    if let Some(path) = uri.file_path() {
        return Some(path);
    }
    if uri.scheme() != "aureline-ws" {
        return None;
    }

    let root = workspace_root?;
    let hierarchical = uri.split_hierarchical()?;
    let logical_path = hierarchical.path.trim_start_matches('/');
    let relative = logical_path
        .split_once('/')
        .map(|(_, relative)| relative)
        .unwrap_or("");
    if relative.is_empty() {
        Some(root.clone())
    } else {
        Some(root.join(relative))
    }
}

fn normalize_entry_pin_actions(entry: &mut RecentWorkEntryRecord) {
    if entry.pinned {
        entry
            .safe_recovery_actions
            .retain(|action| *action != SafeRecoveryAction::Pin);
        if !entry
            .safe_recovery_actions
            .contains(&SafeRecoveryAction::Unpin)
        {
            entry.safe_recovery_actions.push(SafeRecoveryAction::Unpin);
        }
    } else {
        entry
            .safe_recovery_actions
            .retain(|action| *action != SafeRecoveryAction::Unpin);
        if !entry
            .safe_recovery_actions
            .contains(&SafeRecoveryAction::Pin)
        {
            entry.safe_recovery_actions.push(SafeRecoveryAction::Pin);
        }
    }
}

fn activate_recent_work_entry(
    window_dims: (u32, u32),
    workspace_trust_state: &mut String,
    recent_work: &mut RecentWorkRuntimeState,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    frame: &mut DesktopFrame,
    command_runtime: &mut CommandRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    mut entry: RecentWorkEntryRecord,
    trust_override: Option<TrustState>,
) {
    recent_work.suspend_active_frame(frame);
    recent_work.activate_recent_work(frame, window_dims, &entry);
    editor_runtime.set_workspace_recovery_ref(entry.recent_work_id.clone());

    let trust_state = trust_override.unwrap_or(entry.trust_state);
    *workspace_trust_state = trust_state.as_str().to_string();
    if let Some(root) = workspace_root_for_recent_work_entry(&entry) {
        let identity_key = root.to_string_lossy();
        let workspace_id = workspace_id_for_local_folder(&identity_key);
        if let Err(err) = editor_runtime
            .explorer
            .open_workspace(root.clone(), workspace_id.clone())
        {
            command_runtime.note_non_command_action(format!("explorer unavailable — {err}"));
        }
        editor_runtime.terminal_pane.open_workspace(
            workspace_id,
            root,
            trust_state,
            &mono_timestamp_now(),
        );
    } else {
        editor_runtime.explorer.clear_workspace();
        editor_runtime
            .terminal_pane
            .close_active_workspace(&mono_timestamp_now(), Some("workspace_closed"));
    }

    let opened_at = mono_timestamp_now();
    entry.last_opened_at = opened_at.clone();
    normalize_recent_work_entry_recovery_actions(&mut entry);
    recent_work.registry.updated_at = opened_at;
    recent_work.registry.upsert(entry);

    if let Err(err) = recent_work.save() {
        command_runtime.note_non_command_action(format!("recent work save failed — {err}"));
    } else {
        command_runtime.note_non_command_action("workspace switch applied");
    }

    if let Some(active_id) = recent_work.active_recent_work_id.as_deref() {
        if let Some(root) = recent_work
            .find_entry(active_id)
            .and_then(workspace_root_for_recent_work_entry)
        {
            activity_center.note_workspace_opened(
                Some(active_id),
                recent_work.active_workspace_label().unwrap_or("workspace"),
                &root,
            );
            activity_center.note_quick_open_file_index(Some(active_id), &root, false);
        }
    }
}

fn workspace_root_for_recent_work_entry(entry: &RecentWorkEntryRecord) -> Option<PathBuf> {
    if entry.target_state != RecentWorkTargetState::Reachable {
        return None;
    }
    match entry.target_kind {
        TargetKind::LocalFolder | TargetKind::LocalRepoRoot => {
            entry.presentation_subtitle.as_deref().map(PathBuf::from)
        }
        _ => None,
    }
}

fn sync_workspace_activation_runtime(
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    palette: &mut CommandPaletteState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    workspace_trust_state: &str,
    workspace_root: Option<PathBuf>,
) {
    let Some(root) = workspace_root else {
        editor_runtime.explorer.clear_workspace();
        workspace_lifecycle.machine = None;
        workspace_lifecycle.last_logged = None;
        return;
    };

    palette.set_workspace_root(root.clone());
    let identity_key = root.to_string_lossy();
    let workspace_id = workspace_id_for_local_folder(&identity_key);
    if let Err(err) = editor_runtime
        .explorer
        .open_workspace(root, workspace_id.clone())
    {
        editor_runtime.explorer.last_error = Some(err);
    }
    workspace_lifecycle.open_local_folder(
        workspace_id,
        trust_state_for_recent_work(workspace_trust_state),
    );
}

fn apply_workspace_switcher_decision(
    window_size: &LogicalSize<u32>,
    workspace_trust_state: &mut String,
    recent_work: &mut RecentWorkRuntimeState,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    frame: &mut DesktopFrame,
    command_runtime: &mut CommandRuntimeState,
    palette: &mut CommandPaletteState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    decision: WorkspaceSwitcherDecision,
) {
    let window_dims = (window_size.width, window_size.height);

    match decision {
        WorkspaceSwitcherDecision::Activate { recent_work_id } => {
            let Some(entry) = recent_work.find_entry(&recent_work_id).cloned() else {
                command_runtime.note_non_command_action(format!(
                    "workspace switch failed: missing recent-work id {recent_work_id}"
                ));
                return;
            };
            let workspace_root = workspace_root_for_recent_work_entry(&entry);

            let preferred_open = [
                SafeRecoveryAction::Open,
                SafeRecoveryAction::OpenWithoutRestore,
                SafeRecoveryAction::OpenReadOnlyCachedView,
                SafeRecoveryAction::OpenRestricted,
                SafeRecoveryAction::OpenInNewWindow,
            ]
            .into_iter()
            .find(|candidate| entry.safe_recovery_actions.contains(candidate));

            match preferred_open {
                Some(SafeRecoveryAction::OpenRestricted) => {
                    activate_recent_work_entry(
                        window_dims,
                        workspace_trust_state,
                        recent_work,
                        editor_runtime,
                        frame,
                        command_runtime,
                        activity_center,
                        entry,
                        Some(TrustState::Restricted),
                    );
                    sync_workspace_activation_runtime(
                        editor_runtime,
                        palette,
                        workspace_lifecycle,
                        workspace_trust_state.as_str(),
                        workspace_root,
                    );
                }
                Some(SafeRecoveryAction::OpenInNewWindow) => {
                    command_runtime.note_non_command_action(
                        "open in new window not implemented; opening in current window",
                    );
                    activate_recent_work_entry(
                        window_dims,
                        workspace_trust_state,
                        recent_work,
                        editor_runtime,
                        frame,
                        command_runtime,
                        activity_center,
                        entry,
                        None,
                    );
                    sync_workspace_activation_runtime(
                        editor_runtime,
                        palette,
                        workspace_lifecycle,
                        workspace_trust_state.as_str(),
                        workspace_root,
                    );
                }
                Some(_) => {
                    activate_recent_work_entry(
                        window_dims,
                        workspace_trust_state,
                        recent_work,
                        editor_runtime,
                        frame,
                        command_runtime,
                        activity_center,
                        entry,
                        None,
                    );
                    sync_workspace_activation_runtime(
                        editor_runtime,
                        palette,
                        workspace_lifecycle,
                        workspace_trust_state.as_str(),
                        workspace_root,
                    );
                }
                None => command_runtime.note_non_command_action(format!(
                    "workspace switch blocked: no open action available ({recent_work_id})"
                )),
            }
        }
        WorkspaceSwitcherDecision::SetPinned {
            recent_work_id,
            pinned,
        } => {
            let row = recent_work
                .registry
                .entries
                .iter_mut()
                .find(|row| row.recent_work_id == recent_work_id);
            let Some(row) = row else {
                command_runtime.note_non_command_action(format!(
                    "pin update failed: missing recent-work id {recent_work_id}"
                ));
                return;
            };

            row.pinned = pinned;
            normalize_entry_pin_actions(row);
            recent_work.registry.updated_at = mono_timestamp_now();
            if let Err(err) = recent_work.save() {
                command_runtime.note_non_command_action(format!("recent work save failed — {err}"));
            } else {
                command_runtime.note_non_command_action(if pinned {
                    "pinned recent work"
                } else {
                    "unpinned recent work"
                });
            }
        }
        WorkspaceSwitcherDecision::Remove { recent_work_id } => {
            if recent_work.registry.remove(&recent_work_id) {
                recent_work.registry.updated_at = mono_timestamp_now();
                if let Err(err) = recent_work.save() {
                    command_runtime.note_non_command_action(format!(
                        "recent work remove saved failed — {err}"
                    ));
                } else {
                    command_runtime.note_non_command_action("removed recent work entry");
                }
            } else {
                command_runtime.note_non_command_action(format!(
                    "remove failed: missing recent-work id {recent_work_id}"
                ));
            }
        }
        WorkspaceSwitcherDecision::PerformRecoveryAction {
            recent_work_id,
            action,
        } => match action {
            SafeRecoveryAction::Open | SafeRecoveryAction::OpenWithoutRestore => {
                let Some(entry) = recent_work.find_entry(&recent_work_id).cloned() else {
                    command_runtime.note_non_command_action(format!(
                        "workspace switch failed: missing recent-work id {recent_work_id}"
                    ));
                    return;
                };
                let workspace_root = workspace_root_for_recent_work_entry(&entry);
                activate_recent_work_entry(
                    window_dims,
                    workspace_trust_state,
                    recent_work,
                    editor_runtime,
                    frame,
                    command_runtime,
                    activity_center,
                    entry,
                    None,
                );
                sync_workspace_activation_runtime(
                    editor_runtime,
                    palette,
                    workspace_lifecycle,
                    workspace_trust_state.as_str(),
                    workspace_root,
                );
            }
            SafeRecoveryAction::OpenReadOnlyCachedView => {
                command_runtime.note_non_command_action(
                    "read-only cached view not implemented; opening current placeholder workspace",
                );
                let Some(entry) = recent_work.find_entry(&recent_work_id).cloned() else {
                    command_runtime.note_non_command_action(format!(
                        "workspace switch failed: missing recent-work id {recent_work_id}"
                    ));
                    return;
                };
                let workspace_root = workspace_root_for_recent_work_entry(&entry);
                activate_recent_work_entry(
                    window_dims,
                    workspace_trust_state,
                    recent_work,
                    editor_runtime,
                    frame,
                    command_runtime,
                    activity_center,
                    entry,
                    None,
                );
                sync_workspace_activation_runtime(
                    editor_runtime,
                    palette,
                    workspace_lifecycle,
                    workspace_trust_state.as_str(),
                    workspace_root,
                );
            }
            SafeRecoveryAction::OpenRestricted => {
                let Some(entry) = recent_work.find_entry(&recent_work_id).cloned() else {
                    command_runtime.note_non_command_action(format!(
                        "workspace switch failed: missing recent-work id {recent_work_id}"
                    ));
                    return;
                };
                let workspace_root = workspace_root_for_recent_work_entry(&entry);
                activate_recent_work_entry(
                    window_dims,
                    workspace_trust_state,
                    recent_work,
                    editor_runtime,
                    frame,
                    command_runtime,
                    activity_center,
                    entry,
                    Some(TrustState::Restricted),
                );
                sync_workspace_activation_runtime(
                    editor_runtime,
                    palette,
                    workspace_lifecycle,
                    workspace_trust_state.as_str(),
                    workspace_root,
                );
            }
            SafeRecoveryAction::OpenInNewWindow => {
                command_runtime.note_non_command_action(
                    "open in new window not implemented; opening in current window",
                );
                let Some(entry) = recent_work.find_entry(&recent_work_id).cloned() else {
                    command_runtime.note_non_command_action(format!(
                        "workspace switch failed: missing recent-work id {recent_work_id}"
                    ));
                    return;
                };
                let workspace_root = workspace_root_for_recent_work_entry(&entry);
                activate_recent_work_entry(
                    window_dims,
                    workspace_trust_state,
                    recent_work,
                    editor_runtime,
                    frame,
                    command_runtime,
                    activity_center,
                    entry,
                    None,
                );
                sync_workspace_activation_runtime(
                    editor_runtime,
                    palette,
                    workspace_lifecycle,
                    workspace_trust_state.as_str(),
                    workspace_root,
                );
            }
            SafeRecoveryAction::Pin => {
                let row = recent_work
                    .registry
                    .entries
                    .iter_mut()
                    .find(|row| row.recent_work_id == recent_work_id);
                let Some(row) = row else {
                    command_runtime.note_non_command_action(format!(
                        "pin update failed: missing recent-work id {recent_work_id}"
                    ));
                    return;
                };

                row.pinned = true;
                normalize_entry_pin_actions(row);
                recent_work.registry.updated_at = mono_timestamp_now();
                if let Err(err) = recent_work.save() {
                    command_runtime
                        .note_non_command_action(format!("recent work save failed — {err}"));
                } else {
                    command_runtime.note_non_command_action("pinned recent work");
                }
            }
            SafeRecoveryAction::Unpin => {
                let row = recent_work
                    .registry
                    .entries
                    .iter_mut()
                    .find(|row| row.recent_work_id == recent_work_id);
                let Some(row) = row else {
                    command_runtime.note_non_command_action(format!(
                        "pin update failed: missing recent-work id {recent_work_id}"
                    ));
                    return;
                };

                row.pinned = false;
                normalize_entry_pin_actions(row);
                recent_work.registry.updated_at = mono_timestamp_now();
                if let Err(err) = recent_work.save() {
                    command_runtime
                        .note_non_command_action(format!("recent work save failed — {err}"));
                } else {
                    command_runtime.note_non_command_action("unpinned recent work");
                }
            }
            SafeRecoveryAction::RemoveFromRecents => {
                if recent_work.registry.remove(&recent_work_id) {
                    recent_work.registry.updated_at = mono_timestamp_now();
                    if let Err(err) = recent_work.save() {
                        command_runtime.note_non_command_action(format!(
                            "recent work remove saved failed — {err}"
                        ));
                    } else {
                        command_runtime.note_non_command_action("removed recent work entry");
                    }
                } else {
                    command_runtime.note_non_command_action(format!(
                        "remove failed: missing recent-work id {recent_work_id}"
                    ));
                }
            }
            SafeRecoveryAction::LocateMissingTarget => {
                let Some(mut entry) = recent_work.find_entry(&recent_work_id).cloned() else {
                    command_runtime.note_non_command_action(format!(
                        "locate failed: missing recent-work id {recent_work_id}"
                    ));
                    return;
                };
                let selected_path = match folder_picker::pick_folder() {
                    Some(path) => path,
                    None => {
                        command_runtime.note_non_command_action("locate target cancelled");
                        return;
                    }
                };
                let folder_path = match resolve_open_workspace_path(&selected_path) {
                    Ok(path) => path,
                    Err(err) => {
                        command_runtime
                            .note_non_command_action(format!("locate target failed — {err}"));
                        return;
                    }
                };

                entry.presentation_subtitle = Some(folder_path.display().to_string());
                entry.target_state = RecentWorkTargetState::Reachable;
                if !entry
                    .safe_recovery_actions
                    .contains(&SafeRecoveryAction::Open)
                {
                    entry.safe_recovery_actions.push(SafeRecoveryAction::Open);
                }
                normalize_recent_work_entry_recovery_actions(&mut entry);
                let workspace_root = workspace_root_for_recent_work_entry(&entry);
                activate_recent_work_entry(
                    window_dims,
                    workspace_trust_state,
                    recent_work,
                    editor_runtime,
                    frame,
                    command_runtime,
                    activity_center,
                    entry,
                    None,
                );
                sync_workspace_activation_runtime(
                    editor_runtime,
                    palette,
                    workspace_lifecycle,
                    workspace_trust_state.as_str(),
                    workspace_root,
                );
                command_runtime.note_non_command_action("located recent work target");
            }
            SafeRecoveryAction::Reconnect => command_runtime
                .note_non_command_action("reconnect queued; recent-work entry preserved"),
            SafeRecoveryAction::Reauth => command_runtime
                .note_non_command_action("reauth required; recent-work entry preserved"),
            SafeRecoveryAction::RetryLater => {
                command_runtime.note_non_command_action("retry scheduled (placeholder)")
            }
            SafeRecoveryAction::CompareBeforeRestore => command_runtime
                .note_non_command_action("compare-before-restore not implemented in shell build"),
            SafeRecoveryAction::RevealInExplorer => command_runtime
                .note_non_command_action("reveal in explorer not implemented in shell build"),
        },
    }
}

fn open_start_center_entry_flow_sheet(
    frame: &mut DesktopFrame,
    overlay: &mut Option<ShellOverlayState>,
    row: &crate::start_center::StartCenterActionRow,
) {
    let (entry_verb, target_kind, preferred_resulting_mode, degraded_token, note) =
        start_center_flow_tuple(row.action_id);
    let outcome = resolve_entry_flow(EntryFlowRequest {
        entry_verb,
        target: EntryFlowTarget::ExplicitTargetKind(target_kind),
        preferred_resulting_mode: Some(preferred_resulting_mode),
    });

    *overlay = Some(ShellOverlayState::entry_flow_sheet(
        frame.focused_zone(),
        frame.focused_editor_group(),
        outcome,
        row.command_id.to_string(),
        DispatchOrigin::StartCenter,
        row.argument_provenance_map.clone(),
        degraded_token,
        note,
    ));
    frame.focus_zone(ShellZoneId::TransientOverlay);
}

fn start_center_flow_tuple(
    action_id: StartCenterPrimaryActionId,
) -> (
    EntryVerb,
    TargetKind,
    ResultingMode,
    Option<DegradedStateToken>,
    Option<String>,
) {
    match action_id {
        StartCenterPrimaryActionId::OpenFolder => (
            EntryVerb::Open,
            TargetKind::LocalFolder,
            ResultingMode::Folder,
            None,
            None,
        ),
        StartCenterPrimaryActionId::OpenWorkspace => (
            EntryVerb::Open,
            TargetKind::WorkspaceManifest,
            ResultingMode::WorkspaceWithRoots,
            Some(DegradedStateToken::Unsupported),
            Some("Workspace-file selection is not implemented yet.".to_string()),
        ),
        StartCenterPrimaryActionId::CloneRepository => (
            EntryVerb::Clone,
            TargetKind::RemoteRepository,
            ResultingMode::CloneThenReview,
            None,
            None,
        ),
        StartCenterPrimaryActionId::RestoreLastSession => (
            EntryVerb::Restore,
            TargetKind::RecoveryCheckpoint,
            ResultingMode::RestoreLastSession,
            None,
            None,
        ),
        StartCenterPrimaryActionId::ImportFrom => (
            EntryVerb::Import,
            TargetKind::CompetitorConfigRoot,
            ResultingMode::ExtractThenReview,
            Some(DegradedStateToken::Labs),
            Some(
                "Review is available; apply records the import plan without changing settings."
                    .to_string(),
            ),
        ),
    }
}

fn entry_flow_admission_packet(
    origin: DispatchOrigin,
    outcome: &EntryFlowOutcome,
    argument_provenance_map: &[ArgumentProvenanceEntry],
) -> Option<AdmissionReviewPacket> {
    let EntryFlowOutcome::Resolved(resolved) = outcome else {
        return None;
    };
    let target_specifier = admission_target_specifier_for(
        resolved.entry_verb,
        resolved.target_kind,
        argument_provenance_map,
    );
    let destination = admission_destination_for(resolved.entry_verb, argument_provenance_map);
    Some(admission_packet_for_resolved_entry(
        admission_source_surface_for(origin),
        resolved.entry_verb,
        resolved.target_kind,
        resolved.resulting_mode,
        target_specifier,
        destination,
    ))
}

fn admission_source_surface_for(origin: DispatchOrigin) -> AdmissionSourceSurface {
    match origin {
        DispatchOrigin::StartCenter => AdmissionSourceSurface::StartCenter,
        DispatchOrigin::CommandPalette | DispatchOrigin::KeybindingChord => {
            AdmissionSourceSurface::CommandPalette
        }
    }
}

fn admission_target_specifier_for(
    entry_verb: EntryVerb,
    target_kind: TargetKind,
    argument_provenance_map: &[ArgumentProvenanceEntry],
) -> String {
    match entry_verb {
        EntryVerb::Clone => {
            clone_argument_value(argument_provenance_map, "remote_repository_ref", "git-url:")
                .unwrap_or_else(|| target_kind.as_str().to_string())
        }
        EntryVerb::Import => import_source_path_from_argument_map(argument_provenance_map)
            .or_else(|| argument_resolved_value(argument_provenance_map, "import_source_ref"))
            .unwrap_or_else(|| target_kind.as_str().to_string()),
        EntryVerb::Open | EntryVerb::AddRoot => {
            argument_resolved_value(argument_provenance_map, "workspace_scope_ref")
                .unwrap_or_else(|| target_kind.as_str().to_string())
        }
        EntryVerb::Restore | EntryVerb::Resume | EntryVerb::StartFromSnapshot => {
            argument_resolved_value(argument_provenance_map, "checkpoint_ref")
                .unwrap_or_else(|| target_kind.as_str().to_string())
        }
    }
}

fn admission_destination_for(
    entry_verb: EntryVerb,
    argument_provenance_map: &[ArgumentProvenanceEntry],
) -> Option<String> {
    match entry_verb {
        EntryVerb::Clone => {
            clone_argument_value(argument_provenance_map, "destination_root_ref", "path:")
        }
        EntryVerb::Import => import_destination_from_argument_map(argument_provenance_map),
        EntryVerb::AddRoot => {
            argument_resolved_value(argument_provenance_map, "active_workspace_ref")
        }
        EntryVerb::Open | EntryVerb::Restore | EntryVerb::Resume | EntryVerb::StartFromSnapshot => {
            None
        }
    }
}

fn clone_request_from_argument_map(
    argument_provenance_map: &[ArgumentProvenanceEntry],
) -> Result<CloneRequest, CloneError> {
    let remote_url =
        clone_argument_value(argument_provenance_map, "remote_repository_ref", "git-url:")
            .ok_or_else(|| {
                CloneError::new(CloneErrorClass::InvalidInput, "remote URL is required")
            })?;
    let destination_path =
        clone_argument_value(argument_provenance_map, "destination_root_ref", "path:").ok_or_else(
            || {
                CloneError::new(
                    CloneErrorClass::InvalidInput,
                    "destination path is required",
                )
            },
        )?;
    let expanded_destination = expand_tilde(&destination_path);
    let packet = clone_form_admission_packet(
        remote_url.as_str(),
        expanded_destination.display().to_string(),
    );
    if let Some(collision) = packet.collision_review.as_ref() {
        if collision.requires_explicit_choice {
            let actions = collision
                .safe_actions
                .iter()
                .map(|action| action.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            return Err(CloneError::new(
                CloneErrorClass::DestinationExists,
                format!(
                    "{}; choose one of: {actions}. Typed inputs preserved; diagnostics redacted.",
                    collision.summary
                ),
            ));
        }
    }
    let request = CloneRequest::new(remote_url, expanded_destination);
    request.validate()?;
    Ok(request)
}

fn clone_argument_value(
    argument_provenance_map: &[ArgumentProvenanceEntry],
    argument_name: &str,
    prefix: &str,
) -> Option<String> {
    let raw = argument_provenance_map
        .iter()
        .find(|row| row.argument_name == argument_name)?
        .resolved_value_ref
        .as_deref()?;
    raw.strip_prefix(prefix)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn submit_clone_from_entry_flow(
    command_runtime: &mut CommandRuntimeState,
    registry: &CommandRegistry,
    frame: &DesktopFrame,
    overlay: &mut Option<ShellOverlayState>,
    clone_jobs: &mut CloneJobRuntimeState,
    enablement_runtime: &CommandEnablementRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    decision: EntryFlowOverlayDecision,
) -> bool {
    let Some(entry) = registry.get(&decision.command_id).cloned() else {
        set_clone_sheet_status(
            overlay,
            "clone command is missing from the command registry".to_string(),
            false,
        );
        return true;
    };

    let mut session = make_session(
        frame,
        &entry,
        decision.origin,
        "apply_after_preview",
        enablement_runtime.workspace_trust_state.as_str(),
        decision.argument_provenance_map,
        true,
        Some(mint_preview_record_ref(&entry.descriptor.canonical_verb)),
        "not_required",
        None,
    );

    let enablement_context = CommandEnablementContext {
        client_scope: "desktop_product".to_string(),
        workspace_trust_state: enablement_runtime.workspace_trust_state.clone(),
        execution_context_available: enablement_runtime.execution_context_available,
        provider_linked: enablement_runtime.provider_linked,
        credential_available: enablement_runtime.credential_available,
        policy_disabled: enablement_runtime.policy_disabled,
        policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
        labs_enabled: enablement_runtime.labs_enabled,
        argument_provenance_map: session.argument_provenance_map.clone(),
    };
    let preflight = entry.preflight(&enablement_context);
    let enablement_snapshot = preflight.enablement_snapshot.clone();
    session.enablement_decision = EnablementDecisionBlock {
        decision_class: enablement_snapshot.decision_class,
        disabled_reason_code: enablement_snapshot.disabled_reason_code,
        repair_hook_ref: enablement_snapshot.repair_hook_ref,
    };

    if matches!(
        preflight.decision_class,
        PreflightDecisionClass::BlockedByPolicy | PreflightDecisionClass::DisabledWithReason
    ) {
        let denied_code = session
            .enablement_decision
            .disabled_reason_code
            .unwrap_or(DisabledReasonCode::PolicyBlockedInContext);
        command_runtime.record(invocation_and_result_denied(&session, denied_code));
        set_clone_sheet_status(
            overlay,
            format!("clone denied: {}", denied_code.as_str()),
            false,
        );
        return true;
    }

    let request = match clone_request_from_argument_map(&session.argument_provenance_map) {
        Ok(request) => request,
        Err(err) => {
            command_runtime.record(invocation_and_result_clone_failed(&session, &err));
            set_clone_sheet_status(
                overlay,
                format!("{}: {}", err.class.as_str(), err.message),
                false,
            );
            return true;
        }
    };
    let admission_packet = clone_form_admission_packet(
        request.remote_url.as_str(),
        request.destination_path.display().to_string(),
    );
    record_admission_review_packet(command_runtime, &admission_packet);

    match clone_jobs.start(request.clone(), session.clone()) {
        Ok(operation_id) => {
            activity_center.note_clone_running(
                &operation_id,
                &request.remote_url,
                &request.destination_path,
                Some("Starting clone".to_string()),
            );
            command_runtime.note_non_command_action(format!(
                "clone started - {}",
                request.destination_path.display()
            ));
            set_clone_sheet_status(overlay, "clone running".to_string(), true);
        }
        Err(err) => {
            let operation_id = clone_operation_id_for(&request, "failed");
            activity_center.note_clone_failed(
                &operation_id,
                &request.remote_url,
                &request.destination_path,
                &err,
            );
            command_runtime.record(invocation_and_result_clone_failed(&session, &err));
            command_runtime.note_non_command_action(format!(
                "clone failed - {} ({})",
                err.class.as_str(),
                err.message
            ));
            set_clone_sheet_status(
                overlay,
                format!("{}: {}", err.class.as_str(), err.message),
                false,
            );
        }
    }
    true
}

fn submit_import_from_entry_flow(
    command_runtime: &mut CommandRuntimeState,
    registry: &CommandRegistry,
    frame: &DesktopFrame,
    overlay: &mut Option<ShellOverlayState>,
    enablement_runtime: &CommandEnablementRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    decision: EntryFlowOverlayDecision,
) -> bool {
    let Some(entry) = registry.get(&decision.command_id).cloned() else {
        set_import_sheet_status(
            overlay,
            None,
            "import command is missing from the command registry".to_string(),
            false,
        );
        return true;
    };

    let approval_ticket_ref = if entry.descriptor.approval_posture_class != "no_approval_required" {
        Some(mint_approval_ticket_ref(&entry.descriptor.canonical_verb))
    } else {
        None
    };
    let mut session = make_session(
        frame,
        &entry,
        decision.origin,
        "apply_after_preview",
        enablement_runtime.workspace_trust_state.as_str(),
        decision.argument_provenance_map,
        true,
        Some(mint_preview_record_ref(&entry.descriptor.canonical_verb)),
        "approval_granted",
        approval_ticket_ref,
    );

    let enablement_context = CommandEnablementContext {
        client_scope: "desktop_product".to_string(),
        workspace_trust_state: enablement_runtime.workspace_trust_state.clone(),
        execution_context_available: enablement_runtime.execution_context_available,
        provider_linked: enablement_runtime.provider_linked,
        credential_available: enablement_runtime.credential_available,
        policy_disabled: enablement_runtime.policy_disabled,
        policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
        labs_enabled: enablement_runtime.labs_enabled,
        argument_provenance_map: session.argument_provenance_map.clone(),
    };
    let preflight = entry.preflight(&enablement_context);
    let enablement_snapshot = preflight.enablement_snapshot.clone();
    session.enablement_decision = EnablementDecisionBlock {
        decision_class: enablement_snapshot.decision_class,
        disabled_reason_code: enablement_snapshot.disabled_reason_code,
        repair_hook_ref: enablement_snapshot.repair_hook_ref,
    };

    if matches!(
        preflight.decision_class,
        PreflightDecisionClass::BlockedByPolicy | PreflightDecisionClass::DisabledWithReason
    ) {
        let denied_code = session
            .enablement_decision
            .disabled_reason_code
            .unwrap_or(DisabledReasonCode::PolicyBlockedInContext);
        command_runtime.record(invocation_and_result_denied(&session, denied_code));
        set_import_sheet_status(
            overlay,
            None,
            format!("import denied: {}", denied_code.as_str()),
            false,
        );
        return true;
    }

    let review = import_review_from_argument_map(&session.argument_provenance_map);
    if let Some(admission_packet) =
        import_admission_packet_from_argument_map(&session.argument_provenance_map)
    {
        record_admission_review_packet(command_runtime, &admission_packet);
    }
    record_import_review_stub(command_runtime, activity_center, &session, &review);
    let status = match review.decision_class {
        ImportReviewDecisionClass::ApplyAfterPreview => {
            let packet = materialize_import_diff_review_packet(&review);
            format!(
                "diff preview and rollback checkpoint recorded: {} / {}",
                packet.import_diff_preview_ref, packet.rollback_checkpoint.checkpoint_ref
            )
        }
        ImportReviewDecisionClass::Decline => {
            format!("import declined: {}", review.import_review_id)
        }
        ImportReviewDecisionClass::Defer => {
            format!("import deferred: {}", review.status_line)
        }
    };
    set_import_sheet_status(overlay, Some(review), status, true);
    true
}

fn record_import_review_stub(
    command_runtime: &mut CommandRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    session: &CommandInvocationSession,
    review: &ImportReviewRecord,
) {
    write_import_review_log(review);
    let diff_review = materialize_import_diff_review_packet(review);
    write_import_diff_review_log(&diff_review);
    command_runtime.record(invocation_and_result_import_profile_review_recorded(
        session,
        review,
        &diff_review,
    ));
    activity_center.note_import_review_recorded(review);
    command_runtime.note_non_command_action(format!(
        "import diff review recorded - {} ({}) report={}",
        review.classification.variant_name(),
        review.decision_class.as_str(),
        diff_review.retained_migration_report.migration_report_id
    ));
}

fn record_admission_review_packet(
    command_runtime: &mut CommandRuntimeState,
    packet: &AdmissionReviewPacket,
) {
    let filename = format!(
        "{}.json",
        sanitize_activity_id_component(&packet.admission_review_id)
    );
    let path = PathBuf::from(".logs")
        .join("admission_reviews")
        .join(filename);
    match write_admission_review_log(packet, path) {
        Ok(()) => command_runtime.note_non_command_action(format!(
            "admission review recorded - {} {} {}",
            packet.entry_verb.as_str(),
            packet.target_kind.as_str(),
            packet.write_scope.write_scope_class.as_str()
        )),
        Err(err) => {
            command_runtime.note_non_command_action(format!("admission review log failed - {err}"))
        }
    }
}

fn import_review_from_argument_map(
    argument_provenance_map: &[ArgumentProvenanceEntry],
) -> ImportReviewRecord {
    let destination = import_destination_from_argument_map(argument_provenance_map)
        .unwrap_or_else(|| "profile:default".to_string());
    if let Some(source_path) = import_source_path_from_argument_map(argument_provenance_map) {
        return CompetitorConfigClassifier::new()
            .build_review(expand_tilde(&source_path), destination);
    }
    let source_ref = argument_resolved_value(argument_provenance_map, "import_source_ref")
        .unwrap_or_else(|| "import-source:unresolved".to_string());
    ImportReviewRecord::deferred_unresolved_source(source_ref, destination)
}

fn import_admission_packet_from_argument_map(
    argument_provenance_map: &[ArgumentProvenanceEntry],
) -> Option<AdmissionReviewPacket> {
    let source_path = import_source_path_from_argument_map(argument_provenance_map)
        .or_else(|| argument_resolved_value(argument_provenance_map, "import_source_ref"))?;
    let destination = import_destination_from_argument_map(argument_provenance_map)
        .unwrap_or_else(|| "labelled import staging".to_string());
    Some(import_form_admission_packet(source_path, destination))
}

fn import_source_path_from_argument_map(
    argument_provenance_map: &[ArgumentProvenanceEntry],
) -> Option<String> {
    import_argument_value(argument_provenance_map, "import_source_ref", "path:")
}

fn import_destination_from_argument_map(
    argument_provenance_map: &[ArgumentProvenanceEntry],
) -> Option<String> {
    import_argument_value(
        argument_provenance_map,
        "destination_workspace_target_ref",
        "workspace-target:",
    )
}

fn import_argument_value(
    argument_provenance_map: &[ArgumentProvenanceEntry],
    argument_name: &str,
    prefix: &str,
) -> Option<String> {
    argument_resolved_value(argument_provenance_map, argument_name)?
        .strip_prefix(prefix)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
}

fn argument_resolved_value(
    argument_provenance_map: &[ArgumentProvenanceEntry],
    argument_name: &str,
) -> Option<String> {
    argument_provenance_map
        .iter()
        .find(|row| row.argument_name == argument_name)?
        .resolved_value_ref
        .as_deref()
        .map(str::to_string)
}

fn clone_operation_id_for(request: &CloneRequest, suffix: &str) -> String {
    format!(
        "clone-{:016x}-{suffix}",
        fnv1a_64(&format!(
            "{}\n{}",
            request.remote_url,
            request.destination_path.display()
        ))
    )
}

fn set_clone_sheet_status(
    overlay: &mut Option<ShellOverlayState>,
    status_line: String,
    running: bool,
) {
    if let Some(ShellOverlayState {
        kind: ShellOverlayKind::EntryFlowSheet(sheet),
        ..
    }) = overlay.as_mut()
    {
        if let Some(form) = sheet.clone_form.as_mut() {
            form.status_line = Some(status_line);
            form.running = running;
        }
    }
}

fn set_import_sheet_status(
    overlay: &mut Option<ShellOverlayState>,
    review: Option<ImportReviewRecord>,
    status_line: String,
    applied: bool,
) {
    if let Some(ShellOverlayState {
        kind: ShellOverlayKind::EntryFlowSheet(sheet),
        ..
    }) = overlay.as_mut()
    {
        if let Some(form) = sheet.import_form.as_mut() {
            if let Some(review) = review {
                form.diff_review_packet = Some(materialize_import_diff_review_packet(&review));
                form.review_record = Some(review);
            }
            form.status_line = Some(status_line);
            form.applied = applied;
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

fn invocation_and_result_import_profile_review_recorded(
    session: &CommandInvocationSession,
    review: &ImportReviewRecord,
    diff_review: &ImportDiffReviewPacket,
) -> RecordedCommandInvocation {
    let preview_ref = session
        .preview_posture
        .preview_record_ref
        .clone()
        .unwrap_or_else(|| review.import_review_id.clone());
    let import_review_ref = review.import_review_id.clone();
    let import_diff_preview_ref = diff_review.import_diff_preview_ref.clone();
    let checkpoint_ref = diff_review.rollback_checkpoint.checkpoint_ref.clone();
    let migration_report_ref = diff_review
        .retained_migration_report
        .migration_report_id
        .clone();
    let shortcut_delta_ref = diff_review
        .shortcut_delta_report
        .shortcut_delta_report_id
        .clone();
    let activity_ref = format!("ux:event:import-review:{import_review_ref}");
    let mut warning_codes = vec!["import_diff_review_packet_retained".to_string()];
    if review.decision_class == ImportReviewDecisionClass::Defer {
        warning_codes.push("import_deferred_source_unrecognized".to_string());
    }
    if diff_review.apply_gate_class == "requires_manual_review" {
        warning_codes.push("import_requires_manual_review_before_apply".to_string());
    }

    let outcome = InvocationOutcomeBlock {
        outcome_class: "succeeded_with_warnings".to_string(),
        disabled_reason_code: None,
        warnings_summary_refs: warning_codes.clone(),
        partially_applied_artifact_refs: Vec::new(),
        unapplied_artifact_refs: Vec::new(),
    };

    let session_packet = session.invocation_session_packet(
        outcome,
        vec![
            InvocationCreatedArtifactRefEntry {
                result_contract_class: "import_review_record_emitted_ref".to_string(),
                artifact_ref: import_review_ref.clone(),
            },
            InvocationCreatedArtifactRefEntry {
                result_contract_class: "first_run_import_diff_preview_ref".to_string(),
                artifact_ref: import_diff_preview_ref.clone(),
            },
            InvocationCreatedArtifactRefEntry {
                result_contract_class: "migration_report_retained_ref".to_string(),
                artifact_ref: migration_report_ref.clone(),
            },
            InvocationCreatedArtifactRefEntry {
                result_contract_class: "shortcut_delta_digest_ref".to_string(),
                artifact_ref: shortcut_delta_ref.clone(),
            },
        ],
        vec![
            EvidenceRefEntry {
                evidence_ref_class: "preview_record_ref".to_string(),
                evidence_id: preview_ref.clone(),
            },
            EvidenceRefEntry {
                evidence_ref_class: "import_review_record_ref".to_string(),
                evidence_id: import_review_ref.clone(),
            },
            EvidenceRefEntry {
                evidence_ref_class: "import_diff_preview_ref".to_string(),
                evidence_id: import_diff_preview_ref.clone(),
            },
            EvidenceRefEntry {
                evidence_ref_class: "checkpoint_ref".to_string(),
                evidence_id: checkpoint_ref.clone(),
            },
            EvidenceRefEntry {
                evidence_ref_class: "migration_report_ref".to_string(),
                evidence_id: migration_report_ref.clone(),
            },
            EvidenceRefEntry {
                evidence_ref_class: "shortcut_delta_digest_ref".to_string(),
                evidence_id: shortcut_delta_ref.clone(),
            },
        ],
    );

    let result = ResultBodyBlock {
        outcome_code: "succeeded_with_warnings".to_string(),
        warning_codes,
        error_codes: Vec::new(),
        created_artifact_refs: vec![
            ArtifactRefEntry {
                result_contract_class: "import_review_record_emitted_ref".to_string(),
                artifact_ref: import_review_ref.clone(),
                artifact_role: "import_review_record".to_string(),
            },
            ArtifactRefEntry {
                result_contract_class: "first_run_import_diff_preview_ref".to_string(),
                artifact_ref: import_diff_preview_ref.clone(),
                artifact_role: "import_diff_preview".to_string(),
            },
            ArtifactRefEntry {
                result_contract_class: "first_run_import_rollback_checkpoint_ref".to_string(),
                artifact_ref: checkpoint_ref.clone(),
                artifact_role: "rollback_checkpoint".to_string(),
            },
            ArtifactRefEntry {
                result_contract_class: "migration_report_retained_ref".to_string(),
                artifact_ref: migration_report_ref.clone(),
                artifact_role: "migration_report".to_string(),
            },
            ArtifactRefEntry {
                result_contract_class: "shortcut_delta_digest_ref".to_string(),
                artifact_ref: shortcut_delta_ref.clone(),
                artifact_role: "shortcut_delta_digest".to_string(),
            },
        ],
        notification_refs: vec![NotificationRefEntry {
            notification_ref: format!("ux:notif-env:import-review:{import_review_ref}"),
            delivery_posture: "routed_to_activity_center".to_string(),
        }],
        activity_refs: vec![ActivityRefEntry {
            activity_ref,
            activity_role: "durable_import_review_row".to_string(),
        }],
        rollback_handle_ref: RollbackHandleRefBlock {
            rollback_handle_posture: "handle_available_pre_apply_checkpoint".to_string(),
            rollback_handle_id: Some(format!(
                "rollback-handle:workspace.import_profile:{import_review_ref}"
            )),
        },
        checkpoint_refs: vec![aureline_commands::invocation::CheckpointRefEntry {
            checkpoint_class: "migration_import_checkpoint".to_string(),
            checkpoint_ref: Some(checkpoint_ref.clone()),
        }],
        evidence_refs: vec![
            EvidenceRefEntry {
                evidence_ref_class: "preview_record_ref".to_string(),
                evidence_id: preview_ref,
            },
            EvidenceRefEntry {
                evidence_ref_class: "import_review_record_ref".to_string(),
                evidence_id: import_review_ref,
            },
            EvidenceRefEntry {
                evidence_ref_class: "import_diff_preview_ref".to_string(),
                evidence_id: import_diff_preview_ref,
            },
            EvidenceRefEntry {
                evidence_ref_class: "checkpoint_ref".to_string(),
                evidence_id: checkpoint_ref,
            },
            EvidenceRefEntry {
                evidence_ref_class: "migration_report_ref".to_string(),
                evidence_id: migration_report_ref,
            },
            EvidenceRefEntry {
                evidence_ref_class: "shortcut_delta_digest_ref".to_string(),
                evidence_id: shortcut_delta_ref,
            },
        ],
        export_posture: ExportPostureBlock {
            export_posture_class: "exportable_with_redaction".to_string(),
            redaction_class: session.redaction_class.clone(),
            export_review_ref: Some(
                diff_review
                    .retained_migration_report
                    .migration_report_id
                    .clone(),
            ),
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
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    session_restore_store: &mut SessionRestoreStore,
    damage_geometry: &ShellDamageGeometryCache,
    palette: &mut CommandPaletteState,
    palette_focus_return: &mut FocusReturnStack<ShellFocusReturnTarget>,
    start_center: &mut StartCenterState,
    overlay: &mut Option<ShellOverlayState>,
    command_runtime: &mut CommandRuntimeState,
    clone_jobs: &mut CloneJobRuntimeState,
    keybinding_runtime: &mut KeybindingRuntimeState,
    enablement_runtime: &mut CommandEnablementRuntimeState,
    workspace_lifecycle: &mut WorkspaceLifecycleRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    clipboard: &mut ClipboardState,
    appearance: &mut AppearanceRuntimeState,
    hot_path_metrics: &mut HotPathMetrics,
    clock: &WallClock,
    modifiers: &HeldModifiers,
    event: &KeyEvent,
) -> ShellDamageHint {
    if event.state != ElementState::Pressed || event.repeat {
        return ShellDamageHint::None;
    }

    let PhysicalKey::Code(code) = event.physical_key else {
        return ShellDamageHint::None;
    };

    if palette.is_open() {
        let panel_hint = |class| {
            damage_geometry
                .command_palette_panel
                .map(|rect| ShellDamageHint::Rect {
                    layer: CompositionLayerId::FloatingSurface,
                    class,
                    rect,
                })
                .unwrap_or(ShellDamageHint::FullWindow)
        };

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
                    labs_enabled: enablement_runtime.labs_enabled,
                };
                let preview_arguments = palette
                    .selected_entry(registry)
                    .map(|entry| argument_provenance_map_for_shell(entry, frame, editor_runtime));
                let preview = materialize_palette_preview_record_with_arguments(
                    palette.selected_key(),
                    registry,
                    &keybinding_runtime.shortcuts_by_command_id,
                    runtime,
                    preview_arguments,
                );
                let PalettePreviewSelection::Command(command) = &preview.selection else {
                    command_runtime.note_non_command_action("copy: no command selected");
                    return panel_hint(DamageClassId::TextReflowLocal);
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
                    return panel_hint(DamageClassId::TextReflowLocal);
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
                    }
                    Err(err) => {
                        command_runtime.note_non_command_action(format!(
                            "copy failed — {} ({})",
                            command.command_id, err
                        ));
                    }
                }

                panel_hint(DamageClassId::TextReflowLocal)
            }
            KeyCode::KeyD if modifiers.ctrl_or_logo() => {
                let Some(entry) = palette.selected_entry(registry) else {
                    command_runtime.note_non_command_action("diagnostics: no command selected");
                    return panel_hint(DamageClassId::TextReflowLocal);
                };

                let runtime = CommandReviewRuntimeInputs {
                    client_scope: "desktop_product",
                    workspace_trust_state: enablement_runtime.workspace_trust_state.as_str(),
                    execution_context_available: enablement_runtime.execution_context_available,
                    provider_linked: enablement_runtime.provider_linked,
                    credential_available: enablement_runtime.credential_available,
                    policy_disabled: enablement_runtime.policy_disabled,
                    policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
                    labs_enabled: enablement_runtime.labs_enabled,
                };
                let record = materialize_command_diagnostics_sheet_record_with_arguments(
                    entry,
                    runtime,
                    argument_provenance_map_for_shell(entry, frame, editor_runtime),
                );
                write_diagnostics_sheet_log(&record);

                palette.write_snapshot_log(registry, &keybinding_runtime.shortcuts_by_command_id);
                palette.close();
                if let Some(target) = palette_focus_return.pop() {
                    target.apply(frame);
                }

                *overlay = Some(ShellOverlayState::command_diagnostics(
                    frame.focused_zone(),
                    frame.focused_editor_group(),
                    record,
                ));
                frame.focus_zone(ShellZoneId::TransientOverlay);
                command_runtime
                    .note_non_command_action(format!("diagnostics — {}", entry.command_id()));

                ShellDamageHint::FullWindow
            }
            KeyCode::Enter => {
                palette.write_snapshot_log(registry, &keybinding_runtime.shortcuts_by_command_id);
                let commit = palette.commit(registry);
                if let Some(target) = palette_focus_return.pop() {
                    target.apply(frame);
                }
                match commit {
                    Some(CommandPaletteCommit::CommandId(command_id)) => {
                        let mut io = CommandDispatchRuntimeIo {
                            window: Some(window),
                            damage_geometry: Some(damage_geometry),
                            clipboard: Some(clipboard),
                            appearance: Some(&mut *appearance),
                        };
                        let changed = dispatch_command_id_with_io(
                            command_runtime,
                            registry,
                            frame,
                            editor_runtime,
                            palette,
                            palette_focus_return,
                            overlay,
                            &command_id,
                            DispatchOrigin::CommandPalette,
                            enablement_runtime,
                            workspace_lifecycle,
                            recent_work,
                            activity_center,
                            &mut io,
                        );
                        if changed {
                            ShellDamageHint::FullWindow
                        } else {
                            ShellDamageHint::None
                        }
                    }
                    Some(CommandPaletteCommit::FilePath(relative_path)) => {
                        let focused_group = frame.focused_editor_group();
                        let tick = clock.now().0;
                        if frame.active_tab_id(focused_group).is_some() {
                            hot_path_metrics.note_file_switch_to_paint_requested(tick);
                        } else {
                            hot_path_metrics.note_file_open_to_paint_requested(tick);
                        }
                        let Some(tab) = frame.open_tab() else {
                            return ShellDamageHint::None;
                        };
                        let workspace_root = palette
                            .workspace_root()
                            .map(Path::to_path_buf)
                            .unwrap_or_else(|| {
                                std::env::current_dir()
                                    .unwrap_or_else(|_| std::path::PathBuf::from("."))
                            });
                        let path = workspace_root.join(&relative_path);
                        if let Err(err) = editor_runtime.open_file(focused_group, tab, &path) {
                            hot_path_metrics.close_latest_span_as_error(
                                clock.now().0,
                                format!("open file failed — {}", err),
                            );
                            command_runtime
                                .note_non_command_action(format!("open file failed — {}", err));
                            let _ = frame.close_active_tab(focused_group);
                            editor_runtime.close_tab(focused_group, tab);
                            return ShellDamageHint::FullWindow;
                        }
                        command_runtime
                            .note_non_command_action(format!("opened file — {}", relative_path));
                        ShellDamageHint::FullWindow
                    }
                    None => panel_hint(DamageClassId::TextReflowLocal),
                }
            }
            KeyCode::Escape => {
                palette.write_snapshot_log(registry, &keybinding_runtime.shortcuts_by_command_id);
                palette.close();
                if let Some(target) = palette_focus_return.pop() {
                    target.apply(frame);
                }
                ShellDamageHint::FullWindow
            }
            KeyCode::ArrowDown => {
                let handled = palette.handle_arrow_down();
                if handled {
                    panel_hint(DamageClassId::SelectionOverlayOnly)
                } else {
                    ShellDamageHint::None
                }
            }
            KeyCode::ArrowUp => {
                let handled = palette.handle_arrow_up();
                if handled {
                    panel_hint(DamageClassId::SelectionOverlayOnly)
                } else {
                    ShellDamageHint::None
                }
            }
            KeyCode::Backspace => {
                let handled =
                    palette.handle_backspace(registry, &keybinding_runtime.shortcuts_by_command_id);
                if handled {
                    panel_hint(DamageClassId::TextReflowLocal)
                } else {
                    ShellDamageHint::None
                }
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
                            return panel_hint(DamageClassId::TextReflowLocal);
                        }
                    }
                }
                ShellDamageHint::None
            }
        };
    }

    if overlay.is_some() {
        let outcome = {
            let state = overlay.as_mut().expect("overlay checked to be Some");
            state.handle_key(
                code,
                event.text.as_deref(),
                *modifiers,
                frame,
                editor_runtime,
                keybinding_runtime,
            )
        };

        let entry_flow_decision = outcome.entry_flow_decision.clone();
        let settings_decision = outcome.settings_decision.clone();

        if let Some(decision) = outcome.command_decision {
            finalize_command_overlay_decision(
                command_runtime,
                registry,
                frame,
                editor_runtime,
                session_restore_store,
                palette,
                palette_focus_return,
                overlay,
                enablement_runtime,
                workspace_lifecycle,
                recent_work,
                activity_center,
                decision,
            );
        }

        if let Some(decision) = outcome.workspace_switcher_decision {
            apply_workspace_switcher_decision(
                &window.inner_size().to_logical::<u32>(window.scale_factor()),
                &mut enablement_runtime.workspace_trust_state,
                recent_work,
                editor_runtime,
                frame,
                command_runtime,
                palette,
                workspace_lifecycle,
                activity_center,
                decision,
            );
        }

        if outcome.handled {
            if overlay.as_ref().is_some_and(|state| state.closed) {
                *overlay = None;
            }
            if let Some(decision) = entry_flow_decision {
                let changed = if decision.command_id == "cmd:workspace.clone_repository" {
                    submit_clone_from_entry_flow(
                        command_runtime,
                        registry,
                        frame,
                        overlay,
                        clone_jobs,
                        enablement_runtime,
                        activity_center,
                        decision,
                    )
                } else if decision.command_id == "cmd:workspace.import_profile" {
                    submit_import_from_entry_flow(
                        command_runtime,
                        registry,
                        frame,
                        overlay,
                        enablement_runtime,
                        activity_center,
                        decision,
                    )
                } else {
                    dispatch_command_id_with_arguments(
                        command_runtime,
                        registry,
                        frame,
                        editor_runtime,
                        palette,
                        palette_focus_return,
                        overlay,
                        &decision.command_id,
                        decision.origin,
                        enablement_runtime,
                        workspace_lifecycle,
                        recent_work,
                        activity_center,
                        Some(decision.argument_provenance_map),
                    )
                };
                return if changed {
                    ShellDamageHint::FullWindow
                } else {
                    ShellDamageHint::None
                };
            }
            if let Some(decision) = settings_decision {
                apply_settings_overlay_decision(
                    appearance,
                    activity_center,
                    command_runtime,
                    overlay,
                    decision,
                );
                return ShellDamageHint::FullWindow;
            }
            return ShellDamageHint::FullWindow;
        }
        return ShellDamageHint::None;
    }

    if frame.focused_zone() == ShellZoneId::BottomPanel {
        if is_terminal_toggle_chord(code, *modifiers, keybinding_runtime.platform_class) {
            let changed = dispatch_command_id(
                command_runtime,
                registry,
                frame,
                editor_runtime,
                palette,
                palette_focus_return,
                overlay,
                "cmd:terminal.toggle",
                DispatchOrigin::KeybindingChord,
                enablement_runtime,
                workspace_lifecycle,
                recent_work,
                activity_center,
            );
            return if changed {
                ShellDamageHint::FullWindow
            } else {
                ShellDamageHint::None
            };
        }

        if let Some(bytes) = terminal_input_bytes_for_key_event(
            code,
            event.text.as_deref(),
            *modifiers,
            keybinding_runtime.platform_class,
        ) {
            let trust_state =
                trust_state_for_recent_work(enablement_runtime.workspace_trust_state.as_str());
            match editor_runtime.terminal_pane.write_active_input(
                &bytes,
                trust_state,
                &mono_timestamp_now(),
            ) {
                Ok(()) => {}
                Err(err) => command_runtime.note_non_command_action(err),
            }
            return frame
                .layout()
                .bottom_panel
                .map(|rect| ShellDamageHint::Rect {
                    layer: CompositionLayerId::TextAndDecoration,
                    class: DamageClassId::TextReflowLocal,
                    rect: to_physical_rect(rect, window.scale_factor()),
                })
                .unwrap_or(ShellDamageHint::FullWindow);
        }
    }

    if frame.focused_zone() == ShellZoneId::LeftSidebar {
        match code {
            KeyCode::ArrowDown => {
                if editor_runtime.explorer.select_next() {
                    return left_sidebar_damage_hint(frame, window.scale_factor());
                }
                return ShellDamageHint::None;
            }
            KeyCode::ArrowUp => {
                if editor_runtime.explorer.select_prev() {
                    return left_sidebar_damage_hint(frame, window.scale_factor());
                }
                return ShellDamageHint::None;
            }
            KeyCode::ArrowRight => {
                if let Some(selected) = editor_runtime.explorer.tree.selected().cloned() {
                    if editor_runtime.explorer.expand_node(&selected) {
                        return left_sidebar_damage_hint(frame, window.scale_factor());
                    }
                }
                return ShellDamageHint::None;
            }
            KeyCode::ArrowLeft => {
                if editor_runtime.explorer.collapse_or_select_parent() {
                    return left_sidebar_damage_hint(frame, window.scale_factor());
                }
                return ShellDamageHint::None;
            }
            KeyCode::Enter => {
                if editor_runtime.explorer.selected_file_path().is_some() {
                    return open_selected_explorer_file(
                        editor_runtime,
                        frame,
                        command_runtime,
                        hot_path_metrics,
                        clock,
                    );
                }
                if editor_runtime.explorer.activate_selected_directory() {
                    return left_sidebar_damage_hint(frame, window.scale_factor());
                }
                return ShellDamageHint::None;
            }
            _ => {}
        }
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
                let mut io = CommandDispatchRuntimeIo {
                    window: Some(window),
                    damage_geometry: Some(damage_geometry),
                    clipboard: Some(clipboard),
                    appearance: Some(&mut *appearance),
                };
                let changed = dispatch_command_id_with_io(
                    command_runtime,
                    registry,
                    frame,
                    editor_runtime,
                    palette,
                    palette_focus_return,
                    overlay,
                    candidate.command.command_id.as_str(),
                    DispatchOrigin::KeybindingChord,
                    enablement_runtime,
                    workspace_lifecycle,
                    recent_work,
                    activity_center,
                    &mut io,
                );
                if !changed {
                    return ShellDamageHint::None;
                }
                if candidate.command.command_id == "cmd:workspace.open_folder" {
                    if let Some(rect) = damage_geometry.focused_editor_group {
                        return ShellDamageHint::Rect {
                            layer: CompositionLayerId::TextAndDecoration,
                            class: DamageClassId::TextReflowLocal,
                            rect,
                        };
                    }
                }
                return ShellDamageHint::FullWindow;
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
            labs_enabled: enablement_runtime.labs_enabled,
        };
        let rows = start_center_action_rows(registry, runtime);
        let row_count = rows.len();

        match code {
            KeyCode::ArrowDown => {
                start_center.select_next(row_count);
                return damage_geometry
                    .focused_editor_group
                    .map(|rect| ShellDamageHint::Rect {
                        layer: CompositionLayerId::TextAndDecoration,
                        class: DamageClassId::SelectionOverlayOnly,
                        rect,
                    })
                    .unwrap_or(ShellDamageHint::FullWindow);
            }
            KeyCode::ArrowUp => {
                start_center.select_prev(row_count);
                return damage_geometry
                    .focused_editor_group
                    .map(|rect| ShellDamageHint::Rect {
                        layer: CompositionLayerId::TextAndDecoration,
                        class: DamageClassId::SelectionOverlayOnly,
                        rect,
                    })
                    .unwrap_or(ShellDamageHint::FullWindow);
            }
            KeyCode::Enter => {
                let idx = start_center.selection().min(row_count.saturating_sub(1));
                let Some(row) = rows.get(idx) else {
                    return ShellDamageHint::None;
                };
                open_start_center_entry_flow_sheet(frame, overlay, row);
                return ShellDamageHint::FullWindow;
            }
            _ => {}
        }
    }

    match code {
        KeyCode::Enter => {
            if frame.focused_zone() == ShellZoneId::RightInspector {
                let changed = dispatch_command_id(
                    command_runtime,
                    registry,
                    frame,
                    editor_runtime,
                    palette,
                    palette_focus_return,
                    overlay,
                    "cmd:docs.open_in_browser",
                    DispatchOrigin::KeybindingChord,
                    enablement_runtime,
                    workspace_lifecycle,
                    recent_work,
                    activity_center,
                );
                if changed {
                    ShellDamageHint::FullWindow
                } else {
                    ShellDamageHint::None
                }
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::Tab => {
            if modifiers.ctrl_or_logo()
                && frame.focused_zone() == ShellZoneId::MainWorkspace
                && overlay.is_none()
            {
                let group = frame.focused_editor_group();
                let tabs = frame.tab_ids(group);
                let Some(active) = frame.active_tab_id(group) else {
                    return ShellDamageHint::None;
                };
                if tabs.len() <= 1 {
                    return ShellDamageHint::None;
                }
                let current_idx = tabs
                    .iter()
                    .position(|id| *id == active)
                    .unwrap_or(0)
                    .min(tabs.len().saturating_sub(1));
                let next_idx = if modifiers.shift {
                    if current_idx == 0 {
                        tabs.len().saturating_sub(1)
                    } else {
                        current_idx.saturating_sub(1)
                    }
                } else {
                    (current_idx + 1) % tabs.len()
                };
                let next_tab = tabs[next_idx];
                if frame.set_active_tab(group, next_tab) {
                    if let (Some(viewport), Some(session)) = (
                        damage_geometry.focused_editor_viewport,
                        editor_runtime.tab_session_mut(group, next_tab),
                    ) {
                        update_ime_cursor_area_for_viewport(
                            window,
                            &session.viewport,
                            PixelRect::new(viewport.x, viewport.y, viewport.width, viewport.height),
                        );
                    }
                    return damage_geometry
                        .focused_editor_group
                        .map(|rect| ShellDamageHint::Rect {
                            layer: CompositionLayerId::TextAndDecoration,
                            class: DamageClassId::TextReflowLocal,
                            rect,
                        })
                        .unwrap_or(ShellDamageHint::FullWindow);
                }
                return ShellDamageHint::None;
            }

            frame.focus_next();
            ShellDamageHint::FullWindow
        }
        KeyCode::KeyO => {
            if modifiers.ctrl_or_logo() {
                if modifiers.shift
                    && frame.focused_zone() == ShellZoneId::MainWorkspace
                    && overlay.is_none()
                {
                    let group = frame.focused_editor_group();
                    let Some(tab) = frame.active_tab_id(group) else {
                        return ShellDamageHint::None;
                    };
                    match editor_runtime.open_anyway(group, tab) {
                        Ok(()) => {
                            command_runtime
                                .note_non_command_action("opened anyway (large-file override)");
                        }
                        Err(err) => {
                            command_runtime
                                .note_non_command_action(format!("open anyway failed — {err}"));
                        }
                    }
                } else {
                    let group = frame.focused_editor_group();
                    if let Some(tab) = frame.open_tab() {
                        editor_runtime.open_placeholder(group, tab);
                    }
                }
                damage_geometry
                    .focused_editor_group
                    .map(|rect| ShellDamageHint::Rect {
                        layer: CompositionLayerId::TextAndDecoration,
                        class: DamageClassId::TextReflowLocal,
                        rect,
                    })
                    .unwrap_or(ShellDamageHint::FullWindow)
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::Backslash => {
            if modifiers.ctrl_or_logo() {
                let source_group = frame.focused_editor_group();
                let source_tab = frame.active_tab_id(source_group);
                let changed = match frame.request_split_focused_editor_group() {
                    NewEditorGroupOutcome::Created { new_group } => {
                        if let Some(source_tab) = source_tab {
                            if let Some(tab) = frame.open_tab_in_group(new_group) {
                                let _ = editor_runtime.clone_tab_view(
                                    source_group,
                                    source_tab,
                                    new_group,
                                    tab,
                                );
                            }
                        }
                        true
                    }
                    NewEditorGroupOutcome::WouldViolateMinimum(violation) => {
                        *overlay = Some(ShellOverlayState::split_choice(
                            frame.focused_zone(),
                            frame.focused_editor_group(),
                            violation,
                        ));
                        frame.focus_zone(ShellZoneId::TransientOverlay);
                        true
                    }
                };
                if changed {
                    ShellDamageHint::FullWindow
                } else {
                    ShellDamageHint::None
                }
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyG => {
            if modifiers.ctrl_or_logo() {
                frame.focus_next_editor_group();
                ShellDamageHint::FullWindow
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyS => {
            if modifiers.ctrl_or_logo() && frame.focused_zone() == ShellZoneId::MainWorkspace {
                let mut io = CommandDispatchRuntimeIo {
                    window: Some(window),
                    damage_geometry: Some(damage_geometry),
                    clipboard: Some(clipboard),
                    appearance: Some(&mut *appearance),
                };
                let changed = dispatch_command_id_with_io(
                    command_runtime,
                    registry,
                    frame,
                    editor_runtime,
                    palette,
                    palette_focus_return,
                    overlay,
                    "cmd:editor.save",
                    DispatchOrigin::KeybindingChord,
                    enablement_runtime,
                    workspace_lifecycle,
                    recent_work,
                    activity_center,
                    &mut io,
                );
                if changed {
                    ShellDamageHint::FullWindow
                } else {
                    ShellDamageHint::None
                }
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyZ => {
            if modifiers.ctrl_or_logo() && frame.focused_zone() == ShellZoneId::MainWorkspace {
                let command_id = if modifiers.shift {
                    "cmd:editor.redo"
                } else {
                    "cmd:editor.undo"
                };
                let mut io = CommandDispatchRuntimeIo {
                    window: Some(window),
                    damage_geometry: Some(damage_geometry),
                    clipboard: Some(clipboard),
                    appearance: Some(&mut *appearance),
                };
                let changed = dispatch_command_id_with_io(
                    command_runtime,
                    registry,
                    frame,
                    editor_runtime,
                    palette,
                    palette_focus_return,
                    overlay,
                    command_id,
                    DispatchOrigin::KeybindingChord,
                    enablement_runtime,
                    workspace_lifecycle,
                    recent_work,
                    activity_center,
                    &mut io,
                );
                if changed {
                    damage_geometry
                        .focused_editor_group
                        .map(|rect| ShellDamageHint::Rect {
                            layer: CompositionLayerId::TextAndDecoration,
                            class: DamageClassId::TextReflowLocal,
                            rect,
                        })
                        .unwrap_or(ShellDamageHint::FullWindow)
                } else {
                    ShellDamageHint::None
                }
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyY => {
            if modifiers.ctrl_or_logo() && frame.focused_zone() == ShellZoneId::MainWorkspace {
                let mut io = CommandDispatchRuntimeIo {
                    window: Some(window),
                    damage_geometry: Some(damage_geometry),
                    clipboard: Some(clipboard),
                    appearance: Some(&mut *appearance),
                };
                let changed = dispatch_command_id_with_io(
                    command_runtime,
                    registry,
                    frame,
                    editor_runtime,
                    palette,
                    palette_focus_return,
                    overlay,
                    "cmd:editor.redo",
                    DispatchOrigin::KeybindingChord,
                    enablement_runtime,
                    workspace_lifecycle,
                    recent_work,
                    activity_center,
                    &mut io,
                );
                if changed {
                    damage_geometry
                        .focused_editor_group
                        .map(|rect| ShellDamageHint::Rect {
                            layer: CompositionLayerId::TextAndDecoration,
                            class: DamageClassId::TextReflowLocal,
                            rect,
                        })
                        .unwrap_or(ShellDamageHint::FullWindow)
                } else {
                    ShellDamageHint::None
                }
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyW => {
            if modifiers.ctrl_or_logo() {
                if modifiers.shift {
                    let closing = frame.focused_editor_group();
                    let changed = frame.close_focused_editor_group();
                    if changed {
                        editor_runtime.close_group(closing);
                        ShellDamageHint::FullWindow
                    } else {
                        ShellDamageHint::None
                    }
                } else if frame.focused_zone() == ShellZoneId::MainWorkspace {
                    let group = frame.focused_editor_group();
                    let Some(closed) = frame.close_active_tab(group) else {
                        return ShellDamageHint::None;
                    };
                    editor_runtime.close_tab(group, closed);
                    damage_geometry
                        .focused_editor_group
                        .map(|rect| ShellDamageHint::Rect {
                            layer: CompositionLayerId::TextAndDecoration,
                            class: DamageClassId::TextReflowLocal,
                            rect,
                        })
                        .unwrap_or(ShellDamageHint::FullWindow)
                } else {
                    ShellDamageHint::None
                }
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyI => {
            if modifiers.ctrl_or_logo() && frame.layout().right_inspector.is_none() {
                *overlay = Some(ShellOverlayState::inspector_sheet(
                    frame.focused_zone(),
                    frame.focused_editor_group(),
                ));
                frame.focus_zone(ShellZoneId::TransientOverlay);
                ShellDamageHint::FullWindow
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyR => {
            if modifiers.ctrl_or_logo()
                && !modifiers.shift
                && frame.focused_zone() == ShellZoneId::MainWorkspace
            {
                let group = frame.focused_editor_group();
                let Some(tab) = frame.active_tab_id(group) else {
                    return ShellDamageHint::None;
                };
                if !editor_runtime.has_tab_session(group, tab) {
                    editor_runtime.open_placeholder(group, tab);
                }
                let Some(session) = editor_runtime.tab_session_mut(group, tab) else {
                    return ShellDamageHint::None;
                };
                session.ensure_fresh_snapshot();

                let (changed, note) = {
                    let mut authority = session.authority.borrow_mut();
                    if authority.is_dirty() {
                        (
                            false,
                            Some("reload blocked — buffer has unsaved edits".to_string()),
                        )
                    } else if authority.file_path.is_none() {
                        (
                            false,
                            Some("reload blocked — buffer has no file backing".to_string()),
                        )
                    } else {
                        let path = authority
                            .file_path
                            .clone()
                            .expect("path already checked above");

                        match std::fs::read(&path) {
                            Ok(bytes) => {
                                if bytes == authority.buffer.contents() {
                                    (false, Some("reload: no on-disk changes".to_string()))
                                } else {
                                    match std::str::from_utf8(&bytes) {
                                        Ok(text) => {
                                            let len = authority.buffer.len();
                                            let spec = TransactionSpec::new(
                                                UndoClass::ExternalReload,
                                                aureline_editor::undo::originator::EXTERNAL_CHANGE_RELOAD,
                                            )
                                            .with_label("Reloaded from disk");

                                            let (committed, note) = match authority
                                                .buffer
                                                .begin(spec)
                                            {
                                                Ok(mut tx) => {
                                                    if let Err(err) = tx.replace(0..len, text) {
                                                        (false, format!("reload failed — {err}"))
                                                    } else if let Err(err) = tx.commit() {
                                                        (false, format!("reload failed — {err}"))
                                                    } else {
                                                        (true, "reloaded from disk".to_string())
                                                    }
                                                }
                                                Err(err) => {
                                                    (false, format!("reload failed — {err}"))
                                                }
                                            };

                                            if committed {
                                                authority.mark_saved();
                                            }

                                            (committed, Some(note))
                                        }
                                        Err(_) => (
                                            false,
                                            Some(
                                                "reload blocked — on-disk bytes are not UTF-8"
                                                    .to_string(),
                                            ),
                                        ),
                                    }
                                }
                            }
                            Err(err) => (false, Some(format!("reload failed — read error: {err}"))),
                        }
                    }
                };

                if let Some(note) = note {
                    command_runtime.note_non_command_action(note);
                }
                if !changed {
                    return ShellDamageHint::None;
                }

                session.refresh_snapshot_and_cache();
                session.text_input.force_clear_composition();
                session.viewport.set_ime_composition(None);
                session.needs_text_repaint = true;

                damage_geometry
                    .focused_editor_group
                    .map(|rect| ShellDamageHint::Rect {
                        layer: CompositionLayerId::TextAndDecoration,
                        class: DamageClassId::TextReflowLocal,
                        rect,
                    })
                    .unwrap_or(ShellDamageHint::FullWindow)
            } else if modifiers.ctrl_or_logo() && modifiers.shift {
                *overlay = Some(ShellOverlayState::workspace_switcher(
                    frame.focused_zone(),
                    frame.focused_editor_group(),
                    &recent_work.registry,
                    enablement_runtime.workspace_trust_state.as_str(),
                ));
                frame.focus_zone(ShellZoneId::TransientOverlay);
                ShellDamageHint::FullWindow
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyT => {
            if modifiers.ctrl_or_logo() && modifiers.shift {
                let previous_trust_state = enablement_runtime.workspace_trust_state.clone();
                enablement_runtime.toggle_trust_state();
                workspace_lifecycle.resolve_trust(trust_state_for_recent_work(
                    enablement_runtime.workspace_trust_state.as_str(),
                ));
                activity_center.note_trust_state_changed(
                    &previous_trust_state,
                    enablement_runtime.workspace_trust_state.as_str(),
                );
                ShellDamageHint::FullWindow
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyE => {
            if modifiers.ctrl_or_logo() && modifiers.shift {
                let changed = dispatch_command_id(
                    command_runtime,
                    registry,
                    frame,
                    editor_runtime,
                    palette,
                    palette_focus_return,
                    overlay,
                    "cmd:explorer.toggle",
                    DispatchOrigin::KeybindingChord,
                    enablement_runtime,
                    workspace_lifecycle,
                    recent_work,
                    activity_center,
                );
                if changed {
                    ShellDamageHint::FullWindow
                } else {
                    ShellDamageHint::None
                }
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyB => {
            if modifiers.ctrl_or_logo() && modifiers.shift {
                enablement_runtime.toggle_policy_blocked();
                ShellDamageHint::FullWindow
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyL => {
            if modifiers.ctrl_or_logo() && modifiers.shift {
                appearance.toggle_light_dark();
                ShellDamageHint::FullWindow
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyH => {
            if modifiers.ctrl_or_logo() && modifiers.shift && modifiers.alt {
                appearance.toggle_high_contrast();
                ShellDamageHint::FullWindow
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyM => {
            if modifiers.ctrl_or_logo() && modifiers.shift && modifiers.alt {
                appearance.cycle_reduced_motion_posture();
                ShellDamageHint::FullWindow
            } else if modifiers.ctrl_or_logo() && modifiers.shift {
                appearance.cycle_density_class();
                ShellDamageHint::FullWindow
            } else {
                ShellDamageHint::None
            }
        }
        _ => ShellDamageHint::None,
    }
}

fn apply_settings_overlay_decision(
    appearance: &mut AppearanceRuntimeState,
    activity_center: &mut ActivityCenterRuntimeState,
    command_runtime: &mut CommandRuntimeState,
    overlay: &mut Option<ShellOverlayState>,
    decision: SettingsOverlayDecision,
) {
    let outcome = appearance.write_ui_setting(
        decision.setting_id.as_str(),
        decision.value.as_str(),
        SettingScope::UserGlobal,
    );
    let status_line = match outcome.verdict {
        WriteIntent::Allowed => {
            let line = format!(
                "applied {}={} at {}",
                outcome.setting_id,
                outcome.proposed_value.preview(),
                outcome.target_scope.as_str()
            );
            command_runtime.note_non_command_action(format!("settings updated - {}", line));
            line
        }
        WriteIntent::Denied => {
            let reason = outcome
                .denial_reason
                .as_ref()
                .map(settings_denial_reason_label)
                .unwrap_or_else(|| "unknown".to_string());
            let line = format!(
                "denied {}={} - {}",
                outcome.setting_id,
                outcome.proposed_value.preview(),
                reason
            );
            command_runtime.note_non_command_action(format!("settings write denied - {}", line));
            activity_center.note_settings_write_denied(&outcome);
            line
        }
    };

    if let Some(ShellOverlayState {
        kind: ShellOverlayKind::Settings(settings),
        ..
    }) = overlay.as_mut()
    {
        settings.refresh(appearance);
        settings.status_line = Some(status_line);
    }
}

fn settings_denial_reason_label(reason: &WriteDenialReason) -> String {
    match reason {
        WriteDenialReason::UnknownSetting { setting_id } => {
            format!("unknown_setting:{setting_id}")
        }
        WriteDenialReason::ScopeNotAllowed => "scope_not_allowed".to_string(),
        WriteDenialReason::PolicyLocked => "policy_locked".to_string(),
        WriteDenialReason::PolicyConstrainedValue => "policy_constrained_value".to_string(),
        WriteDenialReason::ValidationFailed { detail } => {
            format!("validation_failed:{detail}")
        }
        WriteDenialReason::RetiredSetting => "retired_setting".to_string(),
    }
}

fn text_input_key_code(code: KeyCode) -> aureline_input::text_input::TextInputKeyCode {
    use aureline_input::text_input::TextInputKeyCode as Out;

    match code {
        KeyCode::ArrowLeft => Out::ArrowLeft,
        KeyCode::ArrowRight => Out::ArrowRight,
        KeyCode::ArrowUp => Out::ArrowUp,
        KeyCode::ArrowDown => Out::ArrowDown,
        KeyCode::Home => Out::Home,
        KeyCode::End => Out::End,
        KeyCode::PageUp => Out::PageUp,
        KeyCode::PageDown => Out::PageDown,
        KeyCode::Backspace => Out::Backspace,
        KeyCode::Delete => Out::Delete,
        KeyCode::Enter => Out::Enter,
        _ => Out::Other,
    }
}

fn editor_action_from_text_input(
    action: aureline_input::text_input::TextInputAction,
) -> EditorAction {
    use aureline_input::text_input::{CaretMove as InputMove, TextInputAction as InputAction};

    match action {
        InputAction::InsertText { text } => EditorAction::InsertText { text },
        InputAction::DeleteBackward => EditorAction::DeleteBackward,
        InputAction::DeleteForward => EditorAction::DeleteForward,
        InputAction::MoveCaret {
            movement,
            extend_selection,
        } => EditorAction::MoveCaret {
            movement: match movement {
                InputMove::Left => CaretMove::Left,
                InputMove::Right => CaretMove::Right,
                InputMove::Up => CaretMove::Up,
                InputMove::Down => CaretMove::Down,
                InputMove::WordLeft => CaretMove::WordLeft,
                InputMove::WordRight => CaretMove::WordRight,
                InputMove::LineStart => CaretMove::LineStart,
                InputMove::LineEnd => CaretMove::LineEnd,
                InputMove::PageUp => CaretMove::PageUp,
                InputMove::PageDown => CaretMove::PageDown,
            },
            extend_selection,
        },
        InputAction::UpdateComposition { composition } => EditorAction::UpdateComposition {
            composition: aureline_editor::ImeComposition {
                text: composition.text,
                caret_byte_offset: composition.caret_byte_offset,
            },
        },
        InputAction::ClearComposition => EditorAction::ClearComposition,
    }
}

fn update_ime_cursor_area_for_viewport(
    window: &winit::window::Window,
    viewport: &EditorViewport,
    viewport_rect: PixelRect,
) {
    if viewport_rect.is_empty() {
        return;
    }

    let caret = viewport.caret();
    let layout = viewport.layout();
    let Some(line) = layout.line(caret.line) else {
        return;
    };

    let x_rel = line.grapheme_x_px.get(caret.grapheme).copied().unwrap_or(0);
    let y_rel = line.y_top_px.max(0) as u32;

    let x = viewport_rect.x.saturating_add(x_rel);
    let y = viewport_rect.y.saturating_add(y_rel);
    let height = layout.line_height_px.max(1);

    window.set_ime_cursor_area(
        winit::dpi::PhysicalPosition::new(x, y),
        winit::dpi::PhysicalSize::new(1u32, height),
    );
}

fn update_ime_cursor_area_for_rect(window: &winit::window::Window, rect: Rect) {
    if rect.is_empty() {
        return;
    }

    window.set_ime_cursor_area(
        winit::dpi::PhysicalPosition::new(rect.x, rect.y),
        winit::dpi::PhysicalSize::new(rect.width.max(1), rect.height.max(1)),
    );
}

fn hit_test_viewport_point(
    viewport: &EditorViewport,
    viewport_rect: PixelRect,
    x: u32,
    y: u32,
) -> Option<aureline_editor::TextPoint> {
    if viewport_rect.is_empty() {
        return None;
    }
    if x < viewport_rect.x
        || x >= viewport_rect.right()
        || y < viewport_rect.y
        || y >= viewport_rect.bottom()
    {
        return None;
    }

    let rel_x = x.saturating_sub(viewport_rect.x);
    let rel_y = y.saturating_sub(viewport_rect.y) as i32;
    let layout = viewport.layout();
    if layout.line_height_px == 0 {
        return None;
    }

    for line in &layout.lines {
        let y0 = line.y_top_px;
        let y1 = y0.saturating_add(layout.line_height_px as i32);
        if rel_y < y0 || rel_y >= y1 {
            continue;
        }

        let mut col = 0usize;
        for (idx, x_pos) in line.grapheme_x_px.iter().enumerate() {
            if *x_pos <= rel_x {
                col = idx;
            } else {
                break;
            }
        }
        return Some(aureline_editor::TextPoint {
            line: line.line_index,
            grapheme: col,
        });
    }

    None
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
    backend: &mut ShellRenderBackend,
    frame: &mut DesktopFrame,
    scheduler: &mut FrameScheduler,
) {
    let physical = window.inner_size();
    if physical.width == 0 || physical.height == 0 {
        return;
    }
    let logical = physical.to_logical::<u32>(window.scale_factor());
    frame.relayout(logical.width, logical.height);

    match backend {
        ShellRenderBackend::Gpu {
            renderer,
            last_size,
            ..
        } => {
            *last_size = (physical.width, physical.height);
            let _ = renderer.resize();
        }
        ShellRenderBackend::Software { surface } => {
            if let (Some(w), Some(h)) = (
                NonZeroU32::new(physical.width),
                NonZeroU32::new(physical.height),
            ) {
                let _ = surface.resize(w, h);
            }
        }
    }
    scheduler.invalidate(DamageEvent::new(
        CompositionLayerId::WindowChromeBase,
        DamageClassId::ViewportResizeOrScaleChange,
    ));
    scheduler.invalidate(DamageEvent::new(
        CompositionLayerId::TextAndDecoration,
        DamageClassId::ViewportResizeOrScaleChange,
    ));
}

fn focused_editor_group_physical_rect(frame: &DesktopFrame, scale_factor: f64) -> Option<Rect> {
    let focused = frame.focused_editor_group();
    frame
        .editor_group_layouts()
        .into_iter()
        .find(|group| group.group_id == focused)
        .map(|group| to_physical_rect(group.rect, scale_factor))
}

fn editor_viewport_rect_for_group(group_rect: Rect, style: &ShellRenderStyle) -> Rect {
    let inset = style.stroke_default.max(1);
    let inner = Rect::new(
        group_rect.x.saturating_add(inset),
        group_rect.y.saturating_add(inset),
        group_rect.width.saturating_sub(inset.saturating_mul(2)),
        group_rect.height.saturating_sub(inset.saturating_mul(2)),
    );
    if inner.is_empty() {
        return Rect::new(inner.x, inner.y, 0, 0);
    }
    let tab_h = style.density_tab_height.min(inner.height);
    Rect::new(
        inner.x,
        inner.y.saturating_add(tab_h),
        inner.width,
        inner.height.saturating_sub(tab_h),
    )
}

fn command_palette_panel_rect(
    frame: &DesktopFrame,
    scale_factor: f64,
    style: &ShellRenderStyle,
) -> Option<Rect> {
    let overlay_logical = frame.layout().zone(ShellZoneId::TransientOverlay)?;
    let zone_inset_logical = to_logical_px(style.density_zone_inset, scale_factor);
    let slots = frame.slot_rects_within_zone(
        ShellZoneId::TransientOverlay,
        overlay_logical,
        zone_inset_logical,
    );
    let slot = slots
        .iter()
        .find(|(id, _)| *id == "slot.overlay.command_palette")
        .map(|(_, rect)| *rect)
        .unwrap_or(overlay_logical);
    let slot_physical = to_physical_rect(slot, scale_factor);

    let panel_padding = style.density_zone_inset;
    let panel = Rect::new(
        slot_physical.x.saturating_add(panel_padding),
        slot_physical.y.saturating_add(panel_padding),
        slot_physical.width.saturating_sub(panel_padding * 2),
        slot_physical.height.saturating_sub(panel_padding * 2),
    );

    (!panel.is_empty()).then_some(panel)
}

fn command_palette_query_rect(
    frame: &DesktopFrame,
    scale_factor: f64,
    style: &ShellRenderStyle,
) -> Option<Rect> {
    let panel = command_palette_panel_rect(frame, scale_factor, style)?;
    if panel.is_empty() {
        return None;
    }

    let text_scale = 2u32;
    let line_h = (8u32.saturating_mul(text_scale)).saturating_add(style.space_2);
    let inner_padding = style.density_panel_padding;

    let mut cursor_y = panel.y.saturating_add(inner_padding);
    cursor_y = cursor_y.saturating_add(line_h);
    cursor_y = cursor_y
        .saturating_add(line_h)
        .saturating_add(style.space_2 / 2);

    let query_rect = Rect::new(
        panel.x.saturating_add(inner_padding),
        cursor_y,
        panel.width.saturating_sub(inner_padding.saturating_mul(2)),
        style.density_control_height,
    );

    (!query_rect.is_empty()).then_some(query_rect)
}

fn draw(
    window: &winit::window::Window,
    backend: &mut ShellRenderBackend,
    text_runtime: &mut ShellTextRuntime,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    registry: &CommandRegistry,
    events: &[DamageEvent],
    frame: &DesktopFrame,
    palette: &CommandPaletteState,
    start_center: &StartCenterState,
    docs_help_boundary_card: &EmbeddedBoundaryCardRecord,
    overlay: Option<&ShellOverlayState>,
    command_runtime: &CommandRuntimeState,
    keybinding_runtime: &KeybindingRuntimeState,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &WorkspaceLifecycleRuntimeState,
    recent_work: &RecentWorkRuntimeState,
    activity_center: &ActivityCenterRuntimeState,
    title_context_bar: &TitleContextBarStateRecord,
    appearance: &AppearanceRuntimeState,
    held_modifiers: &HeldModifiers,
    damage_geometry: &mut ShellDamageGeometryCache,
    screenshot_path: Option<&Path>,
) -> Result<(), Box<dyn std::error::Error>> {
    let physical = window.inner_size();
    if physical.width == 0 || physical.height == 0 {
        return Ok(());
    }
    let width = physical.width;
    let height = physical.height;

    let window_bounds = PixelRect::new(0, 0, width, height);
    let plan = DirtyRegionEngine::plan(window_bounds, events);

    let token_registry = seeded_token_registry(appearance.theme_class())?;
    let style = ShellRenderStyle::load(
        token_registry,
        appearance.density_class(),
        window.scale_factor(),
    )?;
    let now = Instant::now();

    damage_geometry.focused_editor_group =
        focused_editor_group_physical_rect(frame, window.scale_factor());
    damage_geometry.focused_editor_viewport = damage_geometry
        .focused_editor_group
        .map(|rect| editor_viewport_rect_for_group(rect, &style));
    damage_geometry.command_palette_panel = if palette.is_open() {
        command_palette_panel_rect(frame, window.scale_factor(), &style)
    } else {
        None
    };
    damage_geometry.command_palette_query = if palette.is_open() {
        command_palette_query_rect(frame, window.scale_factor(), &style)
    } else {
        None
    };

    match backend {
        ShellRenderBackend::Software { surface } => {
            surface.resize(
                NonZeroU32::new(width).ok_or("window width is zero")?,
                NonZeroU32::new(height).ok_or("window height is zero")?,
            )?;
            let mut buffer = surface.buffer_mut()?;
            if buffer.len() != (width as usize).saturating_mul(height as usize) {
                return Ok(());
            }
            rasterize_shell(
                window,
                &mut buffer,
                width,
                height,
                now,
                token_registry,
                text_runtime,
                editor_runtime,
                registry,
                frame,
                palette,
                start_center,
                docs_help_boundary_card,
                overlay,
                command_runtime,
                keybinding_runtime,
                enablement_runtime,
                workspace_lifecycle,
                recent_work,
                activity_center,
                title_context_bar,
                appearance,
                &style,
                held_modifiers,
            );
            if let Some(path) = screenshot_path {
                write_png_0rgb(path, width, height, &buffer)?;
            }
            buffer.present()?;
            Ok(())
        }
        ShellRenderBackend::Gpu {
            renderer,
            retained_frame,
            last_size,
        } => {
            if *last_size != (width, height) {
                *last_size = (width, height);
                let _ = renderer.resize();
            }
            let required = (width as usize).saturating_mul(height as usize);
            let mut force_full_redraw = false;
            if screenshot_path.is_some() {
                force_full_redraw = true;
            }
            if retained_frame.len() != required {
                retained_frame.resize(required, 0);
                force_full_redraw = true;
            }

            let (clip_rect, upload_rect, use_dirty_upload) =
                if force_full_redraw || plan.is_full_window() {
                    (None, window_bounds, false)
                } else {
                    let mut union: Option<PixelRect> = None;
                    for rect in plan.rects() {
                        union = Some(match union {
                            None => rect,
                            Some(prev) => prev.union(rect),
                        });
                    }
                    if let Some(rect) = union {
                        (
                            Some(Rect::new(rect.x, rect.y, rect.width, rect.height)),
                            rect,
                            true,
                        )
                    } else {
                        (None, window_bounds, false)
                    }
                };

            with_raster_clip(clip_rect, || {
                rasterize_shell(
                    window,
                    retained_frame,
                    width,
                    height,
                    now,
                    token_registry,
                    text_runtime,
                    editor_runtime,
                    registry,
                    frame,
                    palette,
                    start_center,
                    docs_help_boundary_card,
                    overlay,
                    command_runtime,
                    keybinding_runtime,
                    enablement_runtime,
                    workspace_lifecycle,
                    recent_work,
                    activity_center,
                    title_context_bar,
                    appearance,
                    &style,
                    held_modifiers,
                );
            });

            if let Some(path) = screenshot_path {
                write_png_0rgb(path, width, height, retained_frame)?;
            }
            if use_dirty_upload {
                renderer.render_0rgb_dirty(retained_frame, &[upload_rect])?;
            } else {
                renderer.render_0rgb(retained_frame)?;
            }
            Ok(())
        }
    }
}

fn rasterize_shell(
    window: &winit::window::Window,
    buffer: &mut [u32],
    width: u32,
    height: u32,
    now: Instant,
    token_registry: &TokenRegistry,
    text_runtime: &mut ShellTextRuntime,
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    registry: &CommandRegistry,
    frame: &DesktopFrame,
    palette: &CommandPaletteState,
    start_center: &StartCenterState,
    docs_help_boundary_card: &EmbeddedBoundaryCardRecord,
    overlay: Option<&ShellOverlayState>,
    command_runtime: &CommandRuntimeState,
    keybinding_runtime: &KeybindingRuntimeState,
    enablement_runtime: &CommandEnablementRuntimeState,
    workspace_lifecycle: &WorkspaceLifecycleRuntimeState,
    recent_work: &RecentWorkRuntimeState,
    activity_center: &ActivityCenterRuntimeState,
    title_context_bar: &TitleContextBarStateRecord,
    appearance: &AppearanceRuntimeState,
    style: &ShellRenderStyle,
    held_modifiers: &HeldModifiers,
) {
    let clip = raster_clip();
    // Background.
    if let Some(clip_rect) = clip {
        fill_rect(buffer, width, height, clip_rect, style.tokens.bg_canvas);
    } else {
        fill(buffer, style.tokens.bg_canvas);
    }
    let focus_ring = style.component_states.focus_ring_style();

    let scale = window.scale_factor();
    let scale_bucket = scale_bucket_for_scale_factor(scale);
    let clip_px = clip.map(|rect| PixelRect::new(rect.x, rect.y, rect.width, rect.height));
    let editor_paint_style = ViewportPaintStyle {
        background: style.tokens.bg_surface,
        text: style.tokens.text_primary,
        selection_fill: ColorRgba {
            a: 96,
            ..style.tokens.accent_interactive
        },
        match_fill: ColorRgba {
            a: 72,
            ..style.status_warning_fill
        },
        match_active_fill: ColorRgba {
            a: 156,
            ..style.status_warning_fill
        },
        caret: style.tokens.text_primary,
        font_size_px: 14.0,
        padding_x_px: style.space_2,
        padding_y_px: style.space_2,
    };
    for zone in ShellZoneId::ALL {
        let zone = *zone;
        if zone == ShellZoneId::TransientOverlay {
            continue;
        }
        let Some(logical_rect) = frame.layout().zone(zone) else {
            continue;
        };
        let rect = to_physical_rect(logical_rect, scale);
        if clip.is_some_and(|clip_rect| !rect_intersects(rect, clip_rect)) {
            continue;
        }
        let color = style.tokens.zone_background(zone);
        fill_rect(buffer, width, height, rect, color);

        match zone {
            ShellZoneId::MainWorkspace => {
                for group in frame.editor_group_layouts() {
                    let group_rect = to_physical_rect(group.rect, scale);
                    if clip.is_some_and(|clip_rect| !rect_intersects(group_rect, clip_rect)) {
                        continue;
                    }
                    fill_rect(buffer, width, height, group_rect, style.tokens.bg_surface);
                    stroke_rect(
                        buffer,
                        width,
                        height,
                        group_rect,
                        style.stroke_default,
                        style.tokens.border_default,
                    );
                    if group.group_id == frame.focused_editor_group()
                        && frame.focused_zone() == ShellZoneId::MainWorkspace
                    {
                        stroke_rect(
                            buffer,
                            width,
                            height,
                            group_rect,
                            focus_ring.stroke_px,
                            focus_ring.color,
                        );
                    }

                    if group.tab_count == 0 {
                        let focused = group.group_id == frame.focused_editor_group()
                            && frame.focused_zone() == ShellZoneId::MainWorkspace;
                        draw_start_center_surface(
                            buffer,
                            width,
                            height,
                            text_runtime,
                            scale_bucket,
                            registry,
                            start_center,
                            enablement_runtime,
                            group_rect,
                            &style,
                            focused,
                        );
                    } else {
                        let inset = style.stroke_default.max(1);
                        let inner = Rect::new(
                            group_rect.x.saturating_add(inset),
                            group_rect.y.saturating_add(inset),
                            group_rect.width.saturating_sub(inset.saturating_mul(2)),
                            group_rect.height.saturating_sub(inset.saturating_mul(2)),
                        );
                        if !inner.is_empty() {
                            let tab_h = style.density_tab_height.min(inner.height);
                            let tab_strip = Rect::new(inner.x, inner.y, inner.width, tab_h);
                            let viewport = Rect::new(
                                inner.x,
                                inner.y.saturating_add(tab_h),
                                inner.width,
                                inner.height.saturating_sub(tab_h),
                            );

                            if !tab_strip.is_empty() {
                                fill_rect(buffer, width, height, tab_strip, style.tokens.bg_raised);
                                stroke_rect(
                                    buffer,
                                    width,
                                    height,
                                    tab_strip,
                                    style.stroke_default,
                                    style.tokens.border_default,
                                );

                                let tabs = frame.tab_ids(group.group_id);
                                let active = frame.active_tab_id(group.group_id);
                                let mut parts = Vec::new();
                                for (idx, tab_id) in tabs.iter().copied().enumerate() {
                                    let info =
                                        editor_runtime.tab_render_info(group.group_id, tab_id);
                                    let label = info
                                        .as_ref()
                                        .map(|row| row.label.as_str())
                                        .unwrap_or("Untitled");
                                    let mut segment = if Some(tab_id) == active {
                                        format!("[{}] {}", idx + 1, label)
                                    } else {
                                        format!("{} {}", idx + 1, label)
                                    };
                                    if let Some(info) = info {
                                        if info.dirty {
                                            segment.push_str(" (Modified)");
                                        }
                                        if let Some(state) = info.large_file_state {
                                            segment.push_str(" (");
                                            segment.push_str(state.token());
                                            segment.push(')');
                                        }
                                        if let Some(token) = info.read_only.token() {
                                            segment.push_str(" (");
                                            segment.push_str(token);
                                            segment.push(')');
                                        }
                                        if let Some(token) = info.generated.token() {
                                            segment.push_str(" (");
                                            segment.push_str(token);
                                            segment.push(')');
                                        }
                                        if let Some(token) = info.managed.token() {
                                            segment.push_str(" (");
                                            segment.push_str(token);
                                            segment.push(')');
                                        }
                                        if let Some(token) = info.projection.token() {
                                            segment.push_str(" (");
                                            segment.push_str(token);
                                            segment.push(')');
                                        }
                                        if let Some(token) = info.snapshot_facing.token() {
                                            segment.push_str(" (");
                                            segment.push_str(token);
                                            segment.push(')');
                                        }
                                    }
                                    parts.push(segment);
                                }
                                let label = parts.join("  ");
                                draw_text(
                                    buffer,
                                    width,
                                    height,
                                    tab_strip.x.saturating_add(style.space_2),
                                    tab_strip.y.saturating_add(style.space_2 / 2),
                                    1,
                                    &label,
                                    style.tokens.text_muted,
                                );
                            }

                            if !viewport.is_empty() {
                                if let Some(active_tab) = frame.active_tab_id(group.group_id) {
                                    if !editor_runtime.has_tab_session(group.group_id, active_tab) {
                                        editor_runtime.open_placeholder(group.group_id, active_tab);
                                    }

                                    editor_runtime.compose_group(
                                        group.group_id,
                                        active_tab,
                                        buffer,
                                        width,
                                        height,
                                        PixelRect::new(
                                            viewport.x,
                                            viewport.y,
                                            viewport.width,
                                            viewport.height,
                                        ),
                                        clip_px,
                                        &editor_paint_style,
                                    );
                                }
                            }
                        }
                    }
                }
            }
            ShellZoneId::BottomPanel => {
                let zone_inset_logical = to_logical_px(style.density_zone_inset, scale);
                for (_slot_id, slot_rect) in
                    frame.slot_rects_within_zone(zone, logical_rect, zone_inset_logical)
                {
                    let slot_rect = to_physical_rect(slot_rect, scale);
                    if clip.is_some_and(|clip_rect| !rect_intersects(slot_rect, clip_rect)) {
                        continue;
                    }
                    draw_terminal_panel(
                        buffer,
                        width,
                        height,
                        slot_rect,
                        &mut editor_runtime.terminal_pane,
                        keybinding_runtime,
                        &style,
                        zone == frame.focused_zone(),
                    );
                }
            }
            ShellZoneId::TitleContextBar => {
                let zone_inset_logical = to_logical_px(style.density_zone_inset, scale);
                for (_slot_id, slot_rect) in
                    frame.slot_rects_within_zone(zone, logical_rect, zone_inset_logical)
                {
                    let slot_rect = to_physical_rect(slot_rect, scale);
                    if clip.is_some_and(|clip_rect| !rect_intersects(slot_rect, clip_rect)) {
                        continue;
                    }
                    fill_rect(buffer, width, height, slot_rect, style.tokens.bg_surface);
                    stroke_rect(
                        buffer,
                        width,
                        height,
                        slot_rect,
                        style.stroke_default,
                        style.tokens.border_default,
                    );
                    let label = title_context_bar
                        .projection_label(SurfaceKind::TitleContextBar)
                        .unwrap_or("Start Center");
                    draw_text(
                        buffer,
                        width,
                        height,
                        slot_rect.x.saturating_add(style.space_2),
                        slot_rect.y.saturating_add(style.space_2 / 2),
                        1,
                        label,
                        style.tokens.text_primary,
                    );
                }
            }
            ShellZoneId::StatusBar => {
                let zone_inset_logical = to_logical_px(style.density_zone_inset, scale);
                let status_snapshot = status_bar_snapshot_for_shell(
                    frame,
                    editor_runtime,
                    enablement_runtime,
                    workspace_lifecycle,
                    recent_work,
                    activity_center,
                    title_context_bar,
                );
                for (slot_id, slot_rect) in
                    frame.slot_rects_within_zone(zone, logical_rect, zone_inset_logical)
                {
                    let slot_rect = to_physical_rect(slot_rect, scale);
                    if clip.is_some_and(|clip_rect| !rect_intersects(slot_rect, clip_rect)) {
                        continue;
                    }

                    fill_rect(buffer, width, height, slot_rect, style.tokens.bg_surface);
                    stroke_rect(
                        buffer,
                        width,
                        height,
                        slot_rect,
                        style.stroke_default,
                        style.tokens.border_default,
                    );

                    let x = slot_rect.x.saturating_add(style.space_2);
                    let y = slot_rect
                        .y
                        .saturating_add(slot_rect.height.saturating_sub(8) / 2);
                    let max_x = slot_rect.right().saturating_sub(style.space_2).max(x);
                    if let Some(paint) =
                        status_bar_slot_paint(slot_id, &status_snapshot, title_context_bar)
                    {
                        let color = if paint.attention {
                            style.status_warning
                        } else {
                            style.tokens.text_primary
                        };
                        draw_text_clamped(
                            buffer,
                            width,
                            height,
                            x,
                            y,
                            1,
                            &paint.label,
                            color,
                            max_x,
                        );
                    } else {
                        let label = shell_slot_label(slot_id);
                        draw_text_clamped(
                            buffer,
                            width,
                            height,
                            x,
                            y,
                            1,
                            label,
                            style.tokens.text_muted,
                            max_x,
                        );
                    }
                }
            }
            ShellZoneId::ActivityRail => {
                let zone_inset_logical = to_logical_px(style.density_zone_inset, scale);
                for (_slot_id, slot_rect) in
                    frame.slot_rects_within_zone(zone, logical_rect, zone_inset_logical)
                {
                    let slot_rect = to_physical_rect(slot_rect, scale);
                    if clip.is_some_and(|clip_rect| !rect_intersects(slot_rect, clip_rect)) {
                        continue;
                    }
                    draw_activity_center_rail(
                        buffer,
                        width,
                        height,
                        slot_rect,
                        &activity_center.snapshot(),
                        &style,
                    );
                }
            }
            ShellZoneId::LeftSidebar => {
                let zone_inset_logical = to_logical_px(style.density_zone_inset, scale);
                for (_slot_id, slot_rect) in
                    frame.slot_rects_within_zone(zone, logical_rect, zone_inset_logical)
                {
                    let slot_rect = to_physical_rect(slot_rect, scale);
                    if clip.is_some_and(|clip_rect| !rect_intersects(slot_rect, clip_rect)) {
                        continue;
                    }
                    draw_explorer_sidebar(
                        buffer,
                        width,
                        height,
                        slot_rect,
                        &editor_runtime.explorer,
                        keybinding_runtime,
                        &style,
                        zone == frame.focused_zone(),
                    );
                }
            }
            _ => {
                let zone_inset_logical = to_logical_px(style.density_zone_inset, scale);
                for (slot_id, slot_rect) in
                    frame.slot_rects_within_zone(zone, logical_rect, zone_inset_logical)
                {
                    let slot_rect = to_physical_rect(slot_rect, scale);
                    if clip.is_some_and(|clip_rect| !rect_intersects(slot_rect, clip_rect)) {
                        continue;
                    }
                    if zone == ShellZoneId::RightInspector
                        && slot_id == "slot.right_inspector.contextual_detail"
                    {
                        draw_docs_help_boundary_card(
                            buffer,
                            width,
                            height,
                            slot_rect,
                            docs_help_boundary_card,
                            keybinding_runtime,
                            &style,
                            zone == frame.focused_zone(),
                        );
                        continue;
                    }
                    fill_rect(buffer, width, height, slot_rect, style.tokens.bg_surface);
                    stroke_rect(
                        buffer,
                        width,
                        height,
                        slot_rect,
                        style.stroke_default,
                        style.tokens.border_default,
                    );
                    draw_shell_slot_placeholder_card(
                        buffer,
                        width,
                        height,
                        slot_rect,
                        slot_id,
                        &title_context_bar.degraded_or_recovery_state.degraded_tokens,
                        &style,
                    );
                }
            }
        }

        if zone == frame.focused_zone() {
            stroke_rect(
                buffer,
                width,
                height,
                rect,
                focus_ring.stroke_px,
                focus_ring.color,
            );
        }

        if zone != ShellZoneId::TitleContextBar && zone != ShellZoneId::StatusBar {
            let zone_label = format!("zone: {}", zone.name());
            draw_text(
                buffer,
                width,
                height,
                rect.x.saturating_add(style.space_2),
                rect.y.saturating_add(style.space_2 / 2),
                1,
                &zone_label,
                style.tokens.text_muted,
            );
        }
    }

    draw_notification_layers(
        buffer,
        width,
        height,
        frame,
        scale,
        &activity_center.notifications,
        now,
        &style,
    );

    let reduced_motion_posture = appearance.reduced_motion_posture();

    if palette.is_open() {
        draw_command_palette_overlay(
            buffer,
            width,
            height,
            scale,
            token_registry,
            reduced_motion_posture,
            now,
            registry,
            frame,
            editor_runtime,
            palette,
            keybinding_runtime,
            enablement_runtime,
            docs_help_boundary_card,
            &style,
            held_modifiers,
        );
    }

    if let Some(overlay) = overlay {
        draw_shell_overlay(
            buffer,
            width,
            height,
            window.scale_factor(),
            token_registry,
            reduced_motion_posture,
            now,
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
        let terminal_keys = keybinding_runtime.shortcuts_label("cmd:terminal.toggle");
        let docs_keys = keybinding_runtime.shortcuts_label("cmd:docs.open_in_browser");
        let enablement_trust_token = enablement_runtime.workspace_trust_state.as_str();
        let theme_label = appearance.theme_class().token();
        let density_label = appearance.density_class().token();
        let motion_label = appearance.reduced_motion_posture().token();
        let active_workspace = recent_work.active_workspace_label().unwrap_or("none");
        let workspace_lifecycle_token = workspace_lifecycle.status_badge_token().unwrap_or("none");
        let watcher_health_token = workspace_lifecycle
            .watcher_health_token()
            .unwrap_or("unknown");
        let hot_index_label = match workspace_lifecycle.hot_index_ready() {
            Some(true) => "ready",
            Some(false) => "warming",
            None => "unknown",
        };
        let recent_work_store = recent_work.store_path.display();
        let activity_snapshot = activity_center.snapshot();
        let activity_store = activity_center.store_path.display();
        let status_item_label = title_context_bar
            .projection_label(SurfaceKind::WorkspaceStatusItem)
            .unwrap_or("Workspace ready");
        let text = format!(
            "{}   theme: {}   density: {}   motion: {}   fallback_modes: [{}]   workspace: {}   runtime_readiness: {}   watcher: {}   hot_index: {}   activity_rows: {}   last_cmd: {}   last_keybinding: {}   enablement: trust={} exec_ctx={} policy={}   keymap: {} ({})   keys: {} palette (resolver)   {} explorer   {} terminal   docs: {} open in browser   Cmd/Ctrl+Shift+R switcher, Enter run, Ctrl+\\\\ split view, Ctrl+Tab next tab, Ctrl+G next group, Ctrl+O new tab, Ctrl+S save, Ctrl+W close tab, Ctrl+Shift+W close group, Ctrl+I keybinding inspector   toggles: Cmd/Ctrl+Shift+T trust, Cmd/Ctrl+Shift+B policy, Cmd/Ctrl+Shift+L theme, Ctrl+Alt+Shift+H high contrast, Cmd/Ctrl+Shift+M density, Cmd/Ctrl+Alt+Shift+M motion   packets: .logs/command_packets   recents: {}   activity: {}",
            status_item_label,
            theme_label,
            density_label,
            motion_label,
            modes,
            active_workspace,
            workspace_lifecycle_token,
            watcher_health_token,
            hot_index_label,
            activity_snapshot.len(),
            last,
            last_keybinding,
            enablement_trust_token,
            exec_ctx,
            policy,
            keybinding_runtime.active_preset.display_name(),
            keybinding_runtime.active_preset.preset_ref(),
            palette_keys,
            keybinding_runtime.shortcuts_label("cmd:explorer.toggle"),
            terminal_keys,
            docs_keys,
            recent_work_store,
            activity_store
        );

        let mut cursor_x = status.x.saturating_add(style.space_2);
        let badge_y = status.y.saturating_add(style.space_2 / 2);
        let badge_max_h = status.height.saturating_sub(1);

        let (lifecycle_label, lifecycle_fg, lifecycle_border, lifecycle_fill) =
            match title_context_bar.workspace_identity.lifecycle_state {
                crate::chrome::title_context_bar::WorkspaceLifecycleState::WorkspaceReady => (
                    "Ready",
                    style.status_success,
                    style.status_success_border,
                    style.status_success_fill,
                ),
                crate::chrome::title_context_bar::WorkspaceLifecycleState::WorkspacePartiallyReady => (
                    "Partially Ready",
                    style.status_warning,
                    style.status_warning_border,
                    style.status_warning_fill,
                ),
                crate::chrome::title_context_bar::WorkspaceLifecycleState::WorkspaceDegraded
                | crate::chrome::title_context_bar::WorkspaceLifecycleState::WorkspaceReadOnlyDegraded
                | crate::chrome::title_context_bar::WorkspaceLifecycleState::WorkspaceRecovering => (
                    "Degraded",
                    style.status_danger,
                    style.status_danger_border,
                    style.status_danger_fill,
                ),
                crate::chrome::title_context_bar::WorkspaceLifecycleState::WorkspaceOpening => (
                    "Opening",
                    style.status_warning,
                    style.status_warning_border,
                    style.status_warning_fill,
                ),
                crate::chrome::title_context_bar::WorkspaceLifecycleState::WorkspaceTrustEvaluating => (
                    "Trust Evaluating",
                    style.status_warning,
                    style.status_warning_border,
                    style.status_warning_fill,
                ),
                crate::chrome::title_context_bar::WorkspaceLifecycleState::WorkspaceClosing => (
                    "Closing",
                    style.status_warning,
                    style.status_warning_border,
                    style.status_warning_fill,
                ),
                crate::chrome::title_context_bar::WorkspaceLifecycleState::WorkspaceDiscovered => (
                    "Discovered",
                    style.tokens.text_muted,
                    style.tokens.border_default,
                    style.tokens.bg_surface,
                ),
                crate::chrome::title_context_bar::WorkspaceLifecycleState::WorkspaceClosed => (
                    "Closed",
                    style.tokens.text_muted,
                    style.tokens.border_default,
                    style.tokens.bg_surface,
                ),
            };

        let badge_rect = draw_status_badge(
            buffer,
            width,
            height,
            cursor_x,
            badge_y,
            badge_max_h,
            style,
            lifecycle_label,
            lifecycle_fg,
            lifecycle_border,
            lifecycle_fill,
        );
        cursor_x = cursor_x
            .saturating_add(badge_rect.width)
            .saturating_add(style.space_2);

        let (trust_badge_text, trust_fg, trust_border, trust_fill) =
            match title_context_bar.trust_identity.trust_state {
                crate::chrome::title_context_bar::TrustState::Trusted => (
                    "Trusted",
                    style.status_success,
                    style.status_success_border,
                    style.status_success_fill,
                ),
                crate::chrome::title_context_bar::TrustState::Restricted => (
                    "Restricted",
                    style.status_warning,
                    style.status_warning_border,
                    style.status_warning_fill,
                ),
                _ => (
                    "Untrusted",
                    style.status_danger,
                    style.status_danger_border,
                    style.status_danger_fill,
                ),
            };

        let trust_badge_rect = draw_status_badge(
            buffer,
            width,
            height,
            cursor_x,
            badge_y,
            badge_max_h,
            style,
            trust_badge_text,
            trust_fg,
            trust_border,
            trust_fill,
        );
        cursor_x = cursor_x
            .saturating_add(trust_badge_rect.width)
            .saturating_add(style.space_2);

        for token in title_context_bar
            .degraded_or_recovery_state
            .degraded_tokens
            .iter()
            .copied()
            .take(2)
        {
            let label = token.label();
            let (fg, border, fill) = (
                style.status_warning,
                style.status_warning_border,
                style.status_warning_fill,
            );
            let token_rect = draw_status_badge(
                buffer,
                width,
                height,
                cursor_x,
                badge_y,
                badge_max_h,
                style,
                label,
                fg,
                border,
                fill,
            );
            cursor_x = cursor_x
                .saturating_add(token_rect.width)
                .saturating_add(style.space_2);
        }

        draw_text(
            buffer,
            width,
            height,
            cursor_x,
            badge_y,
            1,
            &text,
            style.tokens.text_muted,
        );
    }
}

fn draw_notification_layers(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    frame: &DesktopFrame,
    scale_factor: f64,
    notifications: &NotificationSurfaceRuntimeState,
    now: Instant,
    style: &ShellRenderStyle,
) {
    if let Some(banner) = notifications.banners.last() {
        if let Some(rect) = notification_banner_rect(frame, scale_factor) {
            draw_notification_banner(buffer, width, height, rect, banner, style);
        }
    }

    let toast_rects = notification_toast_rects(frame, scale_factor, notifications.toasts.len());
    for (toast, rect) in notifications.toasts.iter().zip(toast_rects.into_iter()) {
        let alpha = toast_alpha(toast, now);
        if alpha > 0.0 {
            draw_notification_toast(buffer, width, height, rect, toast, alpha, style);
        }
    }
}

fn draw_notification_toast(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    rect: Rect,
    toast: &LiveToastNotification,
    alpha: f32,
    style: &ShellRenderStyle,
) {
    let (fg, border, fill) = notification_tone(toast.row.severity_class, style);
    fill_rect(buffer, width, height, rect, scale_alpha(fill, alpha));
    stroke_rect(
        buffer,
        width,
        height,
        rect,
        style.stroke_default,
        scale_alpha(border, alpha),
    );

    let x = rect.x.saturating_add(style.space_3);
    let mut y = rect.y.saturating_add(style.space_2);
    let max_x = rect.right().saturating_sub(style.space_3).max(x);
    draw_text_clamped(
        buffer,
        width,
        height,
        x,
        y,
        1,
        &toast.row.summary_label,
        scale_alpha(fg, alpha),
        max_x,
    );
    y = y.saturating_add(12);
    draw_text_clamped(
        buffer,
        width,
        height,
        x,
        y,
        1,
        "Details in Activity Center",
        scale_alpha(style.tokens.text_secondary, alpha),
        max_x,
    );
}

fn draw_notification_banner(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    rect: Rect,
    banner: &LiveBannerNotification,
    style: &ShellRenderStyle,
) {
    let (fg, border, fill) = notification_tone(banner.row.severity_class, style);
    fill_rect(buffer, width, height, rect, fill);
    stroke_rect(buffer, width, height, rect, style.stroke_default, border);

    let x = rect.x.saturating_add(style.space_3);
    let y = rect.y.saturating_add(rect.height.saturating_sub(8) / 2);
    let close_w = 8u32;
    let close_x = rect
        .right()
        .saturating_sub(style.space_3)
        .saturating_sub(close_w);
    let max_x = close_x.saturating_sub(style.space_3).max(x);
    let label = if let Some(action) = banner.row.primary_action.as_ref() {
        format!("{} - {}", banner.row.summary_label, action.label)
    } else {
        banner.row.summary_label.clone()
    };
    draw_text_clamped(buffer, width, height, x, y, 1, &label, fg, max_x);
    draw_text(buffer, width, height, close_x, y, 1, "x", fg);
}

fn notification_tone(
    severity: SeverityClass,
    style: &ShellRenderStyle,
) -> (ColorRgba, ColorRgba, ColorRgba) {
    match severity {
        SeverityClass::Success => (
            style.status_success,
            style.status_success_border,
            style.status_success_fill,
        ),
        SeverityClass::Warning | SeverityClass::Degraded => (
            style.status_warning,
            style.status_warning_border,
            style.status_warning_fill,
        ),
        SeverityClass::Error | SeverityClass::Blocking | SeverityClass::Critical => (
            style.status_danger,
            style.status_danger_border,
            style.status_danger_fill,
        ),
        SeverityClass::Info => (
            style.tokens.text_primary,
            style.tokens.border_strong,
            style.tokens.bg_raised,
        ),
    }
}

fn notification_toast_rects(frame: &DesktopFrame, scale_factor: f64, count: usize) -> Vec<Rect> {
    let Some(main) = frame.layout().zone(ShellZoneId::MainWorkspace) else {
        return Vec::new();
    };
    let main = to_physical_rect(main, scale_factor);
    if main.is_empty() || count == 0 {
        return Vec::new();
    }

    let margin = scaled_notification_px(16, scale_factor);
    let gap = scaled_notification_px(8, scale_factor);
    let toast_h = scaled_notification_px(48, scale_factor);
    let preferred_w = scaled_notification_px(440, scale_factor);
    let available_w = main.width.saturating_sub(margin.saturating_mul(2));
    let toast_w = preferred_w.min(available_w).max(available_w.min(220));
    let x = main.right().saturating_sub(margin).saturating_sub(toast_w);
    let bottom = main.bottom().saturating_sub(margin);

    let mut rects = Vec::with_capacity(count);
    for idx in 0..count {
        let stack_from_bottom = count.saturating_sub(1).saturating_sub(idx) as u32;
        let y = bottom.saturating_sub(
            toast_h.saturating_mul(stack_from_bottom.saturating_add(1))
                + gap.saturating_mul(stack_from_bottom),
        );
        rects.push(Rect::new(x, y, toast_w, toast_h));
    }
    rects
}

fn notification_toast_hit_index(
    frame: &DesktopFrame,
    scale_factor: f64,
    count: usize,
    x: u32,
    y: u32,
) -> Option<usize> {
    notification_toast_rects(frame, scale_factor, count)
        .iter()
        .enumerate()
        .rev()
        .find_map(|(idx, rect)| point_in_rect(x, y, *rect).then_some(idx))
}

fn notification_banner_rect(frame: &DesktopFrame, scale_factor: f64) -> Option<Rect> {
    let title = to_physical_rect(frame.layout().title_context_bar, scale_factor);
    if title.is_empty() {
        return None;
    }
    let margin = scaled_notification_px(8, scale_factor);
    let banner_h = scaled_notification_px(18, scale_factor).min(title.height);
    let banner_w = title.width.saturating_sub(margin.saturating_mul(2));
    if banner_w == 0 {
        return None;
    }
    Some(Rect::new(
        title.x.saturating_add(margin),
        title.bottom().saturating_sub(banner_h),
        banner_w,
        banner_h,
    ))
}

fn scaled_notification_px(value: u32, scale_factor: f64) -> u32 {
    if !scale_factor.is_finite() || scale_factor <= 0.0 {
        return value;
    }
    ((value as f64) * scale_factor).round().max(1.0) as u32
}

fn draw_activity_center_rail(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    rect: Rect,
    snapshot: &ActivityCenterSnapshot,
    style: &ShellRenderStyle,
) {
    if rect.is_empty() || rect.width < 8 || rect.height < 8 {
        return;
    }

    fill_rect(buffer, width, height, rect, style.tokens.bg_surface);
    stroke_rect(
        buffer,
        width,
        height,
        rect,
        style.stroke_default,
        style.tokens.border_default,
    );

    let inset = style.space_2.max(2);
    let max_x = rect.right().saturating_sub(inset).max(rect.x);
    let mut y = rect.y.saturating_add(inset / 2);
    draw_text_clamped(
        buffer,
        width,
        height,
        rect.x.saturating_add(inset),
        y,
        1,
        "Activity",
        style.tokens.text_primary,
        max_x,
    );
    y = y.saturating_add(14);

    if snapshot.rows.is_empty() {
        draw_text_clamped(
            buffer,
            width,
            height,
            rect.x.saturating_add(inset),
            y,
            1,
            "No rows",
            style.tokens.text_muted,
            max_x,
        );
        return;
    }

    let row_gap = style.space_2.max(2);
    let row_h = 48u32;
    for row in snapshot.rows.iter().rev() {
        if y >= rect.bottom().saturating_sub(inset) {
            break;
        }
        let available_h = rect.bottom().saturating_sub(y).saturating_sub(inset);
        let row_rect = Rect::new(
            rect.x.saturating_add(inset / 2),
            y,
            rect.width.saturating_sub(inset),
            row_h.min(available_h),
        );
        if row_rect.height < 18 {
            break;
        }
        draw_activity_center_row(buffer, width, height, row_rect, row, style);
        y = y.saturating_add(row_rect.height).saturating_add(row_gap);
    }
}

fn handle_activity_rail_click(
    frame: &mut DesktopFrame,
    command_runtime: &mut CommandRuntimeState,
    activity_center: &ActivityCenterRuntimeState,
    scale_factor: f64,
    x: u32,
    y: u32,
) -> bool {
    let rail = to_physical_rect(frame.layout().activity_rail, scale_factor);
    if !point_in_rect(x, y, rail) {
        return false;
    }

    let snapshot = activity_center.snapshot();
    let Some(row) = activity_row_at_point(rail, &snapshot, x, y) else {
        return false;
    };

    if let Some(zone) = originating_zone_for_activity_row(row) {
        if frame.layout().zone(zone).is_some() {
            frame.focus_zone(zone);
            command_runtime
                .note_non_command_action(format!("activity opened - {}", row.summary_label));
            return true;
        }
    }

    command_runtime.note_non_command_action(format!(
        "activity selected - {} (origin unavailable)",
        row.summary_label
    ));
    true
}

fn activity_row_at_point<'a>(
    rect: Rect,
    snapshot: &'a ActivityCenterSnapshot,
    x: u32,
    y: u32,
) -> Option<&'a ActivityCenterRow> {
    let inset = 4u32;
    let mut row_y = rect.y.saturating_add(inset / 2).saturating_add(14);
    let row_h = 48u32;
    let row_gap = 4u32;
    for row in snapshot.rows.iter().rev() {
        let row_rect = Rect::new(
            rect.x.saturating_add(inset / 2),
            row_y,
            rect.width.saturating_sub(inset),
            row_h,
        );
        if point_in_rect(x, y, row_rect) {
            return Some(row);
        }
        row_y = row_y.saturating_add(row_h).saturating_add(row_gap);
    }
    None
}

fn start_center_action_at_point(
    frame: &DesktopFrame,
    scale_factor: f64,
    style: &ShellRenderStyle,
    text_runtime: &mut ShellTextRuntime,
    row_count: usize,
    x: u32,
    y: u32,
) -> Option<(PaneId, usize)> {
    for group in frame.editor_group_layouts() {
        if group.tab_count != 0 {
            continue;
        }
        let group_rect = to_physical_rect(group.rect, scale_factor);
        if !point_in_rect(x, y, group_rect) {
            continue;
        }
        let rects = start_center_action_row_rects(group_rect, style, text_runtime, row_count);
        for (idx, rect) in rects.into_iter().enumerate() {
            if point_in_rect(x, y, rect) {
                return Some((group.group_id, idx));
            }
        }
    }
    None
}

fn start_center_action_row_rects(
    rect: Rect,
    style: &ShellRenderStyle,
    text_runtime: &mut ShellTextRuntime,
    row_count: usize,
) -> Vec<Rect> {
    let padding = style.density_zone_inset;
    let card = Rect::new(
        rect.x.saturating_add(padding),
        rect.y.saturating_add(padding),
        rect.width.saturating_sub(padding.saturating_mul(2)),
        rect.height.saturating_sub(padding.saturating_mul(2)),
    );
    if card.is_empty() {
        return Vec::new();
    }

    let content_padding = style.density_panel_padding;
    let header_x = card.x.saturating_add(content_padding);
    let mut y = card.y.saturating_add(content_padding);
    let (_, header_h) = ui_primary_ascent_and_height(text_runtime, 20.0);
    y = y.saturating_add(header_h).saturating_add(style.space_2);
    let (_, subtitle_h) = ui_primary_ascent_and_height(text_runtime, 14.0);
    y = y
        .saturating_add(subtitle_h)
        .saturating_add(style.density_gutter);

    let (_, label_h) = ui_primary_ascent_and_height(text_runtime, 14.0);
    let (_, detail_h) = ui_primary_ascent_and_height(text_runtime, 12.0);
    let text_block_h = label_h.saturating_add(detail_h);
    let row_height = style
        .density_row_height
        .saturating_mul(2)
        .max(text_block_h.saturating_add(style.space_2.saturating_mul(2)));
    let row_gap = style.density_gutter;
    let row_width = card.width.saturating_sub(content_padding.saturating_mul(2));

    let mut rects = Vec::new();
    for _ in 0..row_count {
        let row_rect = Rect::new(header_x, y, row_width, row_height);
        if row_rect.bottom().saturating_add(content_padding) > card.bottom() {
            break;
        }
        rects.push(row_rect);
        y = y.saturating_add(row_height).saturating_add(row_gap);
    }
    rects
}

fn originating_zone_for_activity_row(row: &ActivityCenterRow) -> Option<ShellZoneId> {
    match row.source_subsystem {
        SourceSubsystem::Editor | SourceSubsystem::VfsSave | SourceSubsystem::Shell => {
            Some(ShellZoneId::MainWorkspace)
        }
        SourceSubsystem::Indexer | SourceSubsystem::PaletteAndSearch => {
            Some(ShellZoneId::MainWorkspace)
        }
        SourceSubsystem::Terminal
        | SourceSubsystem::TaskRunner
        | SourceSubsystem::TestRunner
        | SourceSubsystem::BuildSystem
        | SourceSubsystem::DebugSession => Some(ShellZoneId::BottomPanel),
        SourceSubsystem::DocsHelpServiceHealth => Some(ShellZoneId::RightInspector),
        _ => None,
    }
}

fn point_in_rect(x: u32, y: u32, rect: Rect) -> bool {
    x >= rect.x && x < rect.right() && y >= rect.y && y < rect.bottom()
}

fn draw_activity_center_row(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    rect: Rect,
    row: &ActivityCenterRow,
    style: &ShellRenderStyle,
) {
    fill_rect(buffer, width, height, rect, style.tokens.bg_raised);
    stroke_rect(
        buffer,
        width,
        height,
        rect,
        style.stroke_default,
        activity_lifecycle_border(row.lifecycle_class, style),
    );

    let inset = style.space_2.max(2);
    let x = rect.x.saturating_add(inset / 2);
    let max_x = rect.right().saturating_sub(inset / 2).max(x);
    let chip_y = rect.y.saturating_add(inset / 2);
    draw_activity_chip(buffer, width, height, x, chip_y, max_x, row, style);

    let label_y = chip_y.saturating_add(12);
    draw_text_clamped(
        buffer,
        width,
        height,
        x,
        label_y,
        1,
        &row.summary_label,
        style.tokens.text_secondary,
        max_x,
    );

    let age_y = label_y.saturating_add(10);
    if age_y.saturating_add(8) <= rect.bottom() {
        let age = activity_age_label(&row.last_observed_at);
        draw_text_clamped(
            buffer,
            width,
            height,
            x,
            age_y,
            1,
            &age,
            style.tokens.text_muted,
            max_x,
        );
    }
}

fn draw_activity_chip(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    max_x: u32,
    row: &ActivityCenterRow,
    style: &ShellRenderStyle,
) {
    if max_x <= x {
        return;
    }
    let chip_rect = Rect::new(x, y, max_x.saturating_sub(x).min(92), 10);
    let (fg, border, fill) = activity_lifecycle_colors(row.lifecycle_class, style);
    fill_rect(buffer, width, height, chip_rect, fill);
    stroke_rect(
        buffer,
        width,
        height,
        chip_rect,
        style.stroke_default,
        border,
    );
    draw_text_clamped(
        buffer,
        width,
        height,
        chip_rect.x.saturating_add(2),
        chip_rect.y.saturating_add(1),
        1,
        row.lifecycle_label.as_str(),
        fg,
        chip_rect.right().saturating_sub(2),
    );
}

fn activity_lifecycle_border(
    lifecycle: ActivityRowLifecycleClass,
    style: &ShellRenderStyle,
) -> ColorRgba {
    let (_, border, _) = activity_lifecycle_colors(lifecycle, style);
    border
}

fn activity_lifecycle_colors(
    lifecycle: ActivityRowLifecycleClass,
    style: &ShellRenderStyle,
) -> (ColorRgba, ColorRgba, ColorRgba) {
    match lifecycle {
        ActivityRowLifecycleClass::Completed => (
            style.status_success,
            style.status_success_border,
            style.status_success_fill,
        ),
        ActivityRowLifecycleClass::Failed | ActivityRowLifecycleClass::Cancelled => (
            style.status_danger,
            style.status_danger_border,
            style.status_danger_fill,
        ),
        ActivityRowLifecycleClass::Preparing | ActivityRowLifecycleClass::Running => (
            style.status_warning,
            style.status_warning_border,
            style.status_warning_fill,
        ),
    }
}

fn activity_age_label(observed_at: &str) -> String {
    match rfc3339_epoch_seconds(observed_at) {
        Some(observed) => {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64;
            let elapsed = now.saturating_sub(observed).max(0);
            format!("age {}", compact_duration_label(elapsed))
        }
        None => "age unknown".to_string(),
    }
}

fn compact_duration_label(seconds: i64) -> String {
    if seconds < 60 {
        format!("{seconds}s")
    } else if seconds < 3_600 {
        format!("{}m", seconds / 60)
    } else if seconds < 86_400 {
        format!("{}h", seconds / 3_600)
    } else {
        format!("{}d", seconds / 86_400)
    }
}

fn rfc3339_epoch_seconds(value: &str) -> Option<i64> {
    if value.len() < 20 || !value.ends_with('Z') {
        return None;
    }
    let year = value.get(0..4)?.parse::<i32>().ok()?;
    let month = value.get(5..7)?.parse::<u32>().ok()?;
    let day = value.get(8..10)?.parse::<u32>().ok()?;
    let hour = value.get(11..13)?.parse::<u32>().ok()?;
    let minute = value.get(14..16)?.parse::<u32>().ok()?;
    let second = value.get(17..19)?.parse::<u32>().ok()?;
    if value.as_bytes().get(4) != Some(&b'-')
        || value.as_bytes().get(7) != Some(&b'-')
        || value.as_bytes().get(10) != Some(&b'T')
        || value.as_bytes().get(13) != Some(&b':')
        || value.as_bytes().get(16) != Some(&b':')
        || !(1..=12).contains(&month)
        || !(1..=31).contains(&day)
        || hour > 23
        || minute > 59
        || second > 60
    {
        return None;
    }
    let days = days_from_civil(year, month, day)?;
    Some(days * 86_400 + i64::from(hour) * 3_600 + i64::from(minute) * 60 + i64::from(second))
}

fn days_from_civil(year: i32, month: u32, day: u32) -> Option<i64> {
    let year = i64::from(year) - if month <= 2 { 1 } else { 0 };
    let era = if year >= 0 { year } else { year - 399 } / 400;
    let yoe = year - era * 400;
    let month = i64::from(month);
    let day = i64::from(day);
    let doy = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    if !(0..=365).contains(&doy) {
        return None;
    }
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    Some(era * 146_097 + doe - 719_468)
}

fn draw_shell_slot_placeholder_card(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    rect: Rect,
    slot_id: &str,
    degraded_tokens: &[DegradedStateToken],
    style: &ShellRenderStyle,
) {
    if rect.is_empty() || rect.width < 8 || rect.height < 8 {
        return;
    }

    let card = ShellPlaceholderCard::for_slot(slot_id, degraded_tokens);
    let inset_x = rect.x.saturating_add(style.space_2);
    let max_x = rect.right().saturating_sub(style.space_2).max(inset_x);
    let title_y = rect.y.saturating_add(style.space_2 / 2);

    draw_text_clamped(
        buffer,
        width,
        height,
        inset_x,
        title_y,
        1,
        card.title,
        style.tokens.text_primary,
        max_x,
    );

    let summary_y = title_y.saturating_add(8).saturating_add(style.space_2 / 2);
    draw_text_clamped(
        buffer,
        width,
        height,
        inset_x,
        summary_y,
        1,
        card.summary,
        style.tokens.text_muted,
        max_x,
    );

    let badge_y = summary_y
        .saturating_add(8)
        .saturating_add(style.space_2 / 2);
    if badge_y >= rect.bottom() {
        return;
    }
    let badge_max_h = rect.bottom().saturating_sub(badge_y);
    let mut cursor_x = inset_x;
    for token in card.degraded_tokens() {
        let badge_rect = draw_status_badge(
            buffer,
            width,
            height,
            cursor_x,
            badge_y,
            badge_max_h,
            style,
            token.label(),
            style.status_warning,
            style.status_warning_border,
            style.status_warning_fill,
        );
        cursor_x = cursor_x
            .saturating_add(badge_rect.width)
            .saturating_add(style.space_2);
        if cursor_x >= max_x {
            break;
        }
    }
}

fn draw_explorer_sidebar(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    rect: Rect,
    explorer: &ExplorerViewRuntime,
    keybinding_runtime: &KeybindingRuntimeState,
    style: &ShellRenderStyle,
    focused: bool,
) {
    if rect.is_empty() || rect.width < 8 || rect.height < 8 {
        return;
    }
    fill_rect(buffer, width, height, rect, style.tokens.bg_surface);
    stroke_rect(
        buffer,
        width,
        height,
        rect,
        style.stroke_default,
        style.tokens.border_default,
    );

    let padding = style.density_panel_padding.min(rect.width / 3);
    let inner = Rect::new(
        rect.x.saturating_add(padding),
        rect.y.saturating_add(padding),
        rect.width.saturating_sub(padding.saturating_mul(2)),
        rect.height.saturating_sub(padding.saturating_mul(2)),
    );
    if inner.is_empty() {
        return;
    }

    let max_x = inner.right();
    let mut y = inner.y;
    draw_text_clamped(
        buffer,
        width,
        height,
        inner.x,
        y,
        2,
        "Explorer",
        style.tokens.text_primary,
        max_x,
    );
    y = y.saturating_add(18);

    let shortcut = keybinding_runtime.shortcuts_label("cmd:explorer.toggle");
    let status = if let Some(root) = explorer.workspace_root.as_ref() {
        format!(
            "{}   watcher:{}",
            root.file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("workspace"),
            explorer.watcher_health().as_str()
        )
    } else {
        format!("No folder open   {}", shortcut)
    };
    draw_text_clamped(
        buffer,
        width,
        height,
        inner.x,
        y,
        1,
        &status,
        style.tokens.text_muted,
        max_x,
    );
    y = y.saturating_add(12).saturating_add(style.density_gutter);

    let row_h = style.density_row_height.max(18);
    let glyph_y_offset = row_h.saturating_sub(8) / 2;
    let rows = explorer.visible_rows();
    if rows.is_empty() {
        draw_text_clamped(
            buffer,
            width,
            height,
            inner.x,
            y,
            1,
            "Workspace tree unavailable",
            style.tokens.text_muted,
            max_x,
        );
        return;
    }

    for row in rows {
        if y.saturating_add(row_h) > inner.bottom() {
            break;
        }
        let row_rect = Rect::new(inner.x, y, inner.width, row_h);
        if row.is_selected {
            let chrome_state = if focused {
                ComponentStates::SELECTED | ComponentStates::FOCUS_VISIBLE
            } else {
                ComponentStates::SELECTED
            };
            let chrome = style
                .component_states
                .chrome_style(ComponentSurfaceTone::Surface, chrome_state);
            fill_rect(buffer, width, height, row_rect, chrome.fill);
            stroke_rect(
                buffer,
                width,
                height,
                row_rect,
                chrome.border_stroke_px,
                chrome.border,
            );
        }

        let indent = row.depth.saturating_mul(style.space_3).min(inner.width / 2);
        let marker = match row.kind {
            ExplorerNodeKind::RootMount | ExplorerNodeKind::Directory => {
                if row.is_expanded {
                    "v"
                } else {
                    ">"
                }
            }
            ExplorerNodeKind::GeneratedArtifact => "G",
            ExplorerNodeKind::SpecialFile => "S",
            ExplorerNodeKind::VirtualDocument => "V",
            ExplorerNodeKind::File => "-",
        };
        let readiness = if row.readiness.is_partial() {
            format!(" [{}]", row.readiness.as_str())
        } else {
            String::new()
        };
        let label = format!("{marker} {}{}", row.display_label, readiness);
        let text_x = inner.x.saturating_add(indent);
        draw_text_clamped(
            buffer,
            width,
            height,
            text_x,
            y.saturating_add(glyph_y_offset),
            1,
            &label,
            if row.is_selected {
                style.tokens.text_primary
            } else {
                style.tokens.text_secondary
            },
            max_x,
        );
        y = y.saturating_add(row_h);
    }

    if let Some(err) = explorer.last_error() {
        let footer_y = inner.bottom().saturating_sub(10);
        draw_text_clamped(
            buffer,
            width,
            height,
            inner.x,
            footer_y,
            1,
            err,
            style.status_danger,
            max_x,
        );
    }
}

fn draw_terminal_panel(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    rect: Rect,
    terminal_runtime: &mut TerminalPaneRuntimeState,
    keybinding_runtime: &KeybindingRuntimeState,
    style: &ShellRenderStyle,
    focused: bool,
) {
    if rect.is_empty() || rect.width < 8 || rect.height < 8 {
        return;
    }
    let _ = terminal_runtime.drain_outputs(&mono_timestamp_now());

    fill_rect(buffer, width, height, rect, style.tokens.bg_surface);
    stroke_rect(
        buffer,
        width,
        height,
        rect,
        style.stroke_default,
        style.tokens.border_default,
    );

    let inset = style.space_2.max(2);
    let max_x = rect.right().saturating_sub(inset).max(rect.x);
    let title_x = rect.x.saturating_add(inset);
    let mut y = rect.y.saturating_add(inset / 2);
    let toggle_keys = keybinding_runtime.shortcuts_label("cmd:terminal.toggle");
    let title = match terminal_runtime.active_workspace_id() {
        Some(workspace_id) => format!("Terminal  workspace={workspace_id}  toggle={toggle_keys}"),
        None => format!("Terminal  open a workspace to start a shell  toggle={toggle_keys}"),
    };
    draw_text_clamped(
        buffer,
        width,
        height,
        title_x,
        y,
        1,
        &title,
        style.tokens.text_primary,
        max_x,
    );
    y = y.saturating_add(12);

    let snapshot = terminal_runtime.snapshot();
    let active_tab_id = snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.active_tab_id.as_ref())
        .or(terminal_runtime.active_session_id.as_ref());

    if let Some(snapshot) = snapshot.as_ref() {
        let tab_y = y;
        let mut cursor_x = title_x;
        if snapshot.tabs.is_empty() {
            draw_text_clamped(
                buffer,
                width,
                height,
                cursor_x,
                tab_y,
                1,
                "No terminal session",
                style.tokens.text_muted,
                max_x,
            );
        } else {
            for tab in snapshot.tabs.iter().take(4) {
                let label = terminal_tab_badge_label(tab, active_tab_id == Some(&tab.session_id));
                let badge = draw_status_badge(
                    buffer,
                    width,
                    height,
                    cursor_x,
                    tab_y,
                    18,
                    style,
                    &label,
                    if active_tab_id == Some(&tab.session_id) {
                        style.tokens.text_primary
                    } else {
                        style.tokens.text_muted
                    },
                    if active_tab_id == Some(&tab.session_id) {
                        style.tokens.accent_brand
                    } else {
                        style.tokens.border_default
                    },
                    if active_tab_id == Some(&tab.session_id) {
                        style.tokens.bg_hover
                    } else {
                        style.tokens.bg_raised
                    },
                );
                cursor_x = cursor_x
                    .saturating_add(badge.width)
                    .saturating_add(style.space_2);
                if cursor_x >= max_x {
                    break;
                }
            }
        }
    }
    y = y.saturating_add(22);

    if let Some(card) = terminal_runtime.active_host_boundary_card() {
        let boundary_label = if card.current_boundary_cue_visible {
            card.current_boundary_cue_label.as_str()
        } else {
            "Local desktop"
        };
        let degraded = card
            .current_degraded_token
            .as_deref()
            .map(|token| format!("  state={token}"))
            .unwrap_or_default();
        let label = format!(
            "host boundary: {}  cue={}{}",
            boundary_label, card.current_boundary_cue_token, degraded
        );
        draw_text_clamped(
            buffer,
            width,
            height,
            title_x,
            y,
            1,
            &label,
            if card.has_invariant_violations {
                style.status_danger
            } else {
                style.tokens.text_secondary
            },
            max_x,
        );
    } else {
        draw_text_clamped(
            buffer,
            width,
            height,
            title_x,
            y,
            1,
            "host boundary: waiting for workspace",
            style.tokens.text_muted,
            max_x,
        );
    }
    y = y.saturating_add(14);

    let viewport = Rect::new(
        rect.x.saturating_add(inset),
        y,
        rect.width.saturating_sub(inset.saturating_mul(2)),
        rect.bottom().saturating_sub(y).saturating_sub(inset),
    );
    if viewport.is_empty() {
        return;
    }
    fill_rect(buffer, width, height, viewport, style.tokens.bg_canvas);
    stroke_rect(
        buffer,
        width,
        height,
        viewport,
        style.stroke_default,
        style.tokens.border_default,
    );
    if focused {
        let caret = Rect::new(
            viewport.x.saturating_add(style.space_2),
            viewport.bottom().saturating_sub(12).max(viewport.y),
            6,
            10,
        );
        fill_rect(
            buffer,
            width,
            height,
            caret,
            style.tokens.accent_interactive,
        );
    }

    let line_h = 10u32;
    let max_lines = (viewport.height / line_h).saturating_sub(1).max(1) as usize;
    let lines = terminal_view_lines(terminal_runtime.active_output_text(), max_lines);
    if lines.is_empty() {
        let empty = if terminal_runtime.active_workspace_id().is_some() {
            "Shell starting..."
        } else {
            "Open a workspace, then toggle the terminal."
        };
        draw_text_clamped(
            buffer,
            width,
            height,
            viewport.x.saturating_add(style.space_2),
            viewport.y.saturating_add(style.space_2),
            1,
            empty,
            style.tokens.text_muted,
            viewport.right().saturating_sub(style.space_2),
        );
        return;
    }

    let mut line_y = viewport.y.saturating_add(style.space_2);
    let line_max_x = viewport.right().saturating_sub(style.space_2);
    for line in lines {
        if line_y.saturating_add(line_h) > viewport.bottom() {
            break;
        }
        draw_text_clamped(
            buffer,
            width,
            height,
            viewport.x.saturating_add(style.space_2),
            line_y,
            1,
            &line,
            style.tokens.text_primary,
            line_max_x,
        );
        line_y = line_y.saturating_add(line_h);
    }
}

fn terminal_tab_badge_label(tab: &TerminalPaneTabRecord, active: bool) -> String {
    let mut label = String::new();
    if active {
        label.push('[');
    }
    label.push_str(&tab.display_title);
    if active {
        label.push(']');
    }
    label.push(' ');
    label.push_str(&tab.target_badge);
    if let Some(token) = tab.degraded_token.as_deref() {
        label.push(' ');
        label.push_str(token);
    }
    const MAX_LABEL_CHARS: usize = 28;
    if label.chars().count() <= MAX_LABEL_CHARS {
        return label;
    }
    let mut truncated = label
        .chars()
        .take(MAX_LABEL_CHARS.saturating_sub(1))
        .collect::<String>();
    truncated.push('~');
    truncated
}

fn terminal_view_lines(text: &str, max_lines: usize) -> Vec<String> {
    if max_lines == 0 || text.trim().is_empty() {
        return Vec::new();
    }
    let mut lines: Vec<String> = text
        .lines()
        .map(|line| {
            let trimmed = line.trim_end_matches('\0');
            if trimmed.is_empty() {
                String::new()
            } else {
                trimmed.to_string()
            }
        })
        .collect();
    while lines.first().is_some_and(|line| line.is_empty()) {
        lines.remove(0);
    }
    if lines.len() > max_lines {
        lines.split_off(lines.len().saturating_sub(max_lines))
    } else {
        lines
    }
}

fn draw_status_badge(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    max_height: u32,
    style: &ShellRenderStyle,
    text: &str,
    fg: ColorRgba,
    border: ColorRgba,
    fill: ColorRgba,
) -> Rect {
    let badge_scale = 1u32;
    let badge_char_w = 8u32.saturating_mul(badge_scale);
    let badge_padding = style.space_2 / 2;
    let badge_w = badge_char_w
        .saturating_mul(text.len() as u32)
        .saturating_add(badge_padding.saturating_mul(2));
    let badge_h = 8u32
        .saturating_mul(badge_scale)
        .saturating_add(badge_padding.saturating_mul(2));
    let rect = Rect::new(x, y, badge_w, badge_h.min(max_height));
    fill_rect(buffer, width, height, rect, fill);
    stroke_rect(buffer, width, height, rect, style.stroke_default, border);
    draw_text(
        buffer,
        width,
        height,
        rect.x.saturating_add(badge_padding),
        rect.y.saturating_add(badge_padding),
        badge_scale,
        text,
        fg,
    );
    rect
}

#[derive(Debug, Clone)]
struct CommandDiagnosticsOverlay {
    record: CommandDiagnosticsSheetRecord,
}

#[derive(Debug, Clone)]
struct SettingsOverlayRow {
    setting_id: String,
    label: &'static str,
    value: String,
    source_scope: String,
    source_label: String,
    lock_state: String,
    lock_reason: String,
}

impl SettingsOverlayRow {
    fn from_effective(label: &'static str, effective: EffectiveValue) -> Self {
        Self {
            setting_id: effective.setting_id,
            label,
            value: effective.value.preview(),
            source_scope: effective.winning_scope.as_str().to_string(),
            source_label: effective.source_label,
            lock_state: effective.lock_state.as_str().to_string(),
            lock_reason: effective.lock_reason.as_str().to_string(),
        }
    }
}

#[derive(Debug, Clone)]
struct SettingsOverlay {
    rows: Vec<SettingsOverlayRow>,
    selection: usize,
    status_line: Option<String>,
}

impl SettingsOverlay {
    fn new(appearance: &AppearanceRuntimeState) -> Self {
        Self {
            rows: appearance.settings_rows(),
            selection: 0,
            status_line: None,
        }
    }

    fn refresh(&mut self, appearance: &AppearanceRuntimeState) {
        self.rows = appearance.settings_rows();
        if self.rows.is_empty() {
            self.selection = 0;
        } else {
            self.selection = self.selection.min(self.rows.len().saturating_sub(1));
        }
    }

    fn select_next(&mut self) {
        if self.rows.is_empty() {
            return;
        }
        self.selection = (self.selection + 1).min(self.rows.len().saturating_sub(1));
    }

    fn select_prev(&mut self) {
        self.selection = self.selection.saturating_sub(1);
    }

    fn selected_row(&self) -> Option<&SettingsOverlayRow> {
        self.rows.get(self.selection)
    }

    fn next_value_decision(
        &self,
        direction: SettingsCycleDirection,
    ) -> Option<SettingsOverlayDecision> {
        let row = self.selected_row()?;
        let next = next_setting_value(row.setting_id.as_str(), row.value.as_str(), direction)?;
        Some(SettingsOverlayDecision {
            setting_id: row.setting_id.clone(),
            value: next.to_string(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SettingsCycleDirection {
    Previous,
    Next,
}

#[derive(Debug, Clone)]
struct SettingsOverlayDecision {
    setting_id: String,
    value: String,
}

fn next_setting_value(
    setting_id: &str,
    current: &str,
    direction: SettingsCycleDirection,
) -> Option<&'static str> {
    const THEME_VALUES: [&str; 3] = [UI_THEME_LIGHT, UI_THEME_DARK, UI_THEME_SYSTEM];
    const DENSITY_VALUES: [&str; 3] = [
        UI_DENSITY_COMPACT,
        UI_DENSITY_COMFORTABLE,
        UI_DENSITY_SPACIOUS,
    ];
    const MOTION_VALUES: [&str; 3] = [UI_MOTION_FULL, UI_MOTION_REDUCED, UI_MOTION_NONE];

    let values: &[&str] = match setting_id {
        UI_THEME_SETTING_ID => &THEME_VALUES,
        UI_DENSITY_SETTING_ID => &DENSITY_VALUES,
        UI_MOTION_SETTING_ID => &MOTION_VALUES,
        _ => return None,
    };
    let idx = values
        .iter()
        .position(|value| *value == current)
        .unwrap_or(0);
    let next_idx = match direction {
        SettingsCycleDirection::Next => (idx + 1) % values.len(),
        SettingsCycleDirection::Previous => {
            if idx == 0 {
                values.len().saturating_sub(1)
            } else {
                idx - 1
            }
        }
    };
    values.get(next_idx).copied()
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
struct WedgeInspectorShellOverlay {
    inspector: WedgeInspectorOverlay,
}

#[derive(Debug, Clone)]
struct StatusBarItemDetailOverlay {
    lines: Vec<String>,
}

#[derive(Debug, Clone)]
struct EntryFlowSheetOverlay {
    outcome: EntryFlowOutcome,
    command_id: String,
    origin: DispatchOrigin,
    argument_provenance_map: Vec<ArgumentProvenanceEntry>,
    admission_review: Option<AdmissionReviewPacket>,
    degraded_token: Option<DegradedStateToken>,
    note: Option<String>,
    clone_form: Option<CloneFlowForm>,
    import_form: Option<ImportFlowForm>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CloneFlowField {
    RemoteUrl,
    DestinationPath,
}

#[derive(Debug, Clone)]
struct CloneFlowForm {
    remote_url: String,
    destination_path: String,
    focused_field: CloneFlowField,
    status_line: Option<String>,
    running: bool,
}

impl CloneFlowForm {
    fn new() -> Self {
        Self {
            remote_url: String::new(),
            destination_path: String::new(),
            focused_field: CloneFlowField::RemoteUrl,
            status_line: None,
            running: false,
        }
    }

    fn submit_enabled(&self) -> bool {
        !self.running
            && !self.remote_url.trim().is_empty()
            && !self.destination_path.trim().is_empty()
    }

    fn request(&self) -> Result<CloneRequest, CloneError> {
        let packet = self.admission_review_packet().ok_or_else(|| {
            CloneError::new(
                CloneErrorClass::InvalidInput,
                "remote URL and destination path are required",
            )
        })?;
        if let Some(collision) = packet.collision_review.as_ref() {
            if collision.requires_explicit_choice {
                let actions = collision
                    .safe_actions
                    .iter()
                    .map(|action| action.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(CloneError::new(
                    CloneErrorClass::DestinationExists,
                    format!(
                        "{}; choose one of: {actions}. Typed inputs preserved; diagnostics redacted.",
                        collision.summary
                    ),
                ));
            }
        }
        let request = CloneRequest::new(
            self.remote_url.trim().to_string(),
            expand_tilde(self.destination_path.trim()),
        );
        request.validate()?;
        Ok(request)
    }

    fn admission_review_packet(&self) -> Option<AdmissionReviewPacket> {
        if self.remote_url.trim().is_empty() || self.destination_path.trim().is_empty() {
            return None;
        }
        Some(clone_form_admission_packet(
            self.remote_url.trim(),
            expand_tilde(self.destination_path.trim())
                .display()
                .to_string(),
        ))
    }

    fn argument_provenance_map(&self) -> Vec<ArgumentProvenanceEntry> {
        vec![
            ArgumentProvenanceEntry {
                argument_name: "remote_repository_ref".to_string(),
                provenance: "user_typed_in_clone_sheet".to_string(),
                resolved_value_ref: Some(format!("git-url:{}", self.remote_url.trim())),
            },
            ArgumentProvenanceEntry {
                argument_name: "destination_root_ref".to_string(),
                provenance: "user_typed_in_clone_sheet".to_string(),
                resolved_value_ref: Some(format!("path:{}", self.destination_path.trim())),
            },
            ArgumentProvenanceEntry {
                argument_name: "open_after_clone".to_string(),
                provenance: "default_from_descriptor".to_string(),
                resolved_value_ref: Some("value:bool:true".to_string()),
            },
        ]
    }

    fn push_text(&mut self, text: &str) -> bool {
        let mut changed = false;
        for ch in text.chars() {
            if ch.is_control() {
                continue;
            }
            match self.focused_field {
                CloneFlowField::RemoteUrl => self.remote_url.push(ch),
                CloneFlowField::DestinationPath => self.destination_path.push(ch),
            }
            changed = true;
        }
        if changed {
            self.status_line = None;
        }
        changed
    }

    fn pop_char(&mut self) -> bool {
        let changed = match self.focused_field {
            CloneFlowField::RemoteUrl => self.remote_url.pop().is_some(),
            CloneFlowField::DestinationPath => self.destination_path.pop().is_some(),
        };
        if changed {
            self.status_line = None;
        }
        changed
    }

    fn toggle_field(&mut self) {
        self.focused_field = match self.focused_field {
            CloneFlowField::RemoteUrl => CloneFlowField::DestinationPath,
            CloneFlowField::DestinationPath => CloneFlowField::RemoteUrl,
        };
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ImportFlowField {
    SourcePath,
    DestinationWorkspaceTarget,
}

#[derive(Debug, Clone)]
struct ImportFlowForm {
    source_path: String,
    destination_workspace_target: String,
    focused_field: ImportFlowField,
    review_record: Option<ImportReviewRecord>,
    diff_review_packet: Option<ImportDiffReviewPacket>,
    status_line: Option<String>,
    applied: bool,
}

impl ImportFlowForm {
    fn new() -> Self {
        Self {
            source_path: String::new(),
            destination_workspace_target: "profile:default".to_string(),
            focused_field: ImportFlowField::SourcePath,
            review_record: None,
            diff_review_packet: None,
            status_line: None,
            applied: false,
        }
    }

    fn review_enabled(&self) -> bool {
        !self.source_path.trim().is_empty() && !self.destination_workspace_target.trim().is_empty()
    }

    fn apply_enabled(&self) -> bool {
        !self.applied
            && self.review_record.as_ref().is_some_and(|record| {
                record.decision_class == ImportReviewDecisionClass::ApplyAfterPreview
            })
    }

    fn review_source(&mut self) {
        if !self.review_enabled() {
            self.status_line = Some("source path and destination target are required".to_string());
            return;
        }
        let source_path = expand_tilde(self.source_path.trim());
        let review = CompetitorConfigClassifier::new()
            .build_review(source_path, self.destination_workspace_target.trim());
        self.status_line = Some(review.status_line.clone());
        self.diff_review_packet = Some(materialize_import_diff_review_packet(&review));
        self.review_record = Some(review);
        self.applied = false;
    }

    fn admission_review_packet(&self) -> Option<AdmissionReviewPacket> {
        if self.source_path.trim().is_empty() || self.destination_workspace_target.trim().is_empty()
        {
            return None;
        }
        Some(import_form_admission_packet(
            self.source_path.trim(),
            self.destination_workspace_target.trim(),
        ))
    }

    fn argument_provenance_map(&self) -> Vec<ArgumentProvenanceEntry> {
        let review_ref = self
            .review_record
            .as_ref()
            .map(|record| record.import_review_id.clone());
        let classification = self
            .review_record
            .as_ref()
            .map(|record| record.classification.as_str().to_string());
        vec![
            ArgumentProvenanceEntry {
                argument_name: "import_source_ref".to_string(),
                provenance: "user_typed_in_import_sheet".to_string(),
                resolved_value_ref: Some(format!("path:{}", self.source_path.trim())),
            },
            ArgumentProvenanceEntry {
                argument_name: "destination_workspace_target_ref".to_string(),
                provenance: "user_typed_in_import_sheet".to_string(),
                resolved_value_ref: Some(format!(
                    "workspace-target:{}",
                    self.destination_workspace_target.trim()
                )),
            },
            ArgumentProvenanceEntry {
                argument_name: "import_review_ref".to_string(),
                provenance: "derived_from_import_sheet_review".to_string(),
                resolved_value_ref: review_ref.map(|value| format!("import-review:{value}")),
            },
            ArgumentProvenanceEntry {
                argument_name: "classification".to_string(),
                provenance: "derived_from_import_classifier".to_string(),
                resolved_value_ref: classification.map(|value| format!("enum:{value}")),
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
        ]
    }

    fn push_text(&mut self, text: &str) -> bool {
        let mut changed = false;
        for ch in text.chars() {
            if ch.is_control() {
                continue;
            }
            match self.focused_field {
                ImportFlowField::SourcePath => self.source_path.push(ch),
                ImportFlowField::DestinationWorkspaceTarget => {
                    self.destination_workspace_target.push(ch)
                }
            }
            changed = true;
        }
        if changed {
            self.clear_review_after_edit();
        }
        changed
    }

    fn pop_char(&mut self) -> bool {
        let changed = match self.focused_field {
            ImportFlowField::SourcePath => self.source_path.pop().is_some(),
            ImportFlowField::DestinationWorkspaceTarget => {
                self.destination_workspace_target.pop().is_some()
            }
        };
        if changed {
            self.clear_review_after_edit();
        }
        changed
    }

    fn toggle_field(&mut self) {
        self.focused_field = match self.focused_field {
            ImportFlowField::SourcePath => ImportFlowField::DestinationWorkspaceTarget,
            ImportFlowField::DestinationWorkspaceTarget => ImportFlowField::SourcePath,
        };
    }

    fn clear_review_after_edit(&mut self) {
        self.review_record = None;
        self.diff_review_packet = None;
        self.status_line = None;
        self.applied = false;
    }
}

#[derive(Debug, Clone)]
struct SaveReviewOverlay {
    group: PaneId,
    tab: EditorTabId,
    outcome: aureline_vfs::SaveOutcome,
    record: SaveReviewSheetRecord,
    reviewed_external_state: bool,
    selection: usize,
    status_line: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FindReplaceOverlayField {
    Query,
    Replacement,
}

#[derive(Clone)]
struct FindReplaceOverlay {
    authority: Rc<RefCell<BufferAuthority>>,
    field: FindReplaceOverlayField,
    status_line: Option<String>,
}

impl std::fmt::Debug for FindReplaceOverlay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FindReplaceOverlay")
            .field("field", &self.field)
            .field("status_line", &self.status_line)
            .finish_non_exhaustive()
    }
}

impl FindReplaceOverlay {
    fn new(authority: Rc<RefCell<BufferAuthority>>, mode: FindReplaceMode) -> Self {
        {
            let mut auth = authority.borrow_mut();
            auth.find_replace.set_mode(mode);
        }
        Self {
            authority,
            field: FindReplaceOverlayField::Query,
            status_line: None,
        }
    }
}

#[derive(Debug, Clone)]
enum WorkspaceSwitcherDecision {
    Activate {
        recent_work_id: String,
    },
    SetPinned {
        recent_work_id: String,
        pinned: bool,
    },
    Remove {
        recent_work_id: String,
    },
    PerformRecoveryAction {
        recent_work_id: String,
        action: SafeRecoveryAction,
    },
}

#[derive(Debug, Clone)]
enum WorkspaceSwitcherOverlayMode {
    List,
    ConfirmSwitch {
        recent_work_id: String,
    },
    ConfirmRemove {
        recent_work_id: String,
    },
    RecoveryActions {
        recent_work_id: String,
        selection: usize,
    },
}

#[derive(Debug, Clone)]
struct WorkspaceSwitcherOverlay {
    state: WorkspaceSwitcherState,
    snapshot: RecentWorkRegistry,
    mode: WorkspaceSwitcherOverlayMode,
    current_trust_state: String,
}

impl WorkspaceSwitcherOverlay {
    fn new(mut snapshot: RecentWorkRegistry, current_trust_state: String) -> Self {
        for entry in &mut snapshot.entries {
            normalize_recent_work_entry_recovery_actions(entry);
        }
        Self {
            state: WorkspaceSwitcherState::new(),
            snapshot,
            mode: WorkspaceSwitcherOverlayMode::List,
            current_trust_state,
        }
    }

    fn rows(&self) -> Vec<WorkspaceSwitcherRow> {
        build_switcher_rows(&self.snapshot, self.state.query())
    }

    fn selected_row(&self) -> Option<WorkspaceSwitcherRow> {
        let rows = self.rows();
        let idx = self.state.selection().min(rows.len().saturating_sub(1));
        rows.get(idx).cloned()
    }

    fn toggle_pinned(&mut self, recent_work_id: &str) -> Option<bool> {
        let row = self
            .snapshot
            .entries
            .iter_mut()
            .find(|row| row.recent_work_id == recent_work_id)?;
        if !row
            .safe_recovery_actions
            .iter()
            .any(|action| matches!(action, SafeRecoveryAction::Pin | SafeRecoveryAction::Unpin))
        {
            return None;
        }
        row.pinned = !row.pinned;
        if row.pinned {
            row.safe_recovery_actions
                .retain(|action| *action != SafeRecoveryAction::Pin);
            if !row
                .safe_recovery_actions
                .contains(&SafeRecoveryAction::Unpin)
            {
                row.safe_recovery_actions.push(SafeRecoveryAction::Unpin);
            }
        } else {
            row.safe_recovery_actions
                .retain(|action| *action != SafeRecoveryAction::Unpin);
            if !row.safe_recovery_actions.contains(&SafeRecoveryAction::Pin) {
                row.safe_recovery_actions.push(SafeRecoveryAction::Pin);
            }
        }
        Some(row.pinned)
    }

    fn remove(&mut self, recent_work_id: &str) -> bool {
        self.snapshot.remove(recent_work_id)
    }

    fn requires_switch_preview(&self, entry: &RecentWorkEntryRecord) -> bool {
        entry.trust_state.as_str() != self.current_trust_state
            || entry.target_state != RecentWorkTargetState::Reachable
            || matches!(
                entry.target_kind,
                TargetKind::SshWorkspace
                    | TargetKind::ContainerWorkspace
                    | TargetKind::DevcontainerWorkspace
                    | TargetKind::ManagedCloudWorkspace
                    | TargetKind::RemoteRepository
            )
    }
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

#[derive(Debug, Clone)]
struct EntryFlowOverlayDecision {
    command_id: String,
    origin: DispatchOrigin,
    argument_provenance_map: Vec<ArgumentProvenanceEntry>,
}

#[derive(Debug)]
struct OverlayKeyOutcome {
    handled: bool,
    command_decision: Option<CommandOverlayDecision>,
    entry_flow_decision: Option<EntryFlowOverlayDecision>,
    workspace_switcher_decision: Option<WorkspaceSwitcherDecision>,
    settings_decision: Option<SettingsOverlayDecision>,
}

#[derive(Debug, Clone)]
enum ShellOverlayKind {
    InspectorSheet,
    FindReplace(FindReplaceOverlay),
    SplitChoice {
        violation: SplitViolation,
        selection: usize,
    },
    StagedPeek,
    SaveReview(SaveReviewOverlay),
    EntryFlowSheet(EntryFlowSheetOverlay),
    CommandDiagnostics(CommandDiagnosticsOverlay),
    InvocationPreview(CommandInvocationPreviewOverlay),
    CommandTrace(CommandTraceOverlay),
    WedgeInspector(WedgeInspectorShellOverlay),
    StatusBarItemDetail(StatusBarItemDetailOverlay),
    WorkspaceSwitcher(WorkspaceSwitcherOverlay),
    Settings(SettingsOverlay),
}

#[derive(Debug, Clone)]
struct ShellOverlayState {
    kind: ShellOverlayKind,
    focus_return_zone: ShellZoneId,
    focus_return_group: PaneId,
    opened_at: Instant,
    closed: bool,
}

impl ShellOverlayState {
    fn inspector_sheet(focus_return_zone: ShellZoneId, focus_return_group: PaneId) -> Self {
        Self {
            kind: ShellOverlayKind::InspectorSheet,
            focus_return_zone,
            focus_return_group,
            opened_at: Instant::now(),
            closed: false,
        }
    }

    fn find_replace(
        focus_return_zone: ShellZoneId,
        focus_return_group: PaneId,
        authority: Rc<RefCell<BufferAuthority>>,
        mode: FindReplaceMode,
    ) -> Self {
        Self {
            kind: ShellOverlayKind::FindReplace(FindReplaceOverlay::new(authority, mode)),
            focus_return_zone,
            focus_return_group,
            opened_at: Instant::now(),
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
            opened_at: Instant::now(),
            closed: false,
        }
    }

    fn save_review(
        focus_return_zone: ShellZoneId,
        focus_return_group: PaneId,
        group: PaneId,
        tab: EditorTabId,
        record: SaveReviewSheetRecord,
        outcome: aureline_vfs::SaveOutcome,
    ) -> Self {
        Self {
            kind: ShellOverlayKind::SaveReview(SaveReviewOverlay {
                group,
                tab,
                outcome,
                record,
                reviewed_external_state: false,
                selection: 0,
                status_line: None,
            }),
            focus_return_zone,
            focus_return_group,
            opened_at: Instant::now(),
            closed: false,
        }
    }

    fn entry_flow_sheet(
        focus_return_zone: ShellZoneId,
        focus_return_group: PaneId,
        outcome: EntryFlowOutcome,
        command_id: String,
        origin: DispatchOrigin,
        argument_provenance_map: Vec<ArgumentProvenanceEntry>,
        degraded_token: Option<DegradedStateToken>,
        note: Option<String>,
    ) -> Self {
        let admission_review =
            entry_flow_admission_packet(origin, &outcome, &argument_provenance_map);
        let clone_form = match &outcome {
            EntryFlowOutcome::Resolved(resolved)
                if resolved.sheet_class == OpenFlowSheetClass::CloneRemoteTarget =>
            {
                Some(CloneFlowForm::new())
            }
            _ => None,
        };
        let import_form = match &outcome {
            EntryFlowOutcome::Resolved(resolved)
                if resolved.sheet_class == OpenFlowSheetClass::ImportArtifact
                    && command_id == "cmd:workspace.import_profile" =>
            {
                Some(ImportFlowForm::new())
            }
            _ => None,
        };
        Self {
            kind: ShellOverlayKind::EntryFlowSheet(EntryFlowSheetOverlay {
                outcome,
                command_id,
                origin,
                argument_provenance_map,
                admission_review,
                degraded_token,
                note,
                clone_form,
                import_form,
            }),
            focus_return_zone,
            focus_return_group,
            opened_at: Instant::now(),
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
            opened_at: Instant::now(),
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
            opened_at: Instant::now(),
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
            opened_at: Instant::now(),
            closed: false,
        }
    }

    fn wedge_inspector(
        focus_return_zone: ShellZoneId,
        focus_return_group: PaneId,
        inspector: WedgeInspectorOverlay,
    ) -> Self {
        Self {
            kind: ShellOverlayKind::WedgeInspector(WedgeInspectorShellOverlay { inspector }),
            focus_return_zone,
            focus_return_group,
            opened_at: Instant::now(),
            closed: false,
        }
    }

    fn status_bar_item_detail(
        focus_return_zone: ShellZoneId,
        focus_return_group: PaneId,
        lines: Vec<String>,
    ) -> Self {
        Self {
            kind: ShellOverlayKind::StatusBarItemDetail(StatusBarItemDetailOverlay { lines }),
            focus_return_zone,
            focus_return_group,
            opened_at: Instant::now(),
            closed: false,
        }
    }

    fn workspace_switcher(
        focus_return_zone: ShellZoneId,
        focus_return_group: PaneId,
        snapshot: &RecentWorkRegistry,
        current_trust_state: &str,
    ) -> Self {
        Self {
            kind: ShellOverlayKind::WorkspaceSwitcher(WorkspaceSwitcherOverlay::new(
                snapshot.clone(),
                current_trust_state.to_string(),
            )),
            focus_return_zone,
            focus_return_group,
            opened_at: Instant::now(),
            closed: false,
        }
    }

    fn settings(
        focus_return_zone: ShellZoneId,
        focus_return_group: PaneId,
        appearance: &AppearanceRuntimeState,
    ) -> Self {
        Self {
            kind: ShellOverlayKind::Settings(SettingsOverlay::new(appearance)),
            focus_return_zone,
            focus_return_group,
            opened_at: Instant::now(),
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
        text: Option<&str>,
        modifiers: HeldModifiers,
        frame: &mut DesktopFrame,
        editor_runtime: &mut EditorWorkspaceRuntimeState,
        keybinding_runtime: &mut KeybindingRuntimeState,
    ) -> OverlayKeyOutcome {
        let mut command_decision = None;
        let mut entry_flow_decision = None;
        let mut workspace_switcher_decision = None;
        let mut settings_decision = None;
        let handled = match (&mut self.kind, code) {
            (ShellOverlayKind::WorkspaceSwitcher(switcher), KeyCode::Escape) => {
                if matches!(switcher.mode, WorkspaceSwitcherOverlayMode::List) {
                    self.close(frame);
                } else {
                    switcher.mode = WorkspaceSwitcherOverlayMode::List;
                }
                true
            }
            (ShellOverlayKind::Settings(_settings), KeyCode::Escape) => {
                self.close(frame);
                true
            }
            (ShellOverlayKind::Settings(settings), KeyCode::ArrowDown) => {
                settings.select_next();
                true
            }
            (ShellOverlayKind::Settings(settings), KeyCode::ArrowUp) => {
                settings.select_prev();
                true
            }
            (
                ShellOverlayKind::Settings(settings),
                KeyCode::ArrowRight | KeyCode::Enter | KeyCode::Space,
            ) => {
                settings_decision = settings.next_value_decision(SettingsCycleDirection::Next);
                true
            }
            (ShellOverlayKind::Settings(settings), KeyCode::ArrowLeft) => {
                settings_decision = settings.next_value_decision(SettingsCycleDirection::Previous);
                true
            }
            (ShellOverlayKind::FindReplace(find), KeyCode::Escape) => {
                {
                    let mut auth = find.authority.borrow_mut();
                    auth.find_replace.close();
                }
                self.close(frame);
                true
            }
            (ShellOverlayKind::EntryFlowSheet(sheet), KeyCode::Escape) => {
                if sheet.clone_form.as_ref().is_some_and(|form| form.running) {
                    if let Some(form) = sheet.clone_form.as_mut() {
                        form.status_line = Some(
                            "clone is running; this build does not support cancellation"
                                .to_string(),
                        );
                    }
                } else if sheet.import_form.as_ref().is_some_and(|form| form.applied) {
                    self.close(frame);
                } else {
                    self.close(frame);
                }
                true
            }
            (ShellOverlayKind::EntryFlowSheet(sheet), KeyCode::Tab) => {
                if let Some(form) = sheet.clone_form.as_mut() {
                    form.toggle_field();
                    true
                } else if let Some(form) = sheet.import_form.as_mut() {
                    form.toggle_field();
                    true
                } else {
                    false
                }
            }
            (ShellOverlayKind::EntryFlowSheet(sheet), KeyCode::Backspace | KeyCode::Delete) => {
                if let Some(form) = sheet.clone_form.as_mut() {
                    form.pop_char()
                } else if let Some(form) = sheet.import_form.as_mut() {
                    form.pop_char()
                } else {
                    false
                }
            }
            (ShellOverlayKind::EntryFlowSheet(sheet), KeyCode::Enter) => {
                if let Some(form) = sheet.clone_form.as_mut() {
                    if form.running {
                        form.status_line = Some("clone is already running".to_string());
                        return OverlayKeyOutcome {
                            handled: true,
                            command_decision,
                            entry_flow_decision,
                            workspace_switcher_decision,
                            settings_decision,
                        };
                    }
                    match form.request() {
                        Ok(_) => {
                            form.running = true;
                            form.status_line = Some("clone queued".to_string());
                            entry_flow_decision = Some(EntryFlowOverlayDecision {
                                command_id: sheet.command_id.clone(),
                                origin: sheet.origin,
                                argument_provenance_map: form.argument_provenance_map(),
                            });
                        }
                        Err(err) => {
                            form.status_line =
                                Some(format!("{}: {}", err.class.as_str(), err.message));
                        }
                    }
                } else if let Some(form) = sheet.import_form.as_mut() {
                    if form.review_record.is_none() {
                        form.review_source();
                        return OverlayKeyOutcome {
                            handled: true,
                            command_decision,
                            entry_flow_decision,
                            workspace_switcher_decision,
                            settings_decision,
                        };
                    }
                    if form.applied {
                        form.status_line = Some("import apply already recorded".to_string());
                        return OverlayKeyOutcome {
                            handled: true,
                            command_decision,
                            entry_flow_decision,
                            workspace_switcher_decision,
                            settings_decision,
                        };
                    }
                    if form.apply_enabled() {
                        form.status_line = Some("import apply queued".to_string());
                        entry_flow_decision = Some(EntryFlowOverlayDecision {
                            command_id: sheet.command_id.clone(),
                            origin: sheet.origin,
                            argument_provenance_map: form.argument_provenance_map(),
                        });
                    } else if let Some(review) = form.review_record.as_ref() {
                        form.status_line = Some(format!(
                            "import {}: {}",
                            review.decision_class.as_str(),
                            review.status_line
                        ));
                    }
                } else {
                    if matches!(sheet.outcome, EntryFlowOutcome::Resolved(_)) {
                        entry_flow_decision = Some(EntryFlowOverlayDecision {
                            command_id: sheet.command_id.clone(),
                            origin: sheet.origin,
                            argument_provenance_map: sheet.argument_provenance_map.clone(),
                        });
                    }
                    self.close(frame);
                }
                true
            }
            (ShellOverlayKind::SaveReview(review), KeyCode::Escape) => {
                review.record.selected_choice =
                    Some(SaveReviewChoiceKey::Cancel.as_str().to_string());
                review.record.selected_at = Some(now_rfc3339());
                write_save_review_sheet_log(&review.record);
                self.close(frame);
                true
            }
            (ShellOverlayKind::SaveReview(review), KeyCode::Enter) => {
                let Some(choice) = review.record.offered_choices.get(review.selection).cloned()
                else {
                    self.close(frame);
                    return OverlayKeyOutcome {
                        handled: true,
                        command_decision,
                        entry_flow_decision,
                        workspace_switcher_decision,
                        settings_decision,
                    };
                };

                if !choice.enabled {
                    review.status_line = Some(format!(
                        "choice disabled: {} ({})",
                        choice.choice, choice.forbidden_reason
                    ));
                    return OverlayKeyOutcome {
                        handled: true,
                        command_decision,
                        entry_flow_decision,
                        workspace_switcher_decision,
                        settings_decision,
                    };
                }

                match choice.choice.as_str() {
                    "compare" => {
                        review.record.selected_choice =
                            Some(SaveReviewChoiceKey::Compare.as_str().to_string());
                        review.record.selected_at = Some(now_rfc3339());
                        write_save_review_sheet_log(&review.record);
                        review.reviewed_external_state = true;

                        let Some(tab_session) =
                            editor_runtime.tab_session_mut(review.group, review.tab)
                        else {
                            review.status_line = Some("tab missing for save review".to_string());
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                entry_flow_decision,
                                workspace_switcher_decision,
                                settings_decision,
                            };
                        };
                        tab_session.ensure_fresh_snapshot();
                        let local_bytes = tab_session.snapshot.as_bytes().to_vec();
                        let (token, source_fidelity) = {
                            let auth = tab_session.authority.borrow();
                            let token = match auth.save_target_token.clone() {
                                Some(token) => token,
                                None => {
                                    review.status_line =
                                        Some("missing save target token for review".to_string());
                                    return OverlayKeyOutcome {
                                        handled: true,
                                        command_decision,
                                        entry_flow_decision,
                                        workspace_switcher_decision,
                                        settings_decision,
                                    };
                                }
                            };
                            let source_fidelity = match auth.source_fidelity.clone() {
                                Some(record) => record,
                                None => {
                                    review.status_line = Some(
                                        "missing source-fidelity record for review".to_string(),
                                    );
                                    return OverlayKeyOutcome {
                                        handled: true,
                                        command_decision,
                                        entry_flow_decision,
                                        workspace_switcher_decision,
                                        settings_decision,
                                    };
                                }
                            };
                            (token, source_fidelity)
                        };

                        let root = LocalFilesystemRoot::host_root("ws-shell_proto", "root-local");
                        review.record = materialize_save_review_sheet_record(
                            &root,
                            &token,
                            &source_fidelity,
                            review.record.packet_id.clone(),
                            review.outcome,
                            mono_timestamp_now(),
                            &local_bytes,
                            review.reviewed_external_state,
                        );
                        write_save_review_sheet_log(&review.record);
                        review.selection =
                            1.min(review.record.offered_choices.len().saturating_sub(1));
                        review.status_line =
                            Some("compare admitted; overwrite now enabled".to_string());
                        true
                    }
                    "overwrite" => {
                        review.record.selected_choice =
                            Some(SaveReviewChoiceKey::Overwrite.as_str().to_string());
                        review.record.selected_at = Some(now_rfc3339());
                        write_save_review_sheet_log(&review.record);

                        let (presentation_uri, token, source_fidelity, local_bytes) = {
                            let Some(tab_session) =
                                editor_runtime.tab_session_mut(review.group, review.tab)
                            else {
                                review.status_line = Some("tab missing for overwrite".to_string());
                                return OverlayKeyOutcome {
                                    handled: true,
                                    command_decision,
                                    entry_flow_decision,
                                    workspace_switcher_decision,
                                    settings_decision,
                                };
                            };
                            tab_session.ensure_fresh_snapshot();
                            let local_bytes = tab_session.snapshot.as_bytes().to_vec();

                            let auth = tab_session.authority.borrow();
                            if auth.read_only != ReadOnlyState::Writable {
                                review.status_line = Some("tab is read-only".to_string());
                                return OverlayKeyOutcome {
                                    handled: true,
                                    command_decision,
                                    entry_flow_decision,
                                    workspace_switcher_decision,
                                    settings_decision,
                                };
                            }
                            let Some(path) = auth.file_path.clone() else {
                                review.status_line = Some("tab is not file-backed".to_string());
                                return OverlayKeyOutcome {
                                    handled: true,
                                    command_decision,
                                    entry_flow_decision,
                                    workspace_switcher_decision,
                                    settings_decision,
                                };
                            };
                            let presentation_uri = match VfsUri::file_url_for_path(&path) {
                                Some(uri) => uri,
                                None => {
                                    review.status_line =
                                        Some(format!("vfs uri build failed for {path:?}"));
                                    return OverlayKeyOutcome {
                                        handled: true,
                                        command_decision,
                                        entry_flow_decision,
                                        workspace_switcher_decision,
                                        settings_decision,
                                    };
                                }
                            };
                            let token = match auth.save_target_token.clone() {
                                Some(token) => token,
                                None => {
                                    review.status_line =
                                        Some("missing save target token for overwrite".to_string());
                                    return OverlayKeyOutcome {
                                        handled: true,
                                        command_decision,
                                        entry_flow_decision,
                                        workspace_switcher_decision,
                                        settings_decision,
                                    };
                                }
                            };
                            let source_fidelity = match auth.source_fidelity.clone() {
                                Some(record) => record,
                                None => {
                                    review.status_line = Some(
                                        "missing source-fidelity record for overwrite".to_string(),
                                    );
                                    return OverlayKeyOutcome {
                                        handled: true,
                                        command_decision,
                                        entry_flow_decision,
                                        workspace_switcher_decision,
                                        settings_decision,
                                    };
                                }
                            };
                            (presentation_uri, token, source_fidelity, local_bytes)
                        };

                        let mut root =
                            LocalFilesystemRoot::host_root("ws-shell_proto", "root-local");
                        let mut counters = HookCounters::default();
                        let refreshed_token = match open_save_target(
                            &root,
                            &presentation_uri,
                            mono_timestamp_now(),
                            &mut counters,
                        ) {
                            Ok(token) => token,
                            Err(err) => {
                                review.status_line =
                                    Some(format!("refresh save target failed — {err}"));
                                return OverlayKeyOutcome {
                                    handled: true,
                                    command_decision,
                                    entry_flow_decision,
                                    workspace_switcher_decision,
                                    settings_decision,
                                };
                            }
                        };

                        let request = StagedSaveRequest {
                            token: refreshed_token.clone(),
                            new_content: local_bytes.clone(),
                            source_fidelity: source_fidelity.clone(),
                            save_participant_group_id: None,
                            checkpoint_ref: None,
                            committed_at: mono_timestamp_now(),
                        };
                        let mut participants: Vec<
                            Box<dyn aureline_workspace::save::SaveParticipant>,
                        > = Vec::new();
                        let result = editor_runtime.save_coordinator.save(
                            &mut root,
                            request,
                            participants.as_mut_slice(),
                        );

                        if result.committed() {
                            if let Some(tab_session) =
                                editor_runtime.tab_session_mut(review.group, review.tab)
                            {
                                let mut auth = tab_session.authority.borrow_mut();
                                auth.mark_saved();
                                auth.save_target_token = Some(result.next_token.clone());
                            }
                            review.status_line = Some("overwrite committed".to_string());
                            self.close(frame);
                            true
                        } else {
                            review.reviewed_external_state = false;
                            review.outcome = result.manifest.outcome;
                            review.record = materialize_save_review_sheet_record(
                                &root,
                                &token,
                                &source_fidelity,
                                result.packet_id.clone(),
                                review.outcome,
                                mono_timestamp_now(),
                                &local_bytes,
                                review.reviewed_external_state,
                            );
                            write_save_review_sheet_log(&review.record);
                            review.selection = 0;
                            review.status_line =
                                Some(format!("overwrite refused ({})", review.outcome.as_str()));
                            true
                        }
                    }
                    "merge" => {
                        review.status_line = Some("merge is not available yet".to_string());
                        true
                    }
                    "reload" => {
                        review.status_line = Some("reload is not available yet".to_string());
                        true
                    }
                    "retry" => {
                        review.record.selected_choice =
                            Some(SaveReviewChoiceKey::Retry.as_str().to_string());
                        review.record.selected_at = Some(now_rfc3339());
                        write_save_review_sheet_log(&review.record);

                        let Some(tab_session) =
                            editor_runtime.tab_session_mut(review.group, review.tab)
                        else {
                            review.status_line = Some("tab missing for retry".to_string());
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                entry_flow_decision,
                                workspace_switcher_decision,
                                settings_decision,
                            };
                        };
                        tab_session.ensure_fresh_snapshot();
                        let local_bytes = tab_session.snapshot.as_bytes().to_vec();
                        let (token, source_fidelity) = {
                            let auth = tab_session.authority.borrow();
                            let token = match auth.save_target_token.clone() {
                                Some(token) => token,
                                None => {
                                    review.status_line =
                                        Some("missing save target token for retry".to_string());
                                    return OverlayKeyOutcome {
                                        handled: true,
                                        command_decision,
                                        entry_flow_decision,
                                        workspace_switcher_decision,
                                        settings_decision,
                                    };
                                }
                            };
                            let source_fidelity = match auth.source_fidelity.clone() {
                                Some(record) => record,
                                None => {
                                    review.status_line = Some(
                                        "missing source-fidelity record for retry".to_string(),
                                    );
                                    return OverlayKeyOutcome {
                                        handled: true,
                                        command_decision,
                                        entry_flow_decision,
                                        workspace_switcher_decision,
                                        settings_decision,
                                    };
                                }
                            };
                            (token, source_fidelity)
                        };

                        let root = LocalFilesystemRoot::host_root("ws-shell_proto", "root-local");
                        review.record = materialize_save_review_sheet_record(
                            &root,
                            &token,
                            &source_fidelity,
                            review.record.packet_id.clone(),
                            review.outcome,
                            mono_timestamp_now(),
                            &local_bytes,
                            review.reviewed_external_state,
                        );
                        write_save_review_sheet_log(&review.record);
                        review.selection = review
                            .selection
                            .min(review.record.offered_choices.len().saturating_sub(1));
                        review.status_line = Some("refreshed external state".to_string());
                        true
                    }
                    "save_as" => {
                        review.status_line = Some("save-as is not available yet".to_string());
                        true
                    }
                    "cancel" => {
                        review.record.selected_choice =
                            Some(SaveReviewChoiceKey::Cancel.as_str().to_string());
                        review.record.selected_at = Some(now_rfc3339());
                        write_save_review_sheet_log(&review.record);
                        self.close(frame);
                        true
                    }
                    _ => {
                        review.status_line = Some(format!("unknown choice: {}", choice.choice));
                        true
                    }
                }
            }
            (ShellOverlayKind::SaveReview(review), KeyCode::ArrowDown) => {
                let count = review.record.offered_choices.len();
                if count == 0 {
                    review.selection = 0;
                } else {
                    review.selection = (review.selection + 1) % count;
                }
                true
            }
            (ShellOverlayKind::SaveReview(review), KeyCode::ArrowUp) => {
                let count = review.record.offered_choices.len();
                if count == 0 {
                    review.selection = 0;
                } else {
                    review.selection = (review.selection + count - 1) % count;
                }
                true
            }
            (ShellOverlayKind::WedgeInspector(sheet), KeyCode::ArrowDown) => {
                sheet.inspector.select_next();
                true
            }
            (ShellOverlayKind::WedgeInspector(sheet), KeyCode::ArrowUp) => {
                sheet.inspector.select_prev();
                true
            }
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
            (ShellOverlayKind::FindReplace(find), KeyCode::Tab) => {
                if let Some(tab) = frame.active_tab_id(frame.focused_editor_group()) {
                    if let Some(tab_session) =
                        editor_runtime.tab_session_mut(frame.focused_editor_group(), tab)
                    {
                        tab_session.ensure_fresh_snapshot();
                        let caret = tab_session.viewport.caret();
                        let snapshot = tab_session.snapshot.clone();
                        let mut auth = find.authority.borrow_mut();
                        match auth.find_replace.mode() {
                            FindReplaceMode::Replace => {
                                if modifiers.shift
                                    && matches!(find.field, FindReplaceOverlayField::Replacement)
                                {
                                    find.field = FindReplaceOverlayField::Query;
                                } else if matches!(find.field, FindReplaceOverlayField::Query) {
                                    find.field = FindReplaceOverlayField::Replacement;
                                } else {
                                    find.field = FindReplaceOverlayField::Query;
                                }
                            }
                            FindReplaceMode::Find => {
                                auth.find_replace.set_mode(FindReplaceMode::Replace);
                                find.field = FindReplaceOverlayField::Replacement;
                            }
                            FindReplaceMode::Hidden => {
                                auth.find_replace.set_mode(FindReplaceMode::Find);
                                find.field = FindReplaceOverlayField::Query;
                            }
                        }
                        let _ = auth.find_replace.sync_for_view(&snapshot, caret);
                    }
                }
                true
            }
            (ShellOverlayKind::FindReplace(find), KeyCode::KeyC) if modifiers.alt => {
                if let Some(tab) = frame.active_tab_id(frame.focused_editor_group()) {
                    if let Some(tab_session) =
                        editor_runtime.tab_session_mut(frame.focused_editor_group(), tab)
                    {
                        tab_session.ensure_fresh_snapshot();
                        let caret = tab_session.viewport.caret();
                        let snapshot = tab_session.snapshot.clone();
                        let mut auth = find.authority.borrow_mut();
                        auth.find_replace.toggle_case_sensitive();
                        let _ = auth.find_replace.sync_for_view(&snapshot, caret);
                    }
                }
                true
            }
            (ShellOverlayKind::FindReplace(find), KeyCode::KeyW) if modifiers.alt => {
                if let Some(tab) = frame.active_tab_id(frame.focused_editor_group()) {
                    if let Some(tab_session) =
                        editor_runtime.tab_session_mut(frame.focused_editor_group(), tab)
                    {
                        tab_session.ensure_fresh_snapshot();
                        let caret = tab_session.viewport.caret();
                        let snapshot = tab_session.snapshot.clone();
                        let mut auth = find.authority.borrow_mut();
                        auth.find_replace.toggle_whole_word();
                        let _ = auth.find_replace.sync_for_view(&snapshot, caret);
                    }
                }
                true
            }
            (ShellOverlayKind::FindReplace(find), KeyCode::Backspace | KeyCode::Delete) => {
                if let Some(tab) = frame.active_tab_id(frame.focused_editor_group()) {
                    if let Some(tab_session) =
                        editor_runtime.tab_session_mut(frame.focused_editor_group(), tab)
                    {
                        tab_session.ensure_fresh_snapshot();
                        let caret = tab_session.viewport.caret();
                        let snapshot = tab_session.snapshot.clone();
                        let mut auth = find.authority.borrow_mut();
                        match find.field {
                            FindReplaceOverlayField::Query => {
                                let mut query = auth.find_replace.query().to_string();
                                let _ = query.pop();
                                auth.find_replace.set_query(query);
                            }
                            FindReplaceOverlayField::Replacement => {
                                let mut replacement = auth.find_replace.replacement().to_string();
                                let _ = replacement.pop();
                                auth.find_replace.set_replacement(replacement);
                            }
                        }
                        let _ = auth.find_replace.sync_for_view(&snapshot, caret);
                    }
                }
                true
            }
            (ShellOverlayKind::FindReplace(find), KeyCode::ArrowDown) => {
                if let Some(tab) = frame.active_tab_id(frame.focused_editor_group()) {
                    if let Some(tab_session) =
                        editor_runtime.tab_session_mut(frame.focused_editor_group(), tab)
                    {
                        tab_session.ensure_fresh_snapshot();
                        let caret = tab_session.viewport.caret();
                        let snapshot = tab_session.snapshot.clone();
                        let mut auth = find.authority.borrow_mut();
                        let _ = auth.find_replace.select_next(&snapshot, caret);
                        if let Some(range) = auth.find_replace.active_match_range() {
                            if let Some((line, grapheme)) =
                                snapshot.line_grapheme_for_byte_offset(range.start)
                            {
                                let point = aureline_editor::TextPoint { line, grapheme };
                                tab_session.viewport.set_caret(point);
                                tab_session.viewport.clear_selection();
                                tab_session
                                    .viewport
                                    .reveal_line(point.line, tab_session.max_scroll_line());
                                let _ = auth.find_replace.sync_for_view(&snapshot, point);
                            }
                        }
                    }
                }
                true
            }
            (ShellOverlayKind::FindReplace(find), KeyCode::ArrowUp) => {
                if let Some(tab) = frame.active_tab_id(frame.focused_editor_group()) {
                    if let Some(tab_session) =
                        editor_runtime.tab_session_mut(frame.focused_editor_group(), tab)
                    {
                        tab_session.ensure_fresh_snapshot();
                        let caret = tab_session.viewport.caret();
                        let snapshot = tab_session.snapshot.clone();
                        let mut auth = find.authority.borrow_mut();
                        let _ = auth.find_replace.select_prev(&snapshot, caret);
                        if let Some(range) = auth.find_replace.active_match_range() {
                            if let Some((line, grapheme)) =
                                snapshot.line_grapheme_for_byte_offset(range.start)
                            {
                                let point = aureline_editor::TextPoint { line, grapheme };
                                tab_session.viewport.set_caret(point);
                                tab_session.viewport.clear_selection();
                                tab_session
                                    .viewport
                                    .reveal_line(point.line, tab_session.max_scroll_line());
                                let _ = auth.find_replace.sync_for_view(&snapshot, point);
                            }
                        }
                    }
                }
                true
            }
            (ShellOverlayKind::FindReplace(find), KeyCode::Enter) => {
                if let Some(tab) = frame.active_tab_id(frame.focused_editor_group()) {
                    if let Some(tab_session) =
                        editor_runtime.tab_session_mut(frame.focused_editor_group(), tab)
                    {
                        tab_session.ensure_fresh_snapshot();
                        let caret = tab_session.viewport.caret();
                        let snapshot = tab_session.snapshot.clone();

                        let apply_replace = modifiers.alt
                            && matches!(
                                find.authority.borrow().find_replace.mode(),
                                FindReplaceMode::Replace
                            );

                        if apply_replace {
                            let read_only = find.authority.borrow().read_only;
                            if read_only != ReadOnlyState::Writable {
                                find.status_line =
                                    Some("Replace blocked: buffer is read-only".to_string());
                            } else if modifiers.ctrl_or_logo() {
                                let auth = find.authority.borrow_mut();
                                let (mut find_replace, mut buffer) =
                                    std::cell::RefMut::map_split(auth, |auth| {
                                        (&mut auth.find_replace, &mut auth.buffer)
                                    });

                                match find_replace.replace_all(
                                    &mut buffer,
                                    &snapshot,
                                    caret,
                                    "command:editor.replace.allInFile",
                                ) {
                                    Ok(Some(outcome)) => {
                                        tab_session.snapshot = outcome.snapshot;
                                        tab_session.last_seen_revision = outcome.revision;
                                        tab_session.refresh_document_cache();
                                        tab_session
                                            .viewport
                                            .clamp_to_document(&tab_session.line_graphemes);
                                        tab_session.needs_text_repaint = true;
                                        find.status_line = Some(format!(
                                            "Replaced {} matches (lexical-only)",
                                            outcome.replaced_count
                                        ));
                                        let _ = find_replace.sync_for_view(
                                            &tab_session.snapshot,
                                            tab_session.viewport.caret(),
                                        );
                                    }
                                    Ok(None) => {
                                        find.status_line =
                                            Some("No matches to replace".to_string());
                                    }
                                    Err(err) => {
                                        find.status_line =
                                            Some(format!("Replace-all failed: {err}"));
                                    }
                                }
                            } else {
                                let auth = find.authority.borrow_mut();
                                let (mut find_replace, mut buffer) =
                                    std::cell::RefMut::map_split(auth, |auth| {
                                        (&mut auth.find_replace, &mut auth.buffer)
                                    });

                                match find_replace.replace_active(
                                    &mut buffer,
                                    &snapshot,
                                    caret,
                                    "command:editor.replace.activeInFile",
                                ) {
                                    Ok(Some(outcome)) => {
                                        tab_session.snapshot = outcome.snapshot;
                                        tab_session.last_seen_revision = outcome.revision;
                                        tab_session.refresh_document_cache();
                                        tab_session
                                            .viewport
                                            .clamp_to_document(&tab_session.line_graphemes);
                                        tab_session.needs_text_repaint = true;
                                        find.status_line =
                                            Some("Replaced 1 match (lexical-only)".to_string());

                                        let caret = tab_session.viewport.caret();
                                        let _ = find_replace
                                            .sync_for_view(&tab_session.snapshot, caret);
                                        let _ =
                                            find_replace.select_next(&tab_session.snapshot, caret);
                                        if let Some(range) = find_replace.active_match_range() {
                                            if let Some((line, grapheme)) = tab_session
                                                .snapshot
                                                .line_grapheme_for_byte_offset(range.start)
                                            {
                                                let point =
                                                    aureline_editor::TextPoint { line, grapheme };
                                                tab_session.viewport.set_caret(point);
                                                tab_session.viewport.clear_selection();
                                                tab_session.viewport.reveal_line(
                                                    point.line,
                                                    tab_session.max_scroll_line(),
                                                );
                                                let _ = find_replace
                                                    .sync_for_view(&tab_session.snapshot, point);
                                            }
                                        }
                                    }
                                    Ok(None) => {
                                        find.status_line =
                                            Some("No active match to replace".to_string());
                                    }
                                    Err(err) => {
                                        find.status_line = Some(format!("Replace failed: {err}"));
                                    }
                                }
                            }
                        } else {
                            let mut auth = find.authority.borrow_mut();
                            let _ = if modifiers.shift {
                                auth.find_replace.select_prev(&snapshot, caret)
                            } else {
                                auth.find_replace.select_next(&snapshot, caret)
                            };
                            if let Some(range) = auth.find_replace.active_match_range() {
                                if let Some((line, grapheme)) =
                                    snapshot.line_grapheme_for_byte_offset(range.start)
                                {
                                    let point = aureline_editor::TextPoint { line, grapheme };
                                    tab_session.viewport.set_caret(point);
                                    tab_session.viewport.clear_selection();
                                    tab_session
                                        .viewport
                                        .reveal_line(point.line, tab_session.max_scroll_line());
                                    let _ = auth.find_replace.sync_for_view(&snapshot, point);
                                }
                            }
                        }
                    }
                }
                true
            }
            (ShellOverlayKind::FindReplace(find), _) => {
                if modifiers.ctrl_or_logo() || modifiers.alt {
                    false
                } else if let Some(input_text) = text {
                    if let Some(tab) = frame.active_tab_id(frame.focused_editor_group()) {
                        if let Some(tab_session) =
                            editor_runtime.tab_session_mut(frame.focused_editor_group(), tab)
                        {
                            tab_session.ensure_fresh_snapshot();
                            let caret = tab_session.viewport.caret();
                            let snapshot = tab_session.snapshot.clone();

                            let mut auth = find.authority.borrow_mut();
                            let mut changed = false;
                            for ch in input_text.chars() {
                                if ch.is_control() {
                                    continue;
                                }
                                match find.field {
                                    FindReplaceOverlayField::Query => {
                                        let mut query = auth.find_replace.query().to_string();
                                        query.push(ch);
                                        auth.find_replace.set_query(query);
                                    }
                                    FindReplaceOverlayField::Replacement => {
                                        let mut replacement =
                                            auth.find_replace.replacement().to_string();
                                        replacement.push(ch);
                                        auth.find_replace.set_replacement(replacement);
                                    }
                                }
                                changed = true;
                            }

                            if changed {
                                let _ = auth.find_replace.sync_for_view(&snapshot, caret);
                                true
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    false
                }
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
            (ShellOverlayKind::WorkspaceSwitcher(switcher), KeyCode::ArrowDown) => {
                match &mut switcher.mode {
                    WorkspaceSwitcherOverlayMode::List => {
                        let row_count = switcher.rows().len();
                        switcher.state.select_next(row_count);
                    }
                    WorkspaceSwitcherOverlayMode::RecoveryActions {
                        recent_work_id,
                        selection,
                    } => {
                        let action_count = switcher
                            .snapshot
                            .entries
                            .iter()
                            .find(|row| row.recent_work_id == *recent_work_id)
                            .map(|row| row.safe_recovery_actions.len())
                            .unwrap_or(0);
                        if action_count == 0 {
                            *selection = 0;
                        } else {
                            *selection = (*selection + 1) % action_count;
                        }
                    }
                    _ => {}
                }
                true
            }
            (ShellOverlayKind::WorkspaceSwitcher(switcher), KeyCode::ArrowUp) => {
                match &mut switcher.mode {
                    WorkspaceSwitcherOverlayMode::List => {
                        let row_count = switcher.rows().len();
                        switcher.state.select_prev(row_count);
                    }
                    WorkspaceSwitcherOverlayMode::RecoveryActions {
                        recent_work_id,
                        selection,
                    } => {
                        let action_count = switcher
                            .snapshot
                            .entries
                            .iter()
                            .find(|row| row.recent_work_id == *recent_work_id)
                            .map(|row| row.safe_recovery_actions.len())
                            .unwrap_or(0);
                        if action_count == 0 {
                            *selection = 0;
                        } else {
                            *selection = (*selection + action_count - 1) % action_count;
                        }
                    }
                    _ => {}
                }
                true
            }
            (ShellOverlayKind::WorkspaceSwitcher(switcher), KeyCode::Backspace) => {
                matches!(switcher.mode, WorkspaceSwitcherOverlayMode::List)
                    && switcher.state.pop_query_char()
            }
            (ShellOverlayKind::WorkspaceSwitcher(switcher), KeyCode::Delete) => {
                if let Some(selected) = switcher.selected_row() {
                    switcher.mode = WorkspaceSwitcherOverlayMode::ConfirmRemove {
                        recent_work_id: selected.recent_work_id.clone(),
                    };
                    true
                } else {
                    false
                }
            }
            (ShellOverlayKind::WorkspaceSwitcher(switcher), KeyCode::KeyP) => {
                if !matches!(switcher.mode, WorkspaceSwitcherOverlayMode::List) {
                    false
                } else {
                    match switcher.selected_row() {
                        Some(selected) => {
                            if let Some(pinned) = switcher.toggle_pinned(&selected.recent_work_id) {
                                workspace_switcher_decision =
                                    Some(WorkspaceSwitcherDecision::SetPinned {
                                        recent_work_id: selected.recent_work_id,
                                        pinned,
                                    });
                                true
                            } else {
                                false
                            }
                        }
                        None => false,
                    }
                }
            }
            (ShellOverlayKind::WorkspaceSwitcher(switcher), KeyCode::Enter) => {
                let mode = switcher.mode.clone();
                match mode {
                    WorkspaceSwitcherOverlayMode::List => {
                        let Some(selected) = switcher.selected_row() else {
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                entry_flow_decision,
                                workspace_switcher_decision,
                                settings_decision,
                            };
                        };

                        let entry = switcher
                            .snapshot
                            .entries
                            .iter()
                            .find(|row| row.recent_work_id == selected.recent_work_id);
                        let Some(entry) = entry else {
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                entry_flow_decision,
                                workspace_switcher_decision,
                                settings_decision,
                            };
                        };

                        let requires_placeholder =
                            aureline_workspace::classify_recent_work_failure(entry)
                                .requires_placeholder();
                        let allows_open = entry.safe_recovery_actions.iter().any(|action| {
                            matches!(
                                action,
                                SafeRecoveryAction::Open
                                    | SafeRecoveryAction::OpenInNewWindow
                                    | SafeRecoveryAction::OpenRestricted
                                    | SafeRecoveryAction::OpenReadOnlyCachedView
                                    | SafeRecoveryAction::OpenWithoutRestore
                            )
                        });

                        if requires_placeholder || !allows_open {
                            switcher.mode = WorkspaceSwitcherOverlayMode::RecoveryActions {
                                recent_work_id: entry.recent_work_id.clone(),
                                selection: 0,
                            };
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                entry_flow_decision,
                                workspace_switcher_decision,
                                settings_decision,
                            };
                        }

                        if switcher.requires_switch_preview(entry) {
                            switcher.mode = WorkspaceSwitcherOverlayMode::ConfirmSwitch {
                                recent_work_id: entry.recent_work_id.clone(),
                            };
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                entry_flow_decision,
                                workspace_switcher_decision,
                                settings_decision,
                            };
                        }

                        workspace_switcher_decision = Some(WorkspaceSwitcherDecision::Activate {
                            recent_work_id: entry.recent_work_id.clone(),
                        });
                        self.close(frame);
                        true
                    }
                    WorkspaceSwitcherOverlayMode::ConfirmSwitch { recent_work_id } => {
                        workspace_switcher_decision =
                            Some(WorkspaceSwitcherDecision::Activate { recent_work_id });
                        self.close(frame);
                        true
                    }
                    WorkspaceSwitcherOverlayMode::ConfirmRemove { recent_work_id } => {
                        if switcher.remove(&recent_work_id) {
                            workspace_switcher_decision = Some(WorkspaceSwitcherDecision::Remove {
                                recent_work_id: recent_work_id.clone(),
                            });
                            switcher.mode = WorkspaceSwitcherOverlayMode::List;
                            true
                        } else {
                            false
                        }
                    }
                    WorkspaceSwitcherOverlayMode::RecoveryActions {
                        recent_work_id,
                        selection,
                    } => {
                        let entry = switcher
                            .snapshot
                            .entries
                            .iter()
                            .find(|row| row.recent_work_id == recent_work_id);
                        let Some(entry) = entry else {
                            switcher.mode = WorkspaceSwitcherOverlayMode::List;
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                entry_flow_decision,
                                workspace_switcher_decision,
                                settings_decision,
                            };
                        };

                        let Some(action) = entry.safe_recovery_actions.get(selection).copied()
                        else {
                            switcher.mode = WorkspaceSwitcherOverlayMode::List;
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                entry_flow_decision,
                                workspace_switcher_decision,
                                settings_decision,
                            };
                        };

                        match action {
                            SafeRecoveryAction::RemoveFromRecents => {
                                switcher.mode = WorkspaceSwitcherOverlayMode::ConfirmRemove {
                                    recent_work_id: recent_work_id.clone(),
                                };
                                true
                            }
                            _ => {
                                workspace_switcher_decision =
                                    Some(WorkspaceSwitcherDecision::PerformRecoveryAction {
                                        recent_work_id: recent_work_id.clone(),
                                        action,
                                    });
                                switcher.mode = WorkspaceSwitcherOverlayMode::List;
                                true
                            }
                        }
                    }
                }
            }
            _ => false,
        };

        if !handled {
            if let ShellOverlayKind::EntryFlowSheet(sheet) = &mut self.kind {
                if let Some(form) = sheet.clone_form.as_mut() {
                    if form.running || modifiers.ctrl_or_logo() || modifiers.alt {
                        return OverlayKeyOutcome {
                            handled,
                            command_decision,
                            entry_flow_decision,
                            workspace_switcher_decision,
                            settings_decision,
                        };
                    }
                    if let Some(text) = text {
                        if form.push_text(text) {
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                entry_flow_decision,
                                workspace_switcher_decision,
                                settings_decision,
                            };
                        }
                    }
                } else if let Some(form) = sheet.import_form.as_mut() {
                    if form.applied || modifiers.ctrl_or_logo() || modifiers.alt {
                        return OverlayKeyOutcome {
                            handled,
                            command_decision,
                            entry_flow_decision,
                            workspace_switcher_decision,
                            settings_decision,
                        };
                    }
                    if let Some(text) = text {
                        if form.push_text(text) {
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                entry_flow_decision,
                                workspace_switcher_decision,
                                settings_decision,
                            };
                        }
                    }
                }
            }

            if let ShellOverlayKind::WorkspaceSwitcher(switcher) = &mut self.kind {
                if !matches!(switcher.mode, WorkspaceSwitcherOverlayMode::List) {
                    return OverlayKeyOutcome {
                        handled,
                        command_decision,
                        entry_flow_decision,
                        workspace_switcher_decision,
                        settings_decision,
                    };
                }
                if let Some(text) = text {
                    let mut changed = false;
                    for ch in text.chars() {
                        changed |= switcher.state.push_query_char(ch);
                    }
                    if changed {
                        return OverlayKeyOutcome {
                            handled: true,
                            command_decision,
                            entry_flow_decision,
                            workspace_switcher_decision,
                            settings_decision,
                        };
                    }
                }
            }
        }

        OverlayKeyOutcome {
            handled,
            command_decision,
            entry_flow_decision,
            workspace_switcher_decision,
            settings_decision,
        }
    }
}

fn draw_shell_overlay(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    scale_factor: f64,
    token_registry: &TokenRegistry,
    reduced_motion_posture: AccessibilityPostureClass,
    now: Instant,
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
    let (enter_progress, substitution, _) = overlay_dialog_enter_progress(
        token_registry,
        reduced_motion_posture,
        overlay.opened_at,
        now,
    );
    let fade = |color: ColorRgba| scale_alpha(color, enter_progress);

    let mut sheet_rect = Rect::new(
        overlay_rect.right().saturating_sub(sheet_w),
        overlay_rect.y.saturating_add(sheet_margin_y),
        sheet_w,
        overlay_rect
            .height
            .saturating_sub(sheet_margin_y.saturating_mul(2)),
    );
    if substitution.is_none() && enter_progress < 1.0 {
        let offset = (style.space_4 as f32 * (1.0 - enter_progress))
            .round()
            .max(0.0) as u32;
        sheet_rect.x = sheet_rect.x.saturating_add(offset);
    }

    fill_rect(
        buffer,
        width,
        height,
        overlay_rect,
        fade(style.tokens.bg_overlay),
    );
    fill_rect(
        buffer,
        width,
        height,
        sheet_rect,
        fade(style.tokens.bg_raised),
    );
    stroke_rect(
        buffer,
        width,
        height,
        sheet_rect,
        style.stroke_default,
        fade(style.tokens.border_strong),
    );
    if frame.focused_zone() == ShellZoneId::TransientOverlay {
        let ring = style.component_states.focus_ring_style();
        stroke_rect(
            buffer,
            width,
            height,
            sheet_rect,
            ring.stroke_px,
            fade(ring.color),
        );
    }

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
                    fade(style.tokens.text_muted),
                );
                cursor_y = cursor_y.saturating_add(line_h);
            }
        }
        ShellOverlayKind::SaveReview(review) => {
            let mut lines = save_review_sheet_lines(&review.record, review.selection);
            if let Some(status) = review.status_line.as_deref() {
                lines.push("".to_string());
                lines.push(format!("status: {status}"));
            }

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
                    0 => fade(style.tokens.text_primary),
                    1 => fade(style.tokens.text_secondary),
                    _ => fade(style.tokens.text_muted),
                };
                draw_text(buffer, width, height, cursor_x, cursor_y, 1, &line, color);
                cursor_y = cursor_y.saturating_add(line_h);
            }
        }
        ShellOverlayKind::FindReplace(find) => {
            let auth = find.authority.borrow();
            let state = &auth.find_replace;
            let mode = state.mode();

            let find_keys = keybinding_runtime.shortcuts_label("cmd:editor.find");
            let replace_keys = keybinding_runtime.shortcuts_label("cmd:editor.replace");
            let header = match mode {
                FindReplaceMode::Find => format!(
                    "Find (lexical-only) — Esc closes   {}",
                    if find_keys == "unbound" {
                        String::new()
                    } else {
                        format!("keys: {find_keys}")
                    }
                ),
                FindReplaceMode::Replace => format!(
                    "Find/Replace (lexical-only) — Esc closes   {}",
                    if replace_keys == "unbound" {
                        String::new()
                    } else {
                        format!("keys: {replace_keys}")
                    }
                ),
                FindReplaceMode::Hidden => "Find/Replace (lexical-only) — Esc closes".to_string(),
            };

            let mut cursor_y = sheet_rect.y.saturating_add(style.space_3);
            let cursor_x = sheet_rect.x.saturating_add(style.space_3);
            let line_h = 8u32.saturating_add(style.space_2.saturating_mul(3) / 4);

            draw_text(
                buffer,
                width,
                height,
                cursor_x,
                cursor_y,
                1,
                &header.trim().to_string(),
                fade(style.tokens.text_primary),
            );
            cursor_y = cursor_y.saturating_add(16);

            let query_prefix = if matches!(find.field, FindReplaceOverlayField::Query) {
                ">"
            } else {
                " "
            };
            let replacement_prefix = if matches!(find.field, FindReplaceOverlayField::Replacement) {
                ">"
            } else {
                " "
            };

            let query = state.query();
            let query_line = if query.is_empty() {
                format!("{query_prefix} find: (empty)")
            } else {
                format!("{query_prefix} find: {query}")
            };
            draw_text(
                buffer,
                width,
                height,
                cursor_x,
                cursor_y,
                1,
                &query_line,
                fade(style.tokens.text_secondary),
            );
            cursor_y = cursor_y.saturating_add(16);

            if mode == FindReplaceMode::Replace {
                let replacement = state.replacement();
                let replacement_line = if replacement.is_empty() {
                    format!("{replacement_prefix} replace: (empty)")
                } else {
                    format!("{replacement_prefix} replace: {replacement}")
                };
                draw_text(
                    buffer,
                    width,
                    height,
                    cursor_x,
                    cursor_y,
                    1,
                    &replacement_line,
                    fade(style.tokens.text_secondary),
                );
                cursor_y = cursor_y.saturating_add(16);
            }

            let options = state.options();
            let mut option_tokens = Vec::new();
            option_tokens.push(if options.case_sensitive {
                "Case: sensitive"
            } else {
                "Case: ASCII-insensitive"
            });
            option_tokens.push(if options.whole_word {
                "Word: whole"
            } else {
                "Word: any"
            });
            option_tokens.push("Pattern: literal");
            option_tokens.push("Scope: file");
            let options_line = format!("opts: {}", option_tokens.join("   "));
            draw_text(
                buffer,
                width,
                height,
                cursor_x,
                cursor_y,
                1,
                &options_line,
                fade(style.tokens.text_muted),
            );
            cursor_y = cursor_y.saturating_add(16);

            let count = state.match_count();
            let active = state
                .active_match_index()
                .map(|idx| idx.saturating_add(1))
                .unwrap_or(0);
            let matches_line = if count == 0 {
                "matches: 0".to_string()
            } else {
                format!("matches: {active}/{count}   (Down/Up steps, Enter steps)")
            };
            draw_text(
                buffer,
                width,
                height,
                cursor_x,
                cursor_y,
                1,
                &matches_line,
                fade(style.tokens.text_muted),
            );
            cursor_y = cursor_y.saturating_add(16);

            if let Some(reason) = state.degraded_reason() {
                let line = match reason {
                    aureline_editor::find_replace::FindReplaceDegradedReason::NonUtf8Snapshot => {
                        "degraded: non-UTF8 snapshot (find/replace blocked)".to_string()
                    }
                    aureline_editor::find_replace::FindReplaceDegradedReason::ScanBudgetExceeded {
                        scanned_bytes,
                        total_bytes,
                    } => format!("degraded: scanned {scanned_bytes}/{total_bytes} bytes"),
                    aureline_editor::find_replace::FindReplaceDegradedReason::MatchBudgetExceeded {
                        match_cap,
                    } => format!("degraded: match cap {match_cap} reached"),
                };
                draw_text(
                    buffer,
                    width,
                    height,
                    cursor_x,
                    cursor_y,
                    1,
                    &line,
                    fade(style.status_warning),
                );
                cursor_y = cursor_y.saturating_add(16);
            }

            if let Some(status_line) = find.status_line.as_deref() {
                draw_text(
                    buffer,
                    width,
                    height,
                    cursor_x,
                    cursor_y,
                    1,
                    status_line,
                    fade(style.status_warning),
                );
                cursor_y = cursor_y.saturating_add(16);
            }

            let footer = if mode == FindReplaceMode::Replace {
                "Tab switch field, Alt+C case, Alt+W whole-word, Alt+Enter replace, Ctrl+Alt+Enter replace all"
            } else {
                "Tab opens replace, Alt+C case, Alt+W whole-word"
            };
            if cursor_y.saturating_add(line_h) <= sheet_rect.bottom().saturating_sub(style.space_3)
            {
                draw_text(
                    buffer,
                    width,
                    height,
                    cursor_x,
                    cursor_y,
                    1,
                    footer,
                    fade(style.tokens.text_muted),
                );
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
                    0 => fade(style.tokens.text_primary),
                    1 => fade(style.tokens.text_secondary),
                    _ => fade(style.tokens.text_muted),
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
                    0 => fade(style.tokens.text_primary),
                    1 => fade(style.tokens.text_secondary),
                    _ => fade(style.tokens.text_muted),
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
                fade(style.tokens.text_primary),
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
                fade(style.tokens.text_muted),
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
                    fade(style.tokens.text_muted),
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
                    fade(style.tokens.text_muted),
                );
            }
        }
        ShellOverlayKind::WedgeInspector(sheet) => {
            let mut y = sheet_rect.y.saturating_add(style.space_3);
            let x = sheet_rect.x.saturating_add(style.space_3);
            let max_x = sheet_rect.right().saturating_sub(style.space_3).max(x);
            for (idx, line) in sheet.inspector.render_lines().iter().enumerate() {
                if y.saturating_add(14) > sheet_rect.bottom().saturating_sub(style.space_3) {
                    break;
                }
                let color = match idx {
                    0 => fade(style.tokens.text_primary),
                    1 => fade(style.tokens.text_secondary),
                    _ => fade(style.tokens.text_muted),
                };
                draw_text_clamped(buffer, width, height, x, y, 1, line, color, max_x);
                y = y.saturating_add(14);
            }
        }
        ShellOverlayKind::StatusBarItemDetail(detail) => {
            let mut y = sheet_rect.y.saturating_add(style.space_3);
            let x = sheet_rect.x.saturating_add(style.space_3);
            let max_x = sheet_rect.right().saturating_sub(style.space_3).max(x);
            for (idx, line) in detail.lines.iter().enumerate() {
                if y.saturating_add(14) > sheet_rect.bottom().saturating_sub(style.space_3) {
                    break;
                }
                let color = match idx {
                    0 => fade(style.tokens.text_primary),
                    1 | 2 => fade(style.tokens.text_secondary),
                    _ => fade(style.tokens.text_muted),
                };
                draw_text_clamped(buffer, width, height, x, y, 1, line, color, max_x);
                y = y.saturating_add(14);
            }
        }
        ShellOverlayKind::Settings(settings) => {
            let mut y = sheet_rect.y.saturating_add(style.space_3);
            let x = sheet_rect.x.saturating_add(style.space_3);
            let max_x = sheet_rect.right().saturating_sub(style.space_3).max(x);
            draw_text_clamped(
                buffer,
                width,
                height,
                x,
                y,
                1,
                "Settings - Esc closes",
                fade(style.tokens.text_primary),
                max_x,
            );
            y = y.saturating_add(18);

            for (idx, row) in settings.rows.iter().enumerate() {
                if y.saturating_add(34) > sheet_rect.bottom().saturating_sub(style.space_3) {
                    break;
                }
                if idx == settings.selection {
                    let highlight = Rect::new(
                        sheet_rect.x.saturating_add(style.space_2),
                        y.saturating_sub(2),
                        sheet_rect.width.saturating_sub(style.space_4),
                        30,
                    );
                    fill_rect(
                        buffer,
                        width,
                        height,
                        highlight,
                        fade(style.tokens.bg_hover),
                    );
                }
                let summary = format!(
                    "{}: {}   source: {}   lock: {}",
                    row.label, row.value, row.source_scope, row.lock_state
                );
                draw_text_clamped(
                    buffer,
                    width,
                    height,
                    x,
                    y,
                    1,
                    &summary,
                    if idx == settings.selection {
                        fade(style.tokens.text_primary)
                    } else {
                        fade(style.tokens.text_secondary)
                    },
                    max_x,
                );
                y = y.saturating_add(14);
                let detail = format!(
                    "{}   {}   reason: {}",
                    row.setting_id, row.source_label, row.lock_reason
                );
                draw_text_clamped(
                    buffer,
                    width,
                    height,
                    x,
                    y,
                    1,
                    &detail,
                    fade(style.tokens.text_muted),
                    max_x,
                );
                y = y.saturating_add(20);
            }

            if let Some(status) = settings.status_line.as_deref() {
                if y.saturating_add(14) <= sheet_rect.bottom().saturating_sub(style.space_3) {
                    draw_text_clamped(
                        buffer,
                        width,
                        height,
                        x,
                        y,
                        1,
                        status,
                        fade(style.status_warning),
                        max_x,
                    );
                }
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
                fade(style.tokens.text_primary),
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
                fade(style.tokens.text_secondary),
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
                    fill_rect(
                        buffer,
                        width,
                        height,
                        highlight,
                        fade(style.tokens.bg_hover),
                    );
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
                        fade(style.tokens.text_primary)
                    } else {
                        fade(style.tokens.text_muted)
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
                fade(style.tokens.text_primary),
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
                fade(style.tokens.text_muted),
            );
        }
        ShellOverlayKind::EntryFlowSheet(sheet) => {
            let mut cursor_y = sheet_rect.y.saturating_add(style.space_3);
            let cursor_x = sheet_rect.x.saturating_add(style.space_3);

            let header = match &sheet.outcome {
                EntryFlowOutcome::Resolved(resolved) => format!(
                    "{} flow sheet — Enter commit, Esc cancel",
                    resolved.sheet_title()
                ),
                EntryFlowOutcome::Denied(denied) => format!(
                    "{} flow denied — Esc closes",
                    match denied.entry_verb {
                        EntryVerb::Open => "Open",
                        EntryVerb::Clone => "Clone",
                        EntryVerb::Import => "Import",
                        EntryVerb::Restore => "Restore",
                        EntryVerb::AddRoot => "Add root",
                        EntryVerb::Resume => "Resume",
                        EntryVerb::StartFromSnapshot => "Start from snapshot",
                    }
                ),
            };

            draw_text(
                buffer,
                width,
                height,
                cursor_x,
                cursor_y,
                1,
                &header,
                fade(style.tokens.text_primary),
            );
            cursor_y = cursor_y.saturating_add(16);

            if let Some(token) = sheet.degraded_token {
                let _ = draw_status_badge(
                    buffer,
                    width,
                    height,
                    cursor_x,
                    cursor_y,
                    sheet_rect.bottom().saturating_sub(cursor_y),
                    style,
                    token.label(),
                    fade(style.status_warning),
                    fade(style.status_warning_border),
                    fade(style.status_warning_fill),
                );
                cursor_y = cursor_y.saturating_add(20);
            }

            let command_line = format!(
                "command: {}   origin: {}",
                sheet.command_id,
                sheet.origin.issuing_surface()
            );
            draw_text(
                buffer,
                width,
                height,
                cursor_x,
                cursor_y,
                1,
                &command_line,
                fade(style.tokens.text_secondary),
            );
            cursor_y = cursor_y.saturating_add(16);

            match &sheet.outcome {
                EntryFlowOutcome::Resolved(resolved) => {
                    let sheet_line = format!("sheet_class: {}", resolved.sheet_class.as_str());
                    draw_text(
                        buffer,
                        width,
                        height,
                        cursor_x,
                        cursor_y,
                        1,
                        &sheet_line,
                        fade(style.tokens.text_muted),
                    );
                    cursor_y = cursor_y.saturating_add(16);

                    let target_line = format!("target_kind: {}", resolved.target_kind.as_str());
                    draw_text(
                        buffer,
                        width,
                        height,
                        cursor_x,
                        cursor_y,
                        1,
                        &target_line,
                        fade(style.tokens.text_muted),
                    );
                    cursor_y = cursor_y.saturating_add(16);

                    let result_line =
                        format!("resulting_mode: {}", resolved.resulting_mode.as_str());
                    draw_text(
                        buffer,
                        width,
                        height,
                        cursor_x,
                        cursor_y,
                        1,
                        &result_line,
                        fade(style.tokens.text_muted),
                    );
                    cursor_y = cursor_y.saturating_add(16);

                    if cursor_y < sheet_rect.bottom().saturating_sub(style.space_6) {
                        let mut candidates = String::new();
                        candidates.push_str("candidates: ");
                        for (idx, mode) in resolved.candidate_resulting_modes.iter().enumerate() {
                            if idx > 0 {
                                candidates.push_str(", ");
                            }
                            candidates.push_str(mode.as_str());
                            if candidates.len() > 96 {
                                candidates.push_str(", …");
                                break;
                            }
                        }
                        draw_text(
                            buffer,
                            width,
                            height,
                            cursor_x,
                            cursor_y,
                            1,
                            &candidates,
                            fade(style.tokens.text_muted),
                        );
                        cursor_y = cursor_y.saturating_add(16);
                    }

                    if let Some(packet) = sheet.admission_review.as_ref() {
                        for line in compact_admission_review_lines(packet).into_iter().take(4) {
                            if cursor_y.saturating_add(14)
                                > sheet_rect.bottom().saturating_sub(style.space_3)
                            {
                                break;
                            }
                            draw_text_clamped(
                                buffer,
                                width,
                                height,
                                cursor_x,
                                cursor_y,
                                1,
                                &line,
                                style.tokens.text_muted,
                                sheet_rect
                                    .right()
                                    .saturating_sub(style.space_3)
                                    .max(cursor_x),
                            );
                            cursor_y = cursor_y.saturating_add(16);
                        }
                    }

                    if let Some(form) = sheet.clone_form.as_ref() {
                        cursor_y = draw_clone_form_lines(
                            buffer, width, height, cursor_x, cursor_y, sheet_rect, form, style,
                        );
                    } else if let Some(form) = sheet.import_form.as_ref() {
                        cursor_y = draw_import_form_lines(
                            buffer, width, height, cursor_x, cursor_y, sheet_rect, form, style,
                        );
                    }
                }
                EntryFlowOutcome::Denied(denied) => {
                    let line = format!("denial_code: {}", denied.denial_code.as_str());
                    draw_text(
                        buffer,
                        width,
                        height,
                        cursor_x,
                        cursor_y,
                        1,
                        &line,
                        fade(style.status_warning),
                    );
                    cursor_y = cursor_y.saturating_add(16);
                    if let Some(reroute) = denied.suggested_reroute {
                        let line = format!("suggested_reroute: {}", reroute.as_str());
                        draw_text(
                            buffer,
                            width,
                            height,
                            cursor_x,
                            cursor_y,
                            1,
                            &line,
                            fade(style.tokens.text_muted),
                        );
                        cursor_y = cursor_y.saturating_add(16);
                    }
                }
            }

            if let Some(note) = sheet.note.as_deref() {
                if cursor_y.saturating_add(14) <= sheet_rect.bottom().saturating_sub(style.space_3)
                {
                    draw_text(
                        buffer,
                        width,
                        height,
                        cursor_x,
                        cursor_y,
                        1,
                        note,
                        fade(style.tokens.text_muted),
                    );
                }
            }
        }
        ShellOverlayKind::WorkspaceSwitcher(switcher) => {
            let mut cursor_y = sheet_rect.y.saturating_add(style.space_3);
            let cursor_x = sheet_rect.x.saturating_add(style.space_3);

            let header = format!(
                "{} — type to filter, Up/Down select, Enter open/actions, P pin, Del remove, Esc back/close",
                WORKSPACE_SWITCHER_PRESENTATION_LABEL
            );
            draw_text(
                buffer,
                width,
                height,
                cursor_x,
                cursor_y,
                1,
                &header,
                fade(style.tokens.text_primary),
            );
            cursor_y = cursor_y.saturating_add(16);

            let query = switcher.state.query();
            let query_line = if query.is_empty() {
                "filter: (empty)".to_string()
            } else {
                format!("filter: {query}")
            };
            draw_text(
                buffer,
                width,
                height,
                cursor_x,
                cursor_y,
                1,
                &query_line,
                fade(style.tokens.text_secondary),
            );
            cursor_y = cursor_y.saturating_add(16);

            let mode_line = match &switcher.mode {
                WorkspaceSwitcherOverlayMode::List => None,
                WorkspaceSwitcherOverlayMode::ConfirmSwitch { .. } => {
                    Some("Preview required: Enter confirm switch, Esc cancel".to_string())
                }
                WorkspaceSwitcherOverlayMode::ConfirmRemove { .. } => {
                    Some("Remove from recent: Enter confirm, Esc cancel".to_string())
                }
                WorkspaceSwitcherOverlayMode::RecoveryActions { .. } => {
                    Some("Recovery actions: Up/Down select, Enter apply, Esc back".to_string())
                }
            };
            if let Some(mode_line) = mode_line {
                draw_text(
                    buffer,
                    width,
                    height,
                    cursor_x,
                    cursor_y,
                    1,
                    &mode_line,
                    fade(style.tokens.text_muted),
                );
                cursor_y = cursor_y.saturating_add(16);
            }

            if let WorkspaceSwitcherOverlayMode::RecoveryActions {
                recent_work_id,
                selection,
            } = &switcher.mode
            {
                let entry = switcher
                    .snapshot
                    .entries
                    .iter()
                    .find(|row| row.recent_work_id == *recent_work_id);
                if let Some(entry) = entry {
                    let subtitle = entry
                        .presentation_subtitle
                        .as_deref()
                        .unwrap_or("no location metadata");
                    let summary = format!(
                        "{} — {}  ({}, {}, {}, {}, {})",
                        entry.presentation_label,
                        subtitle,
                        entry.target_kind.surface_label(),
                        entry.target_state.as_str(),
                        aureline_workspace::classify_recent_work_failure(entry).as_str(),
                        entry.trust_state.as_str(),
                        match entry.restore_availability {
                            RestoreAvailability::Exact => "restore:exact",
                            RestoreAvailability::Compatible => "restore:compatible",
                            RestoreAvailability::LayoutOnly => "restore:layout_only",
                            RestoreAvailability::EvidenceOnly => "restore:evidence_only",
                            RestoreAvailability::None => "restore:none",
                        }
                    );
                    draw_text(
                        buffer,
                        width,
                        height,
                        cursor_x,
                        cursor_y,
                        1,
                        &summary,
                        fade(style.tokens.text_secondary),
                    );
                    cursor_y = cursor_y.saturating_add(16);

                    if let Some(card) = recent_work_placeholder_card(
                        entry,
                        crate::restore::placeholders::PlaceholderSurfaceClass::WorkspaceSwitcher,
                    ) {
                        draw_text_clamped(
                            buffer,
                            width,
                            height,
                            cursor_x,
                            cursor_y,
                            1,
                            &card.recovery_summary,
                            fade(style.tokens.text_muted),
                            sheet_rect.right().saturating_sub(style.space_3),
                        );
                        cursor_y = cursor_y.saturating_add(16);
                    }

                    let last_opened = format!("last_opened_at: {}", entry.last_opened_at);
                    draw_text(
                        buffer,
                        width,
                        height,
                        cursor_x,
                        cursor_y,
                        1,
                        &last_opened,
                        fade(style.tokens.text_muted),
                    );
                    cursor_y = cursor_y.saturating_add(16);

                    let line_h = 18u32;
                    for (idx, action) in entry.safe_recovery_actions.iter().enumerate() {
                        let y = cursor_y.saturating_add((idx as u32) * line_h);
                        if y.saturating_add(line_h)
                            > sheet_rect.bottom().saturating_sub(style.space_3)
                        {
                            break;
                        }

                        if idx == *selection {
                            let highlight = Rect::new(
                                sheet_rect.x.saturating_add(style.space_2),
                                y.saturating_sub(2),
                                sheet_rect.width.saturating_sub(style.space_4),
                                16,
                            );
                            fill_rect(
                                buffer,
                                width,
                                height,
                                highlight,
                                fade(style.tokens.bg_hover),
                            );
                        }

                        let line = format!("{} — {}", action.as_str(), action.as_str());
                        draw_text(
                            buffer,
                            width,
                            height,
                            cursor_x,
                            y,
                            1,
                            &line,
                            if idx == *selection {
                                fade(style.tokens.text_primary)
                            } else {
                                fade(style.tokens.text_muted)
                            },
                        );
                    }
                } else {
                    draw_text(
                        buffer,
                        width,
                        height,
                        cursor_x,
                        cursor_y,
                        1,
                        "Selected entry missing from snapshot.",
                        fade(style.tokens.text_muted),
                    );
                }
                return;
            }

            let rows = switcher.rows();
            let selected = switcher.state.selection().min(rows.len().saturating_sub(1));

            let line_h = 18u32;
            for (idx, row) in rows.iter().enumerate() {
                let y = cursor_y.saturating_add((idx as u32) * line_h);
                if y.saturating_add(line_h) > sheet_rect.bottom().saturating_sub(style.space_3) {
                    break;
                }

                if idx == selected {
                    let highlight = Rect::new(
                        sheet_rect.x.saturating_add(style.space_2),
                        y.saturating_sub(2),
                        sheet_rect.width.saturating_sub(style.space_4),
                        16,
                    );
                    fill_rect(
                        buffer,
                        width,
                        height,
                        highlight,
                        fade(style.tokens.bg_hover),
                    );
                }

                let pin = if row.pinned { "*" } else { " " };
                let subtitle = row
                    .location_or_target_subtitle
                    .as_deref()
                    .unwrap_or("no location metadata");
                let classes = row
                    .entry_classes
                    .iter()
                    .map(|class| class.as_str())
                    .collect::<Vec<_>>()
                    .join("+");
                let line = format!(
                    "{pin} {} — {}  last:{}  ({}, {}, {}, {}, {}, {})",
                    row.primary_label,
                    subtitle,
                    row.last_opened_at,
                    row.target_kind_label,
                    classes,
                    row.target_state.as_str(),
                    row.failure_state.as_str(),
                    row.trust_state.as_str(),
                    match row.restore_availability {
                        RestoreAvailability::Exact => "restore:exact",
                        RestoreAvailability::Compatible => "restore:compatible",
                        RestoreAvailability::LayoutOnly => "restore:layout_only",
                        RestoreAvailability::EvidenceOnly => "restore:evidence_only",
                        RestoreAvailability::None => "restore:none",
                    }
                );
                draw_text(
                    buffer,
                    width,
                    height,
                    cursor_x,
                    y,
                    1,
                    &line,
                    if idx == selected {
                        fade(style.tokens.text_primary)
                    } else {
                        fade(style.tokens.text_muted)
                    },
                );
            }

            if rows.is_empty() {
                draw_text(
                    buffer,
                    width,
                    height,
                    cursor_x,
                    cursor_y,
                    1,
                    "No recent work yet. Use Open Folder to seed a row.",
                    fade(style.tokens.text_muted),
                );
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn draw_clone_form_lines(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    x: u32,
    mut y: u32,
    sheet_rect: Rect,
    form: &CloneFlowForm,
    style: &ShellRenderStyle,
) -> u32 {
    let max_x = sheet_rect.right().saturating_sub(style.space_3).max(x);
    let rows = [
        (
            CloneFlowField::RemoteUrl,
            "remote_url",
            form.remote_url.as_str(),
        ),
        (
            CloneFlowField::DestinationPath,
            "destination_path",
            form.destination_path.as_str(),
        ),
    ];

    for (field, label, value) in rows {
        if y.saturating_add(14) > sheet_rect.bottom().saturating_sub(style.space_3) {
            return y;
        }
        let prefix = if form.focused_field == field {
            ">"
        } else {
            " "
        };
        let display_value = if value.trim().is_empty() {
            "(required)"
        } else {
            value
        };
        let color = if form.focused_field == field {
            style.tokens.text_primary
        } else {
            style.tokens.text_muted
        };
        draw_text_clamped(
            buffer,
            width,
            height,
            x,
            y,
            1,
            &format!("{prefix} {label}: {display_value}"),
            color,
            max_x,
        );
        y = y.saturating_add(16);
    }

    if let Some(packet) = form.admission_review_packet() {
        for line in compact_admission_review_lines(&packet).into_iter().take(5) {
            if y.saturating_add(14) > sheet_rect.bottom().saturating_sub(style.space_3) {
                return y;
            }
            draw_text_clamped(
                buffer,
                width,
                height,
                x,
                y,
                1,
                &line,
                style.tokens.text_secondary,
                max_x,
            );
            y = y.saturating_add(16);
        }
    }

    if y.saturating_add(14) <= sheet_rect.bottom().saturating_sub(style.space_3) {
        let state = if form.running {
            "running"
        } else if form.submit_enabled() {
            "enabled"
        } else {
            "disabled"
        };
        let color = if form.submit_enabled() || form.running {
            style.status_success
        } else {
            style.tokens.text_muted
        };
        draw_text_clamped(
            buffer,
            width,
            height,
            x,
            y,
            1,
            &format!("submit: {state}"),
            color,
            max_x,
        );
        y = y.saturating_add(16);
    }

    if let Some(status) = form.status_line.as_deref() {
        if y.saturating_add(14) <= sheet_rect.bottom().saturating_sub(style.space_3) {
            draw_text_clamped(
                buffer,
                width,
                height,
                x,
                y,
                1,
                status,
                style.tokens.text_secondary,
                max_x,
            );
            y = y.saturating_add(16);
        }
    }

    y
}

#[allow(clippy::too_many_arguments)]
fn draw_import_form_lines(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    x: u32,
    mut y: u32,
    sheet_rect: Rect,
    form: &ImportFlowForm,
    style: &ShellRenderStyle,
) -> u32 {
    let max_x = sheet_rect.right().saturating_sub(style.space_3).max(x);
    let rows = [
        (
            ImportFlowField::SourcePath,
            "source_path",
            form.source_path.as_str(),
        ),
        (
            ImportFlowField::DestinationWorkspaceTarget,
            "destination_target",
            form.destination_workspace_target.as_str(),
        ),
    ];

    for (field, label, value) in rows {
        if y.saturating_add(14) > sheet_rect.bottom().saturating_sub(style.space_3) {
            return y;
        }
        let prefix = if form.focused_field == field {
            ">"
        } else {
            " "
        };
        let display_value = if value.trim().is_empty() {
            "(required)"
        } else {
            value
        };
        let color = if form.focused_field == field {
            style.tokens.text_primary
        } else {
            style.tokens.text_muted
        };
        draw_text_clamped(
            buffer,
            width,
            height,
            x,
            y,
            1,
            &format!("{prefix} {label}: {display_value}"),
            color,
            max_x,
        );
        y = y.saturating_add(16);
    }

    if let Some(packet) = form.admission_review_packet() {
        for line in compact_admission_review_lines(&packet).into_iter().take(5) {
            if y.saturating_add(14) > sheet_rect.bottom().saturating_sub(style.space_3) {
                return y;
            }
            draw_text_clamped(
                buffer,
                width,
                height,
                x,
                y,
                1,
                &line,
                style.tokens.text_secondary,
                max_x,
            );
            y = y.saturating_add(16);
        }
    }

    if let Some(record) = form.review_record.as_ref() {
        let review_lines = [
            format!("classification: {}", record.classification.variant_name()),
            format!("decision_class: {}", record.decision_class.as_str()),
            format!("items: {}", record.discovered_item_count_label()),
        ];
        for line in review_lines {
            if y.saturating_add(14) > sheet_rect.bottom().saturating_sub(style.space_3) {
                return y;
            }
            draw_text_clamped(
                buffer,
                width,
                height,
                x,
                y,
                1,
                &line,
                style.tokens.text_muted,
                max_x,
            );
            y = y.saturating_add(16);
        }

        for item in record.discovered_items.iter().take(5) {
            if y.saturating_add(14) > sheet_rect.bottom().saturating_sub(style.space_3) {
                return y;
            }
            let line = format!(
                "- {} [{}]",
                item.source_relative_path,
                item.item_kind.as_str()
            );
            draw_text_clamped(
                buffer,
                width,
                height,
                x,
                y,
                1,
                &line,
                style.tokens.text_secondary,
                max_x,
            );
            y = y.saturating_add(16);
        }
        if record.discovered_items.len() > 5
            && y.saturating_add(14) <= sheet_rect.bottom().saturating_sub(style.space_3)
        {
            draw_text_clamped(
                buffer,
                width,
                height,
                x,
                y,
                1,
                &format!("+ {} more", record.discovered_items.len() - 5),
                style.tokens.text_muted,
                max_x,
            );
            y = y.saturating_add(16);
        }
    } else if y.saturating_add(14) <= sheet_rect.bottom().saturating_sub(style.space_3) {
        let state = if form.review_enabled() {
            "review_ready"
        } else {
            "waiting_for_source"
        };
        draw_text_clamped(
            buffer,
            width,
            height,
            x,
            y,
            1,
            &format!("review: {state}"),
            style.tokens.text_muted,
            max_x,
        );
        y = y.saturating_add(16);
    }

    if let Some(packet) = form.diff_review_packet.as_ref() {
        for line in packet.compact_lines().into_iter().take(6) {
            if y.saturating_add(14) > sheet_rect.bottom().saturating_sub(style.space_3) {
                return y;
            }
            draw_text_clamped(
                buffer,
                width,
                height,
                x,
                y,
                1,
                &line,
                style.tokens.text_secondary,
                max_x,
            );
            y = y.saturating_add(16);
        }
    }

    if y.saturating_add(14) <= sheet_rect.bottom().saturating_sub(style.space_3) {
        let apply_state = if form.applied {
            "recorded"
        } else if form.apply_enabled() {
            "enabled"
        } else {
            "disabled"
        };
        let color = if form.apply_enabled() || form.applied {
            style.status_success
        } else {
            style.tokens.text_muted
        };
        draw_text_clamped(
            buffer,
            width,
            height,
            x,
            y,
            1,
            &format!("apply_checkpointed: {apply_state}"),
            color,
            max_x,
        );
        y = y.saturating_add(16);
    }

    if let Some(status) = form.status_line.as_deref() {
        if y.saturating_add(14) <= sheet_rect.bottom().saturating_sub(style.space_3) {
            draw_text_clamped(
                buffer,
                width,
                height,
                x,
                y,
                1,
                status,
                style.tokens.text_secondary,
                max_x,
            );
            y = y.saturating_add(16);
        }
    }

    y
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

fn to_logical_px(physical_px: u32, scale_factor: f64) -> u32 {
    if !scale_factor.is_finite() || scale_factor <= 0.0 {
        return physical_px;
    }
    ((physical_px as f64) / scale_factor).round().max(0.0) as u32
}

fn scale_bucket_for_scale_factor(scale_factor: f64) -> u8 {
    if !scale_factor.is_finite() || scale_factor <= 0.0 {
        return 1;
    }
    let bucket = (scale_factor * 16.0).round() as i64;
    bucket.clamp(1, 255) as u8
}

thread_local! {
    static ACTIVE_RASTER_CLIP: std::cell::Cell<Option<Rect>> = std::cell::Cell::new(None);
}

fn with_raster_clip<T>(clip: Option<Rect>, f: impl FnOnce() -> T) -> T {
    ACTIVE_RASTER_CLIP.with(|cell| {
        let prev = cell.replace(clip);
        let out = f();
        cell.set(prev);
        out
    })
}

fn raster_clip() -> Option<Rect> {
    ACTIVE_RASTER_CLIP.with(|cell| cell.get())
}

fn rect_intersects(a: Rect, b: Rect) -> bool {
    if a.is_empty() || b.is_empty() {
        return false;
    }
    a.x < b.right() && a.right() > b.x && a.y < b.bottom() && a.bottom() > b.y
}

fn clamp_unit(value: f32) -> f32 {
    if value.is_finite() {
        value.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

fn ease_progress(easing_token: Option<&str>, t: f32) -> f32 {
    let t = clamp_unit(t);
    match easing_token {
        Some("ease.enter") => 1.0 - (1.0 - t).powi(3),
        Some("ease.exit") => t.powi(3),
        Some("ease.standard") | _ => t * t * (3.0 - 2.0 * t),
    }
}

fn scale_alpha(color: ColorRgba, factor: f32) -> ColorRgba {
    let factor = clamp_unit(factor);
    let alpha = ((color.a as f32) * factor).round().clamp(0.0, 255.0) as u8;
    ColorRgba { a: alpha, ..color }
}

fn overlay_dialog_enter_progress(
    token_registry: &TokenRegistry,
    posture: AccessibilityPostureClass,
    opened_at: Instant,
    now: Instant,
) -> (f32, Option<ReducedMotionSubstitutionClass>, Duration) {
    let plan = OVERLAY_DIALOG_ENTER.plan_for(posture);
    let duration = plan.duration(token_registry).unwrap_or_default();
    let progress = if duration.is_zero() {
        1.0
    } else {
        let elapsed = now.saturating_duration_since(opened_at);
        clamp_unit(elapsed.as_secs_f32() / duration.as_secs_f32())
    };
    (
        ease_progress(plan.easing_token, progress),
        plan.substitution_class,
        duration,
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
    let mut x0 = rect.x.min(max_x);
    let mut y0 = rect.y.min(max_y);
    let mut x1 = rect.right().min(width);
    let mut y1 = rect.bottom().min(height);

    if let Some(clip) = raster_clip() {
        let clip_x0 = clip.x.min(width);
        let clip_y0 = clip.y.min(height);
        let clip_x1 = clip.right().min(width);
        let clip_y1 = clip.bottom().min(height);
        x0 = x0.max(clip_x0);
        y0 = y0.max(clip_y0);
        x1 = x1.min(clip_x1);
        y1 = y1.min(clip_y1);
        if x0 >= x1 || y0 >= y1 {
            return;
        }
    }

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
    text_runtime: &mut ShellTextRuntime,
    scale_bucket: u8,
    registry: &CommandRegistry,
    start_center: &StartCenterState,
    enablement_runtime: &CommandEnablementRuntimeState,
    rect: Rect,
    style: &ShellRenderStyle,
    focused: bool,
) {
    let padding = style.density_zone_inset;
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

    let content_padding = style.density_panel_padding;
    let header_x = card.x.saturating_add(content_padding);
    let mut y = card.y.saturating_add(content_padding);

    let header_h = draw_ui_text(
        buffer,
        width,
        height,
        header_x,
        y,
        START_CENTER_PRESENTATION_LABEL,
        style.tokens.text_primary,
        20.0,
        scale_bucket,
        text_runtime,
    );
    y = y.saturating_add(header_h).saturating_add(style.space_2);

    let subtitle_h = draw_ui_text(
        buffer,
        width,
        height,
        header_x,
        y,
        START_CENTER_PRESENTATION_SUBTITLE,
        style.tokens.text_secondary,
        14.0,
        scale_bucket,
        text_runtime,
    );
    y = y
        .saturating_add(subtitle_h)
        .saturating_add(style.density_gutter);

    let runtime = StartCenterRuntimeInputs {
        client_scope: "desktop_product",
        workspace_trust_state: enablement_runtime.workspace_trust_state.as_str(),
        execution_context_available: enablement_runtime.execution_context_available,
        provider_linked: enablement_runtime.provider_linked,
        credential_available: enablement_runtime.credential_available,
        policy_disabled: enablement_runtime.policy_disabled,
        policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
        labs_enabled: enablement_runtime.labs_enabled,
    };
    let rows = start_center_action_rows(registry, runtime);
    let selected = start_center.selection().min(rows.len().saturating_sub(1));

    let (_, label_h) = ui_primary_ascent_and_height(text_runtime, 14.0);
    let (_, detail_h) = ui_primary_ascent_and_height(text_runtime, 12.0);
    let text_block_h = label_h.saturating_add(detail_h);
    let row_height = style
        .density_row_height
        .saturating_mul(2)
        .max(text_block_h.saturating_add(style.space_2.saturating_mul(2)));
    let row_gap = style.density_gutter;
    let row_width = card.width.saturating_sub(content_padding.saturating_mul(2));
    for (idx, row) in rows.iter().enumerate() {
        let row_rect = Rect::new(header_x, y, row_width, row_height);
        if row_rect.bottom().saturating_add(content_padding) > card.bottom() {
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
        let label_y = row_rect
            .y
            .saturating_add(row_height.saturating_sub(text_block_h) / 2);
        let label_h = draw_ui_text(
            buffer,
            width,
            height,
            row_rect.x.saturating_add(style.space_2),
            label_y,
            &label,
            label_color,
            14.0,
            scale_bucket,
            text_runtime,
        );

        let mut detail = row.summary.to_string();
        if let Some(preflight) = row.preflight.as_ref() {
            if let Some(reason) = preflight.enablement_snapshot.disabled_reason_code {
                detail.push_str("  — ");
                detail.push_str(reason.as_str());
            }
        }
        let detail_y = label_y.saturating_add(label_h);
        draw_ui_text(
            buffer,
            width,
            height,
            row_rect.x.saturating_add(style.space_2),
            detail_y,
            &detail,
            style.tokens.text_muted,
            12.0,
            scale_bucket,
            text_runtime,
        );

        y = y.saturating_add(row_height).saturating_add(row_gap);
    }

    if let Ok(bundle_rows) = crate::start_center::build_alpha_bundle_gallery_rows() {
        if !bundle_rows.is_empty()
            && y.saturating_add(row_height.saturating_mul(2))
                < card.bottom().saturating_sub(content_padding)
        {
            let heading_h = draw_ui_text(
                buffer,
                width,
                height,
                header_x,
                y,
                "Launch bundles",
                style.tokens.text_secondary,
                13.0,
                scale_bucket,
                text_runtime,
            );
            y = y.saturating_add(heading_h).saturating_add(style.space_2);
            let max_chars = (row_width / 7).max(24) as usize;
            for bundle in bundle_rows.iter().take(2) {
                let row_rect = Rect::new(header_x, y, row_width, row_height);
                if row_rect.bottom().saturating_add(content_padding) > card.bottom() {
                    break;
                }
                fill_rect(buffer, width, height, row_rect, style.tokens.bg_surface);
                stroke_rect(
                    buffer,
                    width,
                    height,
                    row_rect,
                    style.stroke_default,
                    style.tokens.border_default,
                );
                let label = truncate_chars(
                    &format!("{}   [{}]", bundle.persona_or_stack_label, bundle.channel),
                    max_chars,
                );
                let detail = truncate_chars(
                    &format!(
                        "{} | {} | {}",
                        bundle.bundle_id,
                        bundle.certification_state,
                        bundle.mirror_availability_label
                    ),
                    max_chars,
                );
                let label_y = row_rect
                    .y
                    .saturating_add(row_height.saturating_sub(text_block_h) / 2);
                let label_h = draw_ui_text(
                    buffer,
                    width,
                    height,
                    row_rect.x.saturating_add(style.space_2),
                    label_y,
                    &label,
                    style.tokens.text_secondary,
                    14.0,
                    scale_bucket,
                    text_runtime,
                );
                draw_ui_text(
                    buffer,
                    width,
                    height,
                    row_rect.x.saturating_add(style.space_2),
                    label_y.saturating_add(label_h),
                    &detail,
                    style.tokens.text_muted,
                    12.0,
                    scale_bucket,
                    text_runtime,
                );
                y = y.saturating_add(row_height).saturating_add(row_gap);
            }
        }
    }

    if y.saturating_add(22) < card.bottom() {
        draw_ui_text(
            buffer,
            width,
            height,
            header_x,
            card.bottom()
                .saturating_sub(style.space_3)
                .saturating_sub(12),
            "↑/↓ select • Enter run • Cmd/Ctrl+Shift+P palette • Cmd/Ctrl+Shift+M density • Cmd/Ctrl+Alt+Shift+M motion",
            style.tokens.text_muted,
            12.0,
            scale_bucket,
            text_runtime,
        );
    }

    if card.height > style.space_6.saturating_mul(4) {
        let accent = Rect::new(card.x, card.y, style.stroke_focus.max(2), card.height);
        fill_rect(buffer, width, height, accent, style.tokens.accent_brand);
    }
}

fn truncate_chars(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.to_string();
    }
    if max_chars == 0 {
        return String::new();
    }
    let mut clipped = input
        .chars()
        .take(max_chars.saturating_sub(1))
        .collect::<String>();
    clipped.push('~');
    clipped
}

fn draw_docs_help_boundary_card(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    rect: Rect,
    card: &EmbeddedBoundaryCardRecord,
    keybinding_runtime: &KeybindingRuntimeState,
    style: &ShellRenderStyle,
    focused: bool,
) {
    let padding = style.density_zone_inset;
    let panel = Rect::new(
        rect.x.saturating_add(padding),
        rect.y.saturating_add(padding),
        rect.width.saturating_sub(padding.saturating_mul(2)),
        rect.height.saturating_sub(padding.saturating_mul(2)),
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
        style.tokens.border_default,
    );
    if focused {
        let ring = style.component_states.focus_ring_style();
        stroke_rect(buffer, width, height, panel, ring.stroke_px, ring.color);
    }

    let content_padding = style.density_panel_padding;
    let header_x = panel.x.saturating_add(content_padding);
    let mut y = panel.y.saturating_add(content_padding);
    draw_text(
        buffer,
        width,
        height,
        header_x,
        y,
        2,
        "Embedded docs/help",
        style.tokens.text_primary,
    );
    y = y
        .saturating_add(8u32.saturating_mul(2))
        .saturating_add(style.density_gutter);

    let shortcuts = keybinding_runtime.shortcuts_label("cmd:docs.open_in_browser");

    let docs_browser_surface =
        crate::docs_browser::DocsBrowserSurfaceState::from_boundary_card(card.clone());
    let row_card = docs_browser_surface.render_row_card();
    let packet_ref = row_card
        .browser_handoff_row
        .browser_handoff_packet_ref
        .as_deref()
        .unwrap_or("missing");
    let action_label = row_card.browser_handoff_row.action_label.as_str();

    let mut lines = vec![
        format!("Owner: {}", row_card.owner_label),
        format!("Publisher: {}", row_card.publisher_or_service_label),
        format!(
            "Origin: {} ({})",
            row_card.origin_label, row_card.host_or_domain_label
        ),
        format!(
            "Boundary: {}",
            row_card.client_scope_row.data_boundary_label
        ),
        format!("State: {}", row_card.client_scope_row.boundary_state_label),
        format!("Permission: {}", card.permission_state.permission_label),
        format!("Source: {}", row_card.source_row.label),
    ];
    if let Some(age) = row_card.source_row.snapshot_age_label.as_deref() {
        lines.push(format!("Snapshot age: {age}"));
    }
    lines.push(format!("Version: {}", row_card.version_row.label));
    lines.push(format!(
        "Build: {}",
        row_card.version_row.running_build_identity_ref
    ));
    lines.push(format!("Freshness: {}", row_card.freshness_row.label));
    lines.push(format!(
        "Client scope: identity {}, trust {}",
        row_card.client_scope_row.identity_mode_token, row_card.client_scope_row.trust_state_token
    ));
    lines.push(format!("Action: {}  [{}]", action_label, shortcuts));
    lines.push(format!("Handoff packet: {}", packet_ref));

    let line_h = style.density_row_height;
    let text_scale = 2u32;
    let glyph_h = 8u32.saturating_mul(text_scale);
    for line in lines {
        if y.saturating_add(line_h) > panel.bottom().saturating_sub(content_padding) {
            break;
        }
        let text_y = y.saturating_add(line_h.saturating_sub(glyph_h) / 2);
        draw_text(
            buffer,
            width,
            height,
            header_x,
            text_y,
            text_scale,
            &line,
            style.tokens.text_secondary,
        );
        y = y.saturating_add(line_h);
    }

    if y.saturating_add(12) <= panel.bottom() {
        draw_text(
            buffer,
            width,
            height,
            header_x,
            panel
                .bottom()
                .saturating_sub(content_padding)
                .saturating_sub(10),
            1,
            "Chrome is host-owned; high-risk approval stays native.",
            style.tokens.text_muted,
        );
    }

    if panel.height > style.space_6.saturating_mul(4) {
        let accent = Rect::new(panel.x, panel.y, style.stroke_focus.max(2), panel.height);
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
    token_registry: &TokenRegistry,
    reduced_motion_posture: AccessibilityPostureClass,
    now: Instant,
    registry: &CommandRegistry,
    frame: &DesktopFrame,
    editor_runtime: &EditorWorkspaceRuntimeState,
    palette: &CommandPaletteState,
    keybinding_runtime: &KeybindingRuntimeState,
    enablement_runtime: &CommandEnablementRuntimeState,
    docs_help_boundary_card: &EmbeddedBoundaryCardRecord,
    style: &ShellRenderStyle,
    held_modifiers: &HeldModifiers,
) {
    let Some(overlay_logical) = frame.layout().zone(ShellZoneId::TransientOverlay) else {
        return;
    };
    let overlay_physical = to_physical_rect(overlay_logical, scale_factor);
    let zone_inset_logical = to_logical_px(style.density_zone_inset, scale_factor);
    let slots = frame.slot_rects_within_zone(
        ShellZoneId::TransientOverlay,
        overlay_logical,
        zone_inset_logical,
    );
    let slot = slots
        .iter()
        .find(|(id, _)| *id == "slot.overlay.command_palette")
        .map(|(_, rect)| *rect)
        .unwrap_or(overlay_logical);
    let slot_physical = to_physical_rect(slot, scale_factor);

    let (enter_progress, substitution, _) = overlay_dialog_enter_progress(
        token_registry,
        reduced_motion_posture,
        palette.opened_at(),
        now,
    );
    let fade = |color: ColorRgba| scale_alpha(color, enter_progress);

    // Dim the entire window.
    fill_rect(
        buffer,
        width,
        height,
        overlay_physical,
        fade(style.tokens.bg_overlay),
    );

    // Panel inside the slot.
    let panel_padding = style.density_zone_inset;
    let mut panel = Rect::new(
        slot_physical.x.saturating_add(panel_padding),
        slot_physical.y.saturating_add(panel_padding),
        slot_physical.width.saturating_sub(panel_padding * 2),
        slot_physical.height.saturating_sub(panel_padding * 2),
    );
    if substitution.is_none() && enter_progress < 1.0 {
        let offset = (style.space_3 as f32 * (1.0 - enter_progress))
            .round()
            .max(0.0) as u32;
        panel.y = panel.y.saturating_add(offset);
    }
    if panel.is_empty() {
        return;
    }

    fill_rect(buffer, width, height, panel, fade(style.tokens.bg_raised));
    stroke_rect(
        buffer,
        width,
        height,
        panel,
        style.stroke_default,
        fade(style.tokens.border_strong),
    );

    let text_scale = 2u32;
    let glyph_h = 8u32.saturating_mul(text_scale);
    let line_h = (8u32.saturating_mul(text_scale)).saturating_add(style.space_2);
    let inner_padding = style.density_panel_padding;
    let mut cursor_y = panel.y.saturating_add(inner_padding);
    let cursor_x = panel.x.saturating_add(inner_padding);

    draw_text(
        buffer,
        width,
        height,
        cursor_x,
        cursor_y,
        text_scale,
        "Command Palette (Cmd/Ctrl+Shift+P)",
        fade(style.tokens.text_primary),
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
        fade(style.tokens.text_secondary),
    );
    cursor_y = cursor_y
        .saturating_add(line_h)
        .saturating_add(style.space_2 / 2);

    let query_rect = Rect::new(
        panel.x.saturating_add(inner_padding),
        cursor_y,
        panel.width.saturating_sub(inner_padding.saturating_mul(2)),
        style.density_control_height,
    );
    if !query_rect.is_empty() {
        let chrome = style.component_states.chrome_style(
            ComponentSurfaceTone::Surface,
            ComponentStates::FOCUS_VISIBLE,
        );
        fill_rect(buffer, width, height, query_rect, fade(chrome.fill));
        stroke_rect(
            buffer,
            width,
            height,
            query_rect,
            chrome.border_stroke_px,
            fade(chrome.border),
        );
        if let Some(ring) = chrome.focus_ring {
            stroke_rect(
                buffer,
                width,
                height,
                query_rect,
                ring.stroke_px,
                fade(ring.color),
            );
        }

        let mut composed_query = String::new();
        composed_query.push_str(palette.query());
        if let Some(composition) = palette.ime_composition() {
            composed_query.push_str(&composition.text);
        }

        let query_label = if composed_query.is_empty() {
            "> Type a command id or file path…".to_string()
        } else {
            format!("> {}", composed_query)
        };
        let query_text_y = query_rect
            .y
            .saturating_add(query_rect.height.saturating_sub(glyph_h) / 2);
        draw_text(
            buffer,
            width,
            height,
            query_rect.x.saturating_add(style.space_2),
            query_text_y,
            text_scale,
            &query_label,
            fade(style.tokens.text_primary),
        );
        cursor_y = query_rect.bottom().saturating_add(style.density_gutter);
    }

    let preview_runtime = PalettePreviewRuntimeInputs {
        client_scope: "desktop_product",
        workspace_trust_state: enablement_runtime.workspace_trust_state.as_str(),
        execution_context_available: enablement_runtime.execution_context_available,
        provider_linked: enablement_runtime.provider_linked,
        credential_available: enablement_runtime.credential_available,
        policy_disabled: enablement_runtime.policy_disabled,
        policy_blocked_in_context: enablement_runtime.policy_blocked_in_context,
        labs_enabled: enablement_runtime.labs_enabled,
    };
    let preview_arguments = palette
        .selected_entry(registry)
        .map(|entry| argument_provenance_map_for_shell(entry, frame, editor_runtime));
    let preview = materialize_palette_preview_record_with_arguments(
        palette.selected_key(),
        registry,
        &keybinding_runtime.shortcuts_by_command_id,
        preview_runtime,
        preview_arguments,
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
                labs_enabled: enablement_runtime.labs_enabled,
                argument_provenance_map: argument_provenance_map_for_shell(
                    entry,
                    frame,
                    editor_runtime,
                ),
            };
            let snapshot = entry.evaluate_enablement(&enablement_context);
            (snapshot.decision_class, snapshot.disabled_reason_code)
        },
    );

    let footer_lines = 2u32;
    let footer_height = footer_lines
        .saturating_mul(line_h)
        .saturating_add(style.density_gutter);
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

    let gap = style.density_gutter;
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
    let row_height = style.density_row_height.max(glyph_h);
    for row in view_rows.iter() {
        if list_y.saturating_add(row_height) > list_rect.bottom() {
            break;
        }
        let selected = row
            .key
            .as_ref()
            .and_then(|key| selected_key.as_ref().map(|s| (key, s)))
            .map(|(k, s)| k == s)
            .unwrap_or(false);
        if selected && !row.is_group_header {
            let highlight = Rect::new(list_rect.x, list_y, list_rect.width, row_height);
            let chrome = style
                .component_states
                .chrome_style(ComponentSurfaceTone::Surface, ComponentStates::SELECTED);
            fill_rect(buffer, width, height, highlight, chrome.fill);
            stroke_rect(
                buffer,
                width,
                height,
                highlight,
                chrome.border_stroke_px,
                chrome.border,
            );
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
            list_y.saturating_add(row_height.saturating_sub(glyph_h) / 2),
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
        list_y = list_y.saturating_add(row_height);
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
                if command.command_id == "cmd:docs.open_in_browser" {
                    let packet_ref = docs_help_boundary_card
                        .open_in_browser_action()
                        .and_then(|row| row.browser_handoff_packet_ref.as_deref())
                        .unwrap_or("missing");
                    preview_lines.push("Boundary chrome:".to_string());
                    preview_lines.push(format!(
                        "Owner: {}",
                        docs_help_boundary_card.owner_identity.label
                    ));
                    preview_lines.push(format!(
                        "Origin: {}",
                        docs_help_boundary_card.origin_identity.host_or_domain_label
                    ));
                    preview_lines.push(format!(
                        "State: {}",
                        docs_help_boundary_card.boundary_state_label
                    ));
                    preview_lines.push(format!("Handoff packet: {packet_ref}"));
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

fn draw_ui_text(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    x: u32,
    y_top: u32,
    text: &str,
    color: ColorRgba,
    font_size_px: f32,
    scale_bucket: u8,
    text_runtime: &mut ShellTextRuntime,
) -> u32 {
    let (_, line_height) = ui_primary_ascent_and_height(text_runtime, font_size_px);
    if draw_shaped_text(
        buffer,
        width,
        height,
        x,
        y_top,
        text,
        color,
        font_size_px,
        scale_bucket,
        text_runtime,
    ) {
        return line_height;
    }

    let fallback_scale = ((font_size_px / 8.0).round() as u32).clamp(1, 4);
    draw_text(buffer, width, height, x, y_top, fallback_scale, text, color);
    8u32.saturating_mul(fallback_scale)
}

fn ui_primary_ascent_and_height(
    text_runtime: &mut ShellTextRuntime,
    font_size_px: f32,
) -> (f32, u32) {
    if font_size_px <= 0.0 || !font_size_px.is_finite() {
        return (0.0, 0);
    }

    let font_id = text_runtime
        .font_system
        .resolve_system_ui_face(text_runtime.ui_fallback.system_ui_family)
        .or_else(|| {
            text_runtime
                .font_system
                .database()
                .faces()
                .next()
                .map(|face| face.id)
        });
    let Some(font_id) = font_id else {
        return (font_size_px, font_size_px.ceil().max(1.0) as u32);
    };
    let Some(font) = text_runtime.font_system.swash_font(font_id) else {
        return (font_size_px, font_size_px.ceil().max(1.0) as u32);
    };

    let metrics = font.metrics(&[]).scale(font_size_px);
    let ascent = metrics.ascent.max(0.0);
    let raw_height = (metrics.ascent - metrics.descent + metrics.leading).max(font_size_px);
    let height_px = raw_height.ceil().max(1.0) as u32;
    (ascent, height_px)
}

fn draw_shaped_text(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    x: u32,
    y_top: u32,
    text: &str,
    color: ColorRgba,
    font_size_px: f32,
    scale_bucket: u8,
    text_runtime: &mut ShellTextRuntime,
) -> bool {
    if text.is_empty() {
        return true;
    }
    if text_runtime.font_system.face_count() == 0 {
        return false;
    }

    let (ascent, _) = ui_primary_ascent_and_height(text_runtime, font_size_px);
    let baseline_x = x as f32;
    let baseline_y = y_top as f32 + ascent;

    let shaped = text_runtime.shaper.shape_line(
        &mut text_runtime.font_system,
        text,
        font_size_px,
        &text_runtime.ui_fallback,
        text_runtime.ui_features,
    );
    if shaped.glyphs.is_empty() {
        return false;
    }

    let px_size_q8 = ((font_size_px.max(0.01) * 256.0).round() as u32).max(1);
    let font_system = &mut text_runtime.font_system;
    let atlas = &mut text_runtime.atlas;

    for glyph in shaped.glyphs {
        let entry = atlas.get_or_rasterize(
            font_system,
            GlyphKey {
                glyph_id: glyph.glyph_id,
                font_id: glyph.font_id,
                px_size_q8,
                subpixel_variant: 0,
                scale_bucket,
            },
        );
        let Some(entry) = entry else {
            continue;
        };

        let placement = entry.image.placement;
        if placement.width == 0 || placement.height == 0 {
            continue;
        }

        let glyph_x = baseline_x + glyph.x;
        let glyph_y = baseline_y + glyph.y;
        let dst_x = (glyph_x + placement.left as f32).round() as i32;
        let dst_y = (glyph_y - placement.top as f32).round() as i32;

        let expected_mask = (placement.width as usize).saturating_mul(placement.height as usize);
        if entry.image.data.len() == expected_mask {
            blend_alpha_mask(
                buffer,
                width,
                height,
                dst_x,
                dst_y,
                placement.width,
                placement.height,
                &entry.image.data,
                color,
            );
        } else if entry.image.data.len() == expected_mask.saturating_mul(4) {
            blend_rgba_image(
                buffer,
                width,
                height,
                dst_x,
                dst_y,
                placement.width,
                placement.height,
                &entry.image.data,
            );
        }
    }

    true
}

fn blend_alpha_mask(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    dst_x: i32,
    dst_y: i32,
    mask_width: u32,
    mask_height: u32,
    mask: &[u8],
    color: ColorRgba,
) {
    if width == 0 || height == 0 || mask_width == 0 || mask_height == 0 {
        return;
    }

    let src_width = mask_width as usize;
    let src_height = mask_height as usize;
    let dst_width = width as usize;
    let dst_height = height as usize;

    let src_x0 = if dst_x < 0 { (-dst_x) as usize } else { 0 };
    let src_y0 = if dst_y < 0 { (-dst_y) as usize } else { 0 };
    if src_x0 >= src_width || src_y0 >= src_height {
        return;
    }

    let dst_x0 = if dst_x < 0 { 0 } else { dst_x as usize };
    let dst_y0 = if dst_y < 0 { 0 } else { dst_y as usize };
    if dst_x0 >= dst_width || dst_y0 >= dst_height {
        return;
    }

    let src_end_x = src_width.min(dst_width.saturating_sub(dst_x0).saturating_add(src_x0));
    let src_end_y = src_height.min(dst_height.saturating_sub(dst_y0).saturating_add(src_y0));

    let (clip_x0, clip_y0, clip_x1, clip_y1) = raster_clip()
        .map(|clip| {
            (
                clip.x.min(width) as usize,
                clip.y.min(height) as usize,
                clip.right().min(width) as usize,
                clip.bottom().min(height) as usize,
            )
        })
        .unwrap_or((0, 0, dst_width, dst_height));

    let mut dy = dst_y0;
    for sy in src_y0..src_end_y {
        if dy < clip_y0 {
            dy = dy.saturating_add(1);
            continue;
        }
        if dy >= clip_y1 {
            break;
        }
        let src_row = &mask[sy.saturating_mul(src_width)..];
        let dst_row = dy.saturating_mul(dst_width);
        dy = dy.saturating_add(1);
        let mut dx = dst_x0;
        for sx in src_x0..src_end_x {
            if dx < clip_x0 {
                dx = dx.saturating_add(1);
                continue;
            }
            if dx >= clip_x1 {
                break;
            }
            let a = src_row.get(sx).copied().unwrap_or(0);
            if a == 0 {
                dx = dx.saturating_add(1);
                continue;
            }
            let alpha = (u16::from(a).saturating_mul(u16::from(color.a)) / 255) as u8;
            let tinted = ColorRgba {
                r: color.r,
                g: color.g,
                b: color.b,
                a: alpha,
            };
            if let Some(px) = buffer.get_mut(dst_row.saturating_add(dx)) {
                *px = tinted.blend_over_u32(*px);
            }
            dx = dx.saturating_add(1);
        }
    }
}

fn blend_rgba_image(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    dst_x: i32,
    dst_y: i32,
    image_width: u32,
    image_height: u32,
    image: &[u8],
) {
    if width == 0 || height == 0 || image_width == 0 || image_height == 0 {
        return;
    }

    let src_width = image_width as usize;
    let src_height = image_height as usize;
    let dst_width = width as usize;
    let dst_height = height as usize;

    let src_x0 = if dst_x < 0 { (-dst_x) as usize } else { 0 };
    let src_y0 = if dst_y < 0 { (-dst_y) as usize } else { 0 };
    if src_x0 >= src_width || src_y0 >= src_height {
        return;
    }

    let dst_x0 = if dst_x < 0 { 0 } else { dst_x as usize };
    let dst_y0 = if dst_y < 0 { 0 } else { dst_y as usize };
    if dst_x0 >= dst_width || dst_y0 >= dst_height {
        return;
    }

    let src_end_x = src_width.min(dst_width.saturating_sub(dst_x0).saturating_add(src_x0));
    let src_end_y = src_height.min(dst_height.saturating_sub(dst_y0).saturating_add(src_y0));

    let (clip_x0, clip_y0, clip_x1, clip_y1) = raster_clip()
        .map(|clip| {
            (
                clip.x.min(width) as usize,
                clip.y.min(height) as usize,
                clip.right().min(width) as usize,
                clip.bottom().min(height) as usize,
            )
        })
        .unwrap_or((0, 0, dst_width, dst_height));

    let mut dy = dst_y0;
    for sy in src_y0..src_end_y {
        if dy < clip_y0 {
            dy = dy.saturating_add(1);
            continue;
        }
        if dy >= clip_y1 {
            break;
        }
        let dst_row = dy.saturating_mul(dst_width);
        dy = dy.saturating_add(1);
        let mut dx = dst_x0;
        for sx in src_x0..src_end_x {
            if dx < clip_x0 {
                dx = dx.saturating_add(1);
                continue;
            }
            if dx >= clip_x1 {
                break;
            }
            let base = (sy.saturating_mul(src_width).saturating_add(sx)).saturating_mul(4);
            let Some(chunk) = image.get(base..base.saturating_add(4)) else {
                dx = dx.saturating_add(1);
                continue;
            };
            let color = ColorRgba {
                r: chunk[0],
                g: chunk[1],
                b: chunk[2],
                a: chunk[3],
            };
            if color.a == 0 {
                dx = dx.saturating_add(1);
                continue;
            }
            if let Some(px) = buffer.get_mut(dst_row.saturating_add(dx)) {
                *px = color.blend_over_u32(*px);
            }
            dx = dx.saturating_add(1);
        }
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

fn draw_text_clamped(
    buffer: &mut [u32],
    width: u32,
    height: u32,
    mut x: u32,
    y: u32,
    scale: u32,
    text: &str,
    color: ColorRgba,
    max_x: u32,
) {
    let char_w = 8u32.saturating_mul(scale);
    if char_w == 0 || max_x <= x {
        return;
    }
    for ch in text.chars() {
        if x.saturating_add(char_w) > max_x {
            break;
        }
        draw_glyph(buffer, width, height, x, y, scale, ch, color);
        x = x.saturating_add(char_w);
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
        KeyCode::Backquote => Some("`".to_string()),
        KeyCode::Comma => Some(",".to_string()),
        KeyCode::F3 => Some("F3".to_string()),
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
    let mut x0 = x;
    let mut y0 = y;
    let mut x1 = x.saturating_add(scale).min(max_x.saturating_add(1));
    let mut y1 = y.saturating_add(scale).min(max_y.saturating_add(1));

    if let Some(clip) = raster_clip() {
        let clip_x0 = clip.x.min(width);
        let clip_y0 = clip.y.min(height);
        let clip_x1 = clip.right().min(width);
        let clip_y1 = clip.bottom().min(height);
        x0 = x0.max(clip_x0);
        y0 = y0.max(clip_y0);
        x1 = x1.min(clip_x1);
        y1 = y1.min(clip_y1);
        if x0 >= x1 || y0 >= y1 {
            return;
        }
    }

    for yy in y0..y1 {
        let row = (yy as usize).saturating_mul(width as usize);
        for xx in x0..x1 {
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

#[cfg(test)]
mod notification_surface_tests {
    use super::*;

    #[test]
    fn save_success_routes_to_transient_toast_and_durable_row() {
        let mut runtime =
            ActivityCenterRuntimeState::in_memory_with_quiet_hours(QuietHoursPosture::none());
        runtime.note_save_running("save-test-success", "demo.rs");
        runtime.note_save_completed("save-test-success", "demo.rs", Some("packet:save".into()));

        assert_eq!(runtime.notifications.toasts.len(), 1);
        assert!(runtime.notifications.banners.is_empty());
        assert_eq!(
            runtime.notifications.toasts[0].row.summary_label,
            "Save completed: demo.rs"
        );

        let snapshot = runtime.snapshot();
        let row = snapshot
            .find("ux:event:save-fsync:save-test-success")
            .expect("save row should persist");
        assert_eq!(row.lifecycle_class, ActivityRowLifecycleClass::Completed);
        assert_eq!(row.severity_class, SeverityClass::Success);
    }

    #[test]
    fn save_failure_routes_to_persistent_banner() {
        let mut runtime =
            ActivityCenterRuntimeState::in_memory_with_quiet_hours(QuietHoursPosture::none());
        runtime.note_save_running("save-test-failure", "readonly.rs");
        runtime.note_save_failed("save-test-failure", "readonly.rs", "permission denied");

        assert!(runtime.notifications.toasts.is_empty());
        assert_eq!(runtime.notifications.banners.len(), 1);
        assert_eq!(
            runtime.notifications.banners[0].row.summary_label,
            "Save failed: readonly.rs"
        );

        let later = Instant::now() + Duration::from_secs(30);
        assert!(!runtime.tick_notifications(later));
        assert_eq!(runtime.notifications.banners.len(), 1);
    }

    #[test]
    fn quiet_hours_holds_toast_but_keeps_activity_row() {
        let mut runtime = ActivityCenterRuntimeState::in_memory_with_quiet_hours(
            QuietHoursPosture::quiet_hours_user(),
        );
        runtime.note_save_running("save-test-held", "quiet.rs");
        runtime.note_save_completed("save-test-held", "quiet.rs", None);

        assert!(runtime.notifications.toasts.is_empty());
        assert!(runtime.notifications.banners.is_empty());

        let snapshot = runtime.snapshot();
        let row = snapshot
            .find("ux:event:save-fsync:save-test-held")
            .expect("held toast should still have a durable row");
        assert_eq!(row.lifecycle_class, ActivityRowLifecycleClass::Completed);
        assert_eq!(row.summary_label, "Save completed: quiet.rs");
    }

    #[test]
    fn clicking_banner_dismisses_it() {
        let mut runtime =
            ActivityCenterRuntimeState::in_memory_with_quiet_hours(QuietHoursPosture::none());
        runtime.note_trust_state_changed("trusted", "restricted");
        assert_eq!(runtime.notifications.banners.len(), 1);

        let frame = DesktopFrame::new(1280, 720);
        let rect = notification_banner_rect(&frame, 1.0).expect("banner rect");
        assert!(runtime.dismiss_notification_at(&frame, 1.0, rect.x + 1, rect.y + 1));
        assert!(runtime.notifications.banners.is_empty());
    }

    #[test]
    fn clicking_toast_dismisses_it() {
        let mut runtime =
            ActivityCenterRuntimeState::in_memory_with_quiet_hours(QuietHoursPosture::none());
        runtime.note_save_running("save-test-click-toast", "toast.rs");
        runtime.note_save_completed("save-test-click-toast", "toast.rs", None);
        assert_eq!(runtime.notifications.toasts.len(), 1);

        let frame = DesktopFrame::new(1280, 720);
        let rects = notification_toast_rects(&frame, 1.0, runtime.notifications.toasts.len());
        let rect = rects.first().copied().expect("toast rect");
        assert!(runtime.dismiss_notification_at(&frame, 1.0, rect.x + 1, rect.y + 1));
        assert!(runtime.notifications.toasts.is_empty());
    }
}

#[cfg(test)]
mod status_bar_shell_tests {
    use super::*;

    #[test]
    fn status_bar_painted_slots_have_projection_or_documented_fallback() {
        let frame = DesktopFrame::new(1280, 720);

        for slot_id in frame.slot_ids_for_zone(ShellZoneId::StatusBar) {
            assert!(
                status_bar_slot_source(slot_id).is_some()
                    || status_bar_slot_has_documented_fallback(slot_id),
                "status slot {slot_id} must map to typed projection data or documented fallback"
            );
        }
    }

    #[test]
    fn trust_slot_routes_to_trust_review_activation() {
        assert_eq!(
            status_bar_slot_activation("status.slot.context.workspace"),
            StatusBarSlotActivation::TrustReview
        );
    }

    #[test]
    fn target_slot_routes_to_workspace_switcher_activation() {
        assert_eq!(
            status_bar_slot_activation("status.slot.context.execution"),
            StatusBarSlotActivation::WorkspaceSwitcher
        );
    }
}

#[cfg(test)]
mod tab_case_tests {
    use super::*;

    use std::fs;
    use std::path::PathBuf;

    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Meta {
        name: String,
        scenario: String,
    }

    #[derive(Debug, Deserialize)]
    struct DocumentFixture {
        text: String,
        #[serde(default)]
        read_only: bool,
    }

    #[derive(Debug, Deserialize)]
    struct EditFixture {
        offset: usize,
        insert: String,
        expected_text: String,
        #[serde(default = "default_true")]
        save_should_succeed: bool,
        #[serde(default)]
        expected_on_disk: Option<String>,
    }

    fn default_true() -> bool {
        true
    }

    #[derive(Debug, Deserialize)]
    struct TabCaseFixture {
        #[serde(rename = "__fixture__")]
        meta: Meta,
        document: DocumentFixture,
        edit: EditFixture,
    }

    #[test]
    fn tab_case_fixtures_preserve_shared_buffer_authority() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let fixtures_dir = repo_root.join("fixtures/editor/tab_cases");

        let mut fixture_paths: Vec<PathBuf> = fs::read_dir(&fixtures_dir)
            .expect("fixture directory must exist")
            .filter_map(|entry| entry.ok())
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
            .collect();
        fixture_paths.sort();

        assert!(
            !fixture_paths.is_empty(),
            "expected at least one fixture under {fixtures_dir:?}"
        );

        for fixture_path in fixture_paths {
            let raw = fs::read_to_string(&fixture_path).expect("fixture should be readable");
            let fixture: TabCaseFixture =
                serde_json::from_str(&raw).expect("fixture should be valid JSON");

            let tmp_dir = std::env::temp_dir();
            let suffix = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos();
            let tmp_path = tmp_dir.join(format!(
                "aureline_tab_case_{}_{}.txt",
                fixture.meta.name, suffix
            ));

            fs::write(&tmp_path, fixture.document.text.as_bytes()).expect("write temp file");
            if fixture.document.read_only {
                let mut perms = fs::metadata(&tmp_path)
                    .expect("read temp metadata")
                    .permissions();
                perms.set_readonly(true);
                fs::set_permissions(&tmp_path, perms).expect("set read-only permissions");
            }

            let mut frame = DesktopFrame::new(1280, 720);
            let group = frame.focused_editor_group();
            let tab1 = frame.open_tab().expect("open first tab");
            let tab2 = frame.open_tab().expect("open second tab");

            let mut editor_runtime = EditorWorkspaceRuntimeState::new();
            editor_runtime
                .open_file(group, tab1, &tmp_path)
                .expect("open file in tab1");
            editor_runtime
                .open_file(group, tab2, &tmp_path)
                .expect("open file in tab2");

            let auth1 = {
                editor_runtime
                    .tab_session_mut(group, tab1)
                    .expect("tab1 session")
                    .authority
                    .clone()
            };
            let auth2 = {
                editor_runtime
                    .tab_session_mut(group, tab2)
                    .expect("tab2 session")
                    .authority
                    .clone()
            };
            assert!(
                Rc::ptr_eq(&auth1, &auth2),
                "expected tab sessions to share one buffer authority ({})",
                fixture.meta.scenario
            );

            let viewport_rect = PixelRect::new(0, 0, 800, 600);
            {
                let session = editor_runtime
                    .tab_session_mut(group, tab1)
                    .expect("tab1 session");
                let (line, grapheme) = session
                    .snapshot
                    .line_grapheme_for_byte_offset(fixture.edit.offset)
                    .expect("offset should map into document");
                session
                    .viewport
                    .set_caret(aureline_editor::TextPoint { line, grapheme });
                session.viewport.clear_selection();
                let _ = session.apply_action(
                    &EditorAction::InsertText {
                        text: fixture.edit.insert.clone(),
                    },
                    viewport_rect,
                );
            }

            let info1 = editor_runtime
                .tab_render_info(group, tab1)
                .expect("tab1 render info");
            let info2 = editor_runtime
                .tab_render_info(group, tab2)
                .expect("tab2 render info");

            assert!(info1.dirty, "expected tab1 to be Modified");
            assert!(info2.dirty, "expected tab2 to be Modified");
            if fixture.document.read_only {
                assert_eq!(info1.read_only, ReadOnlyState::Filesystem);
                assert_eq!(info2.read_only, ReadOnlyState::Filesystem);
            } else {
                assert_eq!(info1.read_only, ReadOnlyState::Writable);
                assert_eq!(info2.read_only, ReadOnlyState::Writable);
            }

            {
                let session = editor_runtime
                    .tab_session_mut(group, tab2)
                    .expect("tab2 session");
                session.ensure_fresh_snapshot();
                assert_eq!(
                    session.snapshot.as_str().unwrap_or_default(),
                    fixture.edit.expected_text.as_str(),
                    "expected shared buffer content to be visible in both views"
                );
            }

            let save = editor_runtime.save_tab(group, tab1);
            if fixture.edit.save_should_succeed {
                save.expect("save should succeed");
            } else {
                assert!(
                    save.is_err(),
                    "expected save failure ({})",
                    fixture.meta.scenario
                );
            }

            let expected_on_disk = if fixture.edit.save_should_succeed {
                fixture.edit.expected_text.clone()
            } else {
                fixture
                    .edit
                    .expected_on_disk
                    .clone()
                    .unwrap_or_else(|| fixture.document.text.clone())
            };
            let on_disk = fs::read_to_string(&tmp_path).expect("read temp file");
            assert_eq!(on_disk, expected_on_disk);

            let info1 = editor_runtime
                .tab_render_info(group, tab1)
                .expect("tab1 render info");
            let info2 = editor_runtime
                .tab_render_info(group, tab2)
                .expect("tab2 render info");
            if fixture.edit.save_should_succeed {
                assert!(!info1.dirty, "expected tab1 to be clean after save");
                assert!(!info2.dirty, "expected tab2 to be clean after save");
            } else {
                assert!(
                    info1.dirty,
                    "expected tab1 to remain Modified after save failure"
                );
                assert!(
                    info2.dirty,
                    "expected tab2 to remain Modified after save failure"
                );
            }

            if fixture.document.read_only {
                let mut perms = fs::metadata(&tmp_path)
                    .expect("read temp metadata")
                    .permissions();
                perms.set_readonly(false);
                let _ = fs::set_permissions(&tmp_path, perms);
            }
            let _ = fs::remove_file(&tmp_path);
        }
    }
}

#[cfg(test)]
mod continuity_tests;
