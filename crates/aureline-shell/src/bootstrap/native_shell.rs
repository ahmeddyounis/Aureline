//! Native desktop shell bootstrap and event-loop wiring.
//!
//! Owns the canonical native window bootstrap, input dispatch root, and
//! startup-milestone emission for the desktop shell.

use std::borrow::Cow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::num::NonZeroU32;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::a11y::shell_bridge::{
    materialize_shell_accessibility_tree, write_shell_accessibility_tree_log,
    ShellA11yEnablementContext,
};
use crate::app_frame::desktop_frame::{DesktopFrame, EditorTabId, NewEditorGroupOutcome, SplitViolation};
use crate::bootstrap::startup_trace::{StartupMilestone, StartupTrace, StartupTraceConfig};
use crate::commands::diagnostics_sheet::{
    diagnostics_sheet_lines, materialize_command_diagnostics_sheet_record,
    write_diagnostics_sheet_log, CommandDiagnosticsSheetRecord,
};
use crate::commands::invocation_preview::{
    invocation_preview_sheet_lines, materialize_command_invocation_preview_sheet_record,
    write_invocation_preview_sheet_log, CommandInvocationPreviewSheetRecord,
};
use crate::commands::CommandReviewRuntimeInputs;
use crate::embedded::boundary_card::EmbeddedBoundaryCardRecord;
use crate::embedded::docs_help::{resolve_docs_help_handoff_url, seeded_docs_help_boundary_card};
use crate::help::keybinding_inspector::build_inspector_lines;
use crate::layout::split_tree::PaneId;
use crate::layout::zone_registry::{Rect, ShellZoneId};
use crate::palette::preview::{
    argument_provenance_map_for, copy_payload_for, materialize_palette_preview_record,
    write_preview_log, PaletteCopyIntent, PalettePreviewRuntimeInputs, PalettePreviewSelection,
};
use crate::palette::results_view::palette_view_rows;
use crate::palette::{CommandPaletteCommit, CommandPaletteState};
use crate::start_center::{
    build_action_rows as start_center_action_rows, StartCenterRuntimeInputs, StartCenterState,
    START_CENTER_PRESENTATION_LABEL, START_CENTER_PRESENTATION_SUBTITLE,
};
use crate::workspace_switcher::{
    build_switcher_rows, WorkspaceSwitcherRow, WorkspaceSwitcherState,
    WORKSPACE_SWITCHER_PRESENTATION_LABEL,
};
use aureline_buffer::{Buffer, RevisionId, Snapshot};
use aureline_build_info as build_info;
use aureline_commands::invocation::{
    mint_approval_ticket_ref, mint_basis_snapshot_ref, mint_invocation_session_id,
    mint_preview_record_ref, now_rfc3339, AliasUsedBlock, ApprovalPostureBlock,
    ArgumentProvenanceEntry, ArtifactRefEntry, CommandInvocationSession, CommandResultPacketRecord,
    ContextRefsBlock, EnablementDecisionBlock, EvidenceRefEntry, ExportPostureBlock,
    InvocationContextSnapshot, InvocationCreatedArtifactRefEntry, InvocationOutcomeBlock,
    InvocationSessionPacketRecord, NoBypassGuards, ResultBodyBlock, RollbackHandleRefBlock,
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
use aureline_telemetry::hot_path_metrics::{
    HotPathMetrics, HotPathMetricsConfig, HotPathMetricsContext,
};
use aureline_telemetry::trace_event::BuildIdentityRecord as TelemetryBuildIdentityRecord;
use aureline_ui::components::{
    ComponentStateRegistry, ComponentStates, ComponentSurfaceTone, FocusReturnStack,
};
use aureline_ui::density::DensityProfile;
use aureline_ui::motion::{ReducedMotionSubstitutionClass, OVERLAY_DIALOG_ENTER};
use aureline_ui::themes::{
    AccessibilityPostureClass, AppearanceSessionRecord, DensityClass, LiveFollowSystemPolicyRecord,
    ReducedMotionSource,
};
use aureline_ui::tokens::{
    seeded_token_registry, ColorRgba, ThemeClass, TokenRegistry, TokenRegistryError,
};
use aureline_workspace::{
    PortabilityClass, RecentWorkEntryRecord, RecentWorkEntryRecordKind, RecentWorkRegistry,
    RecentWorkRegistryError, RecentWorkRegistryRecordKind, RecentWorkTargetState,
    RestoreAvailability, SafeRecoveryAction, TargetKind, TrustState,
};
use serde::Serialize;

use crate::bootstrap::appearance_golden::write_png_0rgb;
use crate::windowing::display_safety::{
    materialize_adjustment_record, materialize_topology_record, write_display_safety_log,
    write_display_safety_topology_log, DisplaySafetyGuard,
};
use crate::windowing::winit_softbuffer::{create_softbuffer_surface, SoftbufferSurface};
use crate::windowing::winit_window::WinitWindow;
use arboard::Clipboard;
use aureline_editor::{
    CaretMove, EditorAction, EditorTextRuntime, EditorViewport, SelectionDelta, ViewportCompositor,
    ViewportPaintStyle,
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
use winit::keyboard::{KeyCode, PhysicalKey};

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
    component_states: ComponentStateRegistry,
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
        })
    }
}

#[derive(Debug, Default)]
struct NativeShellArgs {
    startup_trace: StartupTraceConfig,
    hot_path_metrics: HotPathMetricsConfig,
    disable_clipboard: bool,
    renderer: ShellRendererChoice,
    window_size: Option<(f64, f64)>,
    screenshot_path: Option<PathBuf>,
    theme_class: Option<ThemeClass>,
    density_class: Option<DensityClass>,
    reduced_motion_posture: Option<AccessibilityPostureClass>,
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
    let mut iter = env::args().skip(1);
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
            "--exit-after-first-frame" => {
                args.startup_trace.exit_after_first_frame = true;
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
            other => return Err(format!("unknown argument: {other}\n\n{}", usage())),
        }
    }
    Ok(args)
}

fn usage() -> String {
    "aureline_shell — Aureline desktop shell\n\n\
     Usage:\n\
     \taureline_shell\n\
     \taureline_shell --emit-startup-trace <path> [--exit-after-first-frame] [--disable-clipboard]\n\
     \taureline_shell --emit-hot-path-metrics <path>\n\
     \taureline_shell --emit-screenshot <path> [--theme-class <token>] [--density-class <token>] [--reduced-motion-posture <token>] [--window-size <WxH>] [--renderer (gpu|software)]\n\
     \taureline_shell --renderer (gpu|software)\n"
        .to_string()
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
    let window = WinitWindow::new(
        &event_loop,
        window_title(None, None, None),
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
    let mut keybinding_runtime = KeybindingRuntimeState::new(platform_class_for_shell());
    let mut enablement_runtime = CommandEnablementRuntimeState::default();
    let mut recent_work = RecentWorkRuntimeState::load();
    let mut clipboard = ClipboardState::new(!args.disable_clipboard);
    let mut appearance = AppearanceRuntimeState::load();
    appearance.apply_cli_overrides(
        args.theme_class,
        args.density_class,
        args.reduced_motion_posture,
    );
    let docs_help_boundary_card =
        seeded_docs_help_boundary_card(build_info::exact_build_identity_ref());
    let mut text_runtime = ShellTextRuntime::new();
    let mut editor_runtime = EditorWorkspaceRuntimeState::new();
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
    if let Some(err) = appearance.last_error.as_deref() {
        command_runtime.note_non_command_action(format!("appearance session unavailable — {err}"));
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
            if palette.tick(registry, &keybinding_runtime.shortcuts_by_command_id, now) {
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    palette.selected_entry(registry),
                    recent_work.active_workspace_label(),
                ));
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
            let mut next_deadline = palette.next_wake_deadline(now);
            let animation_frame = now + Duration::from_millis(16);
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
                            Ime::Commit(text) => aureline_input::text_input::ImeEvent::Commit { text },
                        },
                        registry,
                        &keybinding_runtime.shortcuts_by_command_id,
                    );

                    if changed {
                        window.set_title(&window_title(
                            Some(frame.focused_zone()),
                            palette.selected_entry(registry),
                            recent_work.active_workspace_label(),
                        ));

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

                let Some(session) = editor_runtime.tab_session_mut(focused, active_tab) else {
                    return;
                };

                let normalized = session.text_input.handle_ime_event(match ime {
                    Ime::Enabled => aureline_input::text_input::ImeEvent::Enabled,
                    Ime::Disabled => aureline_input::text_input::ImeEvent::Disabled,
                    Ime::Preedit(text, cursor) => aureline_input::text_input::ImeEvent::Preedit {
                        text,
                        cursor,
                    },
                    Ime::Commit(text) => aureline_input::text_input::ImeEvent::Commit { text },
                });

                if let Some(normalized) = normalized {
                    let action = editor_action_from_text_input(normalized);
                    if let Some(damage) = session.apply_action(&action, viewport_rect) {
                        if damage.hook == Hook::ReflowLineRange {
                            hot_path_metrics.note_keystroke_to_paint_admitted(clock.now().0);
                        }
                        scheduler.invalidate(damage.event);
                        scheduler.mark_hook(damage.hook, &clock);
                    }
                }

                if should_update_ime_cursor_area {
                    update_ime_cursor_area_for_viewport(&window, &session.viewport, viewport_rect);
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if state != ElementState::Pressed {
                    return;
                }
                if button != winit::event::MouseButton::Left {
                    return;
                }
                if frame.focused_zone() != ShellZoneId::MainWorkspace
                    || palette.is_open()
                    || overlay.is_some()
                {
                    return;
                }

                let (x, y) = match last_cursor_pos {
                    Some(pos) => pos,
                    None => return,
                };

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
                let Some(session) = editor_runtime.tab_session_mut(focused, active_tab) else {
                    return;
                };

                if let Some(point) = hit_test_viewport_point(&session.viewport, viewport_rect, x, y)
                {
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
                    if let Some(damage) =
                        session.apply_action(&EditorAction::ClearComposition, viewport_rect)
                    {
                        scheduler.invalidate(damage.event);
                        scheduler.mark_hook(damage.hook, &clock);
                    }

                    if let Some(damage) = session.apply_action(
                        &EditorAction::ChangeSelection {
                            delta: SelectionDelta::Cleared,
                        },
                        viewport_rect,
                    ) {
                        scheduler.invalidate(damage.event);
                        scheduler.mark_hook(damage.hook, &clock);
                    }

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
                    &damage_geometry,
                    &mut palette,
                    &mut palette_focus_return,
                    &mut start_center,
                    &mut overlay,
                    &mut command_runtime,
                    &mut keybinding_runtime,
                    &mut enablement_runtime,
                    &mut recent_work,
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
                            let Some(viewport_rect) = damage_geometry
                                .focused_editor_viewport
                                .map(|rect| PixelRect::new(rect.x, rect.y, rect.width, rect.height))
                            else {
                                return;
                            };

                            if !editor_runtime.has_tab_session(focused, active_tab) {
                                editor_runtime.open_placeholder(focused, active_tab);
                            }

                            let Some(session) = editor_runtime.tab_session_mut(focused, active_tab)
                            else {
                                return;
                            };

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

                                if let Some(normalized) = session.text_input.handle_key_event(&key_event)
                                {
                                    let action = editor_action_from_text_input(normalized);
                                    if let Some(damage) = session.apply_action(&action, viewport_rect)
                                    {
                                        if damage.hook == Hook::ReflowLineRange {
                                            hot_path_metrics
                                                .note_keystroke_to_paint_admitted(clock.now().0);
                                        }
                                        scheduler.invalidate(damage.event);
                                        scheduler.mark_hook(damage.hook, &clock);
                                    }
                                    update_ime_cursor_area_for_viewport(
                                        &window,
                                        &session.viewport,
                                        viewport_rect,
                                    );
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
                    &recent_work,
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

fn window_title(
    focused: Option<ShellZoneId>,
    palette_selected: Option<&CommandRegistryEntryRecord>,
    active_workspace_label: Option<&str>,
) -> String {
    let identity = build_info::build_identity();
    let workspace_suffix = active_workspace_label
        .map(|label| format!(" — workspace: {label}"))
        .unwrap_or_default();
    let focus_suffix = focused
        .map(|z| format!(" — focus: {}", z.name()))
        .unwrap_or_default();
    let palette_suffix = palette_selected
        .map(|entry| format!(" — cmd: {}", entry.command_id()))
        .unwrap_or_default();
    format!(
        "Aureline Shell{}{}{}{}",
        workspace_suffix,
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
}

impl ReadOnlyState {
    const fn token(self) -> Option<&'static str> {
        match self {
            Self::Writable => None,
            Self::Filesystem => Some("Read-only"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GeneratedState {
    Authored,
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

struct BufferAuthority {
    label: String,
    file_path: Option<PathBuf>,
    read_only: ReadOnlyState,
    generated: GeneratedState,
    managed: ManagedState,
    projection: ProjectionState,
    snapshot_facing: SnapshotFacingState,
    buffer: Buffer,
    saved_revision: RevisionId,
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

#[derive(Debug, Clone)]
struct TabRenderInfo {
    label: String,
    dirty: bool,
    read_only: ReadOnlyState,
    generated: GeneratedState,
    managed: ManagedState,
    projection: ProjectionState,
    snapshot_facing: SnapshotFacingState,
}

struct BufferAuthorityStore {
    next_view_id: u64,
    by_canonical_path: HashMap<PathBuf, Rc<RefCell<BufferAuthority>>>,
}

impl BufferAuthorityStore {
    fn new() -> Self {
        Self {
            next_view_id: 1,
            by_canonical_path: HashMap::new(),
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

        let bytes = std::fs::read(&canonical).map_err(|err| err.to_string())?;
        let buffer = Buffer::from_bytes(&bytes);
        let label = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("untitled")
            .to_string();
        let read_only = read_only_state_for_path(&canonical);
        let saved_revision = buffer.revision_id();
        let authority = Rc::new(RefCell::new(BufferAuthority {
            label,
            file_path: Some(canonical.clone()),
            read_only,
            generated: GeneratedState::Authored,
            managed: ManagedState::Unmanaged,
            projection: ProjectionState::Direct,
            snapshot_facing: SnapshotFacingState::Live,
            buffer,
            saved_revision,
        }));
        self.by_canonical_path.insert(canonical, authority.clone());
        Ok(authority)
    }

    fn placeholder_authority(&mut self, label: impl Into<String>, text: &str) -> Rc<RefCell<BufferAuthority>> {
        let buffer = Buffer::from_str(text);
        let saved_revision = buffer.revision_id();
        Rc::new(RefCell::new(BufferAuthority {
            label: label.into(),
            file_path: None,
            read_only: ReadOnlyState::Writable,
            generated: GeneratedState::Authored,
            managed: ManagedState::Unmanaged,
            projection: ProjectionState::Direct,
            snapshot_facing: SnapshotFacingState::Live,
            buffer,
            saved_revision,
        }))
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
        TabRenderInfo {
            label: auth.label.clone(),
            dirty: auth.is_dirty(),
            read_only: auth.read_only,
            generated: auth.generated,
            managed: auth.managed,
            projection: auth.projection,
            snapshot_facing: auth.snapshot_facing,
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

        match action {
            EditorAction::InsertText { text } => {
                let scope = if self.viewport.caret_count() > 1 && self.viewport.ime_composition().is_some() {
                    aureline_editor::TextEditScope::PrimaryOnly
                } else {
                    aureline_editor::TextEditScope::AllCarets
                };

                let outcome = {
                    let mut authority = self.authority.borrow_mut();
                    self.viewport
                        .selections_mut()
                        .apply_insert_text(&mut authority.buffer, &self.snapshot, text, "user_keystroke", scope)
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
                    self.viewport
                        .selections_mut()
                        .apply_delete_backward(&mut authority.buffer, &self.snapshot, "user_keystroke", aureline_editor::TextEditScope::AllCarets)
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
            } => {
                if !self
                    .viewport
                    .move_caret(*movement, &self.line_graphemes, *extend_selection)
                {
                    return None;
                }
            }
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

struct EditorWorkspaceRuntimeState {
    text_runtime: EditorTextRuntime,
    groups: HashMap<PaneId, EditorGroupSession>,
    buffers: BufferAuthorityStore,
}

impl EditorWorkspaceRuntimeState {
    fn new() -> Self {
        Self {
            text_runtime: EditorTextRuntime::with_system_fonts(),
            groups: HashMap::new(),
            buffers: BufferAuthorityStore::new(),
        }
    }

    fn ensure_group(&mut self, group: PaneId) -> &mut EditorGroupSession {
        self.groups.entry(group).or_insert_with(EditorGroupSession::new)
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

    fn open_file(&mut self, group: PaneId, tab: EditorTabId, path: &Path) -> Result<(), String> {
        let authority = self.buffers.open_file_authority(path)?;
        self.ensure_tab_session(group, tab, authority);
        Ok(())
    }

    fn clone_tab_view(
        &mut self,
        source_group: PaneId,
        source_tab: EditorTabId,
        target_group: PaneId,
        target_tab: EditorTabId,
    ) -> bool {
        let Some(authority) = self
            .groups
            .get(&source_group)
            .and_then(|group| group.tabs.get(&source_tab))
            .map(|tab| tab.authority.clone())
        else {
            return false;
        };
        self.ensure_tab_session(target_group, target_tab, authority);
        true
    }

    fn save_tab(&mut self, group: PaneId, tab: EditorTabId) -> Result<(), String> {
        let Some(session) = self.groups.get_mut(&group).and_then(|g| g.tabs.get_mut(&tab)) else {
            return Err("tab not found".to_string());
        };
        session.ensure_fresh_snapshot();

        let mut authority = session.authority.borrow_mut();
        if authority.read_only != ReadOnlyState::Writable {
            return Err("tab is read-only".to_string());
        }
        let Some(path) = authority.file_path.clone() else {
            authority.mark_saved();
            return Ok(());
        };
        let snapshot = authority.buffer.snapshot();
        std::fs::write(&path, snapshot.as_bytes()).map_err(|err| err.to_string())?;
        authority.mark_saved();
        Ok(())
    }

    fn close_group(&mut self, group: PaneId) {
        self.groups.remove(&group);
    }

    fn close_tab(&mut self, group: PaneId, tab: EditorTabId) {
        if let Some(session) = self.groups.get_mut(&group) {
            session.tabs.remove(&tab);
        }
    }

    fn has_tab_session(&self, group: PaneId, tab: EditorTabId) -> bool {
        self.groups
            .get(&group)
            .is_some_and(|session| session.tabs.contains_key(&tab))
    }

    fn tab_session_mut(&mut self, group: PaneId, tab: EditorTabId) -> Option<&mut EditorTabSession> {
        self.groups.get_mut(&group)?.tabs.get_mut(&tab)
    }

    fn tab_render_info(&self, group: PaneId, tab: EditorTabId) -> Option<TabRenderInfo> {
        let group = self.groups.get(&group)?;
        let tab = group.tabs.get(&tab)?;
        Some(tab.render_info())
    }

    fn apply_action(
        &mut self,
        group: PaneId,
        tab: EditorTabId,
        action: &EditorAction,
        viewport_rect: PixelRect,
    ) -> Option<aureline_editor::ViewportDamage> {
        let group = self.groups.get_mut(&group)?;
        let tab = group.tabs.get_mut(&tab)?;
        tab.apply_action(action, viewport_rect)
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
        session.compositor.compose_into_window(
            window_buffer,
            window_width,
            window_height,
            viewport_rect,
            &session.viewport,
            paint_style,
            clip,
        );
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
            Ok(registry) => Self {
                store_path,
                registry,
                active_recent_work_id: None,
                active_workspace_label: None,
                suspended_frames: HashMap::new(),
                last_error: None,
            },
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

#[derive(Debug, Clone)]
struct AppearanceRuntimeState {
    store_path: PathBuf,
    policy_path: PathBuf,
    session: AppearanceSessionRecord,
    policy: LiveFollowSystemPolicyRecord,
    last_error: Option<String>,
}

impl AppearanceRuntimeState {
    fn load() -> Self {
        let store_path = PathBuf::from(".logs")
            .join("appearance")
            .join("appearance_session.json");
        let policy_path = PathBuf::from(".logs")
            .join("appearance")
            .join("live_follow_system_policy.json");

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

        let mut state = Self {
            store_path,
            policy_path,
            session,
            policy,
            last_error,
        };

        if needs_persist {
            state.persist();
        }

        state
    }

    const fn theme_class(&self) -> ThemeClass {
        self.session.theme_class()
    }

    const fn density_class(&self) -> DensityClass {
        self.session.density_class()
    }

    const fn reduced_motion_posture(&self) -> AccessibilityPostureClass {
        self.session.reduced_motion_posture
    }

    fn apply_cli_overrides(
        &mut self,
        theme_class: Option<ThemeClass>,
        density_class: Option<DensityClass>,
        reduced_motion_posture: Option<AccessibilityPostureClass>,
    ) {
        if theme_class.is_none() && density_class.is_none() && reduced_motion_posture.is_none() {
            return;
        }

        let minted_at = now_rfc3339();
        if let Some(theme) = theme_class {
            self.session.apply_theme_class(theme, minted_at.clone());
        }
        if let Some(density) = density_class {
            self.session.apply_density_class(density, minted_at.clone());
        }
        if let Some(posture) = reduced_motion_posture {
            self.session.apply_reduced_motion_posture(
                posture,
                ReducedMotionSource::UserSetting,
                minted_at,
            );
        }
        self.persist();
    }

    fn toggle_light_dark(&mut self) {
        let minted_at = now_rfc3339();
        self.session.toggle_light_dark(minted_at);
        self.persist();
    }

    fn toggle_high_contrast(&mut self) {
        let minted_at = now_rfc3339();
        self.session.toggle_high_contrast(minted_at);
        self.persist();
    }

    fn cycle_density_class(&mut self) {
        let minted_at = now_rfc3339();
        self.session.cycle_density_class(minted_at);
        self.persist();
    }

    fn cycle_reduced_motion_posture(&mut self) {
        let minted_at = now_rfc3339();
        self.session.cycle_reduced_motion_posture(minted_at);
        self.persist();
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
    }
}

fn recent_work_id_for(target_kind: TargetKind, identity_key: &str) -> String {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in identity_key.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("recent:{}:{:016x}", target_kind.as_str(), hash)
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

fn trust_state_for_recent_work(value: &str) -> TrustState {
    match value {
        "trusted" => TrustState::Trusted,
        "restricted" => TrustState::Restricted,
        "pending_evaluation" => TrustState::PendingEvaluation,
        _ => TrustState::PendingEvaluation,
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
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    palette: &mut CommandPaletteState,
    palette_focus_return: &mut FocusReturnStack<ShellFocusReturnTarget>,
    overlay: &mut Option<ShellOverlayState>,
    command_id: &str,
    origin: DispatchOrigin,
    enablement_runtime: &CommandEnablementRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
) -> bool {
    dispatch_command_id_with_arguments(
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
        recent_work,
        None,
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
    recent_work: &mut RecentWorkRuntimeState,
    argument_provenance_map_override: Option<Vec<ArgumentProvenanceEntry>>,
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
        recent_work,
        argument_provenance_map_override,
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
    recent_work: &mut RecentWorkRuntimeState,
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
            if !palette.is_open() {
                palette_focus_return.record_if_changed(ShellFocusReturnTarget::capture(frame));
                palette.open(registry, cwd);
                frame.focus_zone(ShellZoneId::TransientOverlay);
            }
            let invocation = invocation_and_result_simple_success(&session, "succeeded");
            command_runtime.record(invocation);
            true
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
            let group = frame.focused_editor_group();
            if let Some(tab) = frame.open_tab() {
                editor_runtime.open_placeholder(group, tab);
            }
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

            let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
            recent_work.note_local_folder_opened(
                &cwd,
                trust_state_for_recent_work(enablement_runtime.workspace_trust_state.as_str()),
            );
            if let Err(err) = recent_work.save() {
                command_runtime.note_non_command_action(format!("recent work save failed — {err}"));
            }
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
    frame: &mut DesktopFrame,
    command_runtime: &mut CommandRuntimeState,
    mut entry: RecentWorkEntryRecord,
    trust_override: Option<TrustState>,
) {
    recent_work.suspend_active_frame(frame);
    recent_work.activate_recent_work(frame, window_dims, &entry);

    let trust_state = trust_override.unwrap_or(entry.trust_state);
    *workspace_trust_state = trust_state.as_str().to_string();

    let opened_at = mono_timestamp_now();
    entry.last_opened_at = opened_at.clone();
    recent_work.registry.updated_at = opened_at;
    recent_work.registry.upsert(entry);

    if let Err(err) = recent_work.save() {
        command_runtime.note_non_command_action(format!("recent work save failed — {err}"));
    } else {
        command_runtime.note_non_command_action("workspace switch applied");
    }
}

fn apply_workspace_switcher_decision(
    window_size: &LogicalSize<u32>,
    workspace_trust_state: &mut String,
    recent_work: &mut RecentWorkRuntimeState,
    frame: &mut DesktopFrame,
    command_runtime: &mut CommandRuntimeState,
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
                        frame,
                        command_runtime,
                        entry,
                        Some(TrustState::Restricted),
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
                        frame,
                        command_runtime,
                        entry,
                        None,
                    );
                }
                Some(_) => activate_recent_work_entry(
                    window_dims,
                    workspace_trust_state,
                    recent_work,
                    frame,
                    command_runtime,
                    entry,
                    None,
                ),
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
                activate_recent_work_entry(
                    window_dims,
                    workspace_trust_state,
                    recent_work,
                    frame,
                    command_runtime,
                    entry,
                    None,
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
                activate_recent_work_entry(
                    window_dims,
                    workspace_trust_state,
                    recent_work,
                    frame,
                    command_runtime,
                    entry,
                    None,
                );
            }
            SafeRecoveryAction::OpenRestricted => {
                let Some(entry) = recent_work.find_entry(&recent_work_id).cloned() else {
                    command_runtime.note_non_command_action(format!(
                        "workspace switch failed: missing recent-work id {recent_work_id}"
                    ));
                    return;
                };
                activate_recent_work_entry(
                    window_dims,
                    workspace_trust_state,
                    recent_work,
                    frame,
                    command_runtime,
                    entry,
                    Some(TrustState::Restricted),
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
                activate_recent_work_entry(
                    window_dims,
                    workspace_trust_state,
                    recent_work,
                    frame,
                    command_runtime,
                    entry,
                    None,
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
            SafeRecoveryAction::LocateMissingTarget => command_runtime
                .note_non_command_action("locate missing target not implemented in shell build"),
            SafeRecoveryAction::Reconnect => {
                command_runtime.note_non_command_action("reconnect not implemented in shell build")
            }
            SafeRecoveryAction::Reauth => {
                command_runtime.note_non_command_action("reauth not implemented in shell build")
            }
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
    editor_runtime: &mut EditorWorkspaceRuntimeState,
    damage_geometry: &ShellDamageGeometryCache,
    palette: &mut CommandPaletteState,
    palette_focus_return: &mut FocusReturnStack<ShellFocusReturnTarget>,
    start_center: &mut StartCenterState,
    overlay: &mut Option<ShellOverlayState>,
    command_runtime: &mut CommandRuntimeState,
    keybinding_runtime: &mut KeybindingRuntimeState,
    enablement_runtime: &mut CommandEnablementRuntimeState,
    recent_work: &mut RecentWorkRuntimeState,
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
                };
                let preview = materialize_palette_preview_record(
                    palette.selected_key(),
                    registry,
                    &keybinding_runtime.shortcuts_by_command_id,
                    runtime,
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
                };
                let record = materialize_command_diagnostics_sheet_record(entry, runtime);
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
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    None,
                    recent_work.active_workspace_label(),
                ));

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
                        let changed = dispatch_command_id(
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
                            recent_work,
                        );
                        window.set_title(&window_title(
                            Some(frame.focused_zone()),
                            None,
                            recent_work.active_workspace_label(),
                        ));
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
                        let cwd = std::env::current_dir()
                            .unwrap_or_else(|_| std::path::PathBuf::from("."));
                        let path = cwd.join(&relative_path);
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
                        window.set_title(&window_title(
                            Some(frame.focused_zone()),
                            None,
                            recent_work.active_workspace_label(),
                        ));
                        ShellDamageHint::FullWindow
                    }
                    None => {
                        window.set_title(&window_title(
                            Some(frame.focused_zone()),
                            None,
                            recent_work.active_workspace_label(),
                        ));
                        panel_hint(DamageClassId::TextReflowLocal)
                    }
                }
            }
            KeyCode::Escape => {
                palette.write_snapshot_log(registry, &keybinding_runtime.shortcuts_by_command_id);
                palette.close();
                if let Some(target) = palette_focus_return.pop() {
                    target.apply(frame);
                }
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    None,
                    recent_work.active_workspace_label(),
                ));
                ShellDamageHint::FullWindow
            }
            KeyCode::ArrowDown => {
                let handled = palette.handle_arrow_down();
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    palette.selected_entry(registry),
                    recent_work.active_workspace_label(),
                ));
                if handled {
                    panel_hint(DamageClassId::SelectionOverlayOnly)
                } else {
                    ShellDamageHint::None
                }
            }
            KeyCode::ArrowUp => {
                let handled = palette.handle_arrow_up();
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    palette.selected_entry(registry),
                    recent_work.active_workspace_label(),
                ));
                if handled {
                    panel_hint(DamageClassId::SelectionOverlayOnly)
                } else {
                    ShellDamageHint::None
                }
            }
            KeyCode::Backspace => {
                let handled =
                    palette.handle_backspace(registry, &keybinding_runtime.shortcuts_by_command_id);
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    palette.selected_entry(registry),
                    recent_work.active_workspace_label(),
                ));
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
                            window.set_title(&window_title(
                                Some(frame.focused_zone()),
                                palette.selected_entry(registry),
                                recent_work.active_workspace_label(),
                            ));
                            return panel_hint(DamageClassId::TextReflowLocal);
                        }
                    }
                }
                ShellDamageHint::None
            }
        };
    }

    if let Some(state) = overlay.as_mut() {
        let outcome = state.handle_key(code, event.text.as_deref(), frame, keybinding_runtime);
        if let Some(decision) = outcome.command_decision {
            finalize_command_overlay_decision(command_runtime, registry, decision);
        }
        if let Some(decision) = outcome.workspace_switcher_decision {
            apply_workspace_switcher_decision(
                &window.inner_size().to_logical::<u32>(window.scale_factor()),
                &mut enablement_runtime.workspace_trust_state,
                recent_work,
                frame,
                command_runtime,
                decision,
            );
        }
        if outcome.handled {
            if state.closed {
                *overlay = None;
            }
            window.set_title(&window_title(
                Some(frame.focused_zone()),
                None,
                recent_work.active_workspace_label(),
            ));
            return ShellDamageHint::FullWindow;
        }
        return ShellDamageHint::None;
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
                    editor_runtime,
                    palette,
                    palette_focus_return,
                    overlay,
                    candidate.command.command_id.as_str(),
                    DispatchOrigin::KeybindingChord,
                    enablement_runtime,
                    recent_work,
                );
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    palette
                        .is_open()
                        .then(|| palette.selected_entry(registry))
                        .flatten(),
                    recent_work.active_workspace_label(),
                ));
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
        };
        let rows = start_center_action_rows(registry, runtime);
        let row_count = rows.len();

        match code {
            KeyCode::ArrowDown => {
                start_center.select_next(row_count);
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    None,
                    recent_work.active_workspace_label(),
                ));
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
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    None,
                    recent_work.active_workspace_label(),
                ));
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
                let changed = dispatch_command_id_with_arguments(
                    command_runtime,
                    registry,
                    frame,
                    editor_runtime,
                    palette,
                    palette_focus_return,
                    overlay,
                    row.command_id,
                    DispatchOrigin::StartCenter,
                    enablement_runtime,
                    recent_work,
                    Some(row.argument_provenance_map.clone()),
                );
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    None,
                    recent_work.active_workspace_label(),
                ));
                if !changed {
                    return ShellDamageHint::None;
                }
                if row.command_id == "cmd:workspace.open_folder" {
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
                    recent_work,
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
            window.set_title(&window_title(
                Some(frame.focused_zone()),
                None,
                recent_work.active_workspace_label(),
            ));
            ShellDamageHint::FullWindow
        }
        KeyCode::KeyO => {
            if modifiers.ctrl_or_logo() {
                let group = frame.focused_editor_group();
                if let Some(tab) = frame.open_tab() {
                    editor_runtime.open_placeholder(group, tab);
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
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    None,
                    recent_work.active_workspace_label(),
                ));
                ShellDamageHint::FullWindow
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyS => {
            if modifiers.ctrl_or_logo() && frame.focused_zone() == ShellZoneId::MainWorkspace {
                let group = frame.focused_editor_group();
                let Some(tab) = frame.active_tab_id(group) else {
                    return ShellDamageHint::None;
                };
                if let Err(err) = editor_runtime.save_tab(group, tab) {
                    command_runtime.note_non_command_action(format!("save failed — {err}"));
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
        KeyCode::KeyZ => {
            if modifiers.ctrl_or_logo() && frame.focused_zone() == ShellZoneId::MainWorkspace {
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

                let changed = {
                    let mut authority = session.authority.borrow_mut();
                    let outcome = if modifiers.shift {
                        authority.buffer.redo()
                    } else {
                        authority.buffer.undo()
                    };
                    outcome.is_some()
                };
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
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyY => {
            if modifiers.ctrl_or_logo() && frame.focused_zone() == ShellZoneId::MainWorkspace {
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

                let changed = {
                    let mut authority = session.authority.borrow_mut();
                    authority.buffer.redo().is_some()
                };
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
            if modifiers.ctrl_or_logo() && modifiers.shift {
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
                enablement_runtime.toggle_trust_state();
                ShellDamageHint::FullWindow
            } else {
                ShellDamageHint::None
            }
        }
        KeyCode::KeyE => {
            if modifiers.ctrl_or_logo() && modifiers.shift {
                enablement_runtime.toggle_execution_context();
                ShellDamageHint::FullWindow
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
        InputAction::MoveCaret {
            movement,
            extend_selection,
        } => EditorAction::MoveCaret {
            movement: match movement {
                InputMove::Left => CaretMove::Left,
                InputMove::Right => CaretMove::Right,
                InputMove::Up => CaretMove::Up,
                InputMove::Down => CaretMove::Down,
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

    let x_rel = line
        .grapheme_x_px
        .get(caret.grapheme)
        .copied()
        .unwrap_or(0);
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
    recent_work: &RecentWorkRuntimeState,
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
                recent_work,
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
                    recent_work,
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
    recent_work: &RecentWorkRuntimeState,
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
                    draw_text(
                        buffer,
                        width,
                        height,
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
                buffer,
                width,
                height,
                rect,
                focus_ring.stroke_px,
                focus_ring.color,
            );
        }

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
        let docs_keys = keybinding_runtime.shortcuts_label("cmd:docs.open_in_browser");
        let trust_state = enablement_runtime.workspace_trust_state.as_str();
        let theme_label = appearance.theme_class().token();
        let density_label = appearance.density_class().token();
        let motion_label = appearance.reduced_motion_posture().token();
        let active_workspace = recent_work.active_workspace_label().unwrap_or("none");
        let recent_work_store = recent_work.store_path.display();
        let text = format!(
            "theme: {}   density: {}   motion: {}   fallback_modes: [{}]   workspace: {}   last_cmd: {}   last_keybinding: {}   enablement: trust={} exec_ctx={} policy={}   keymap: {} ({})   keys: {} palette (resolver)   docs: {} open in browser   Cmd/Ctrl+Shift+R switcher, Enter run, Ctrl+\\\\ split view, Ctrl+Tab next tab, Ctrl+G next group, Ctrl+O new tab, Ctrl+S save, Ctrl+W close tab, Ctrl+Shift+W close group, Ctrl+I keybinding inspector   toggles: Cmd/Ctrl+Shift+T trust, Cmd/Ctrl+Shift+E exec_ctx, Cmd/Ctrl+Shift+B policy, Cmd/Ctrl+Shift+L theme, Ctrl+Alt+Shift+H high contrast, Cmd/Ctrl+Shift+M density, Cmd/Ctrl+Alt+Shift+M motion   packets: .logs/command_packets   recents: {}",
            theme_label,
            density_label,
            motion_label,
            modes,
            active_workspace,
            last,
            last_keybinding,
            trust_state,
            exec_ctx,
            policy,
            keybinding_runtime.active_preset.display_name(),
            keybinding_runtime.active_preset.preset_ref(),
            palette_keys,
            docs_keys,
            recent_work_store
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
        fill_rect(buffer, width, height, badge_rect, badge_fill);
        stroke_rect(
            buffer,
            width,
            height,
            badge_rect,
            style.stroke_default,
            badge_border,
        );
        draw_text(
            buffer,
            width,
            height,
            badge_rect.x.saturating_add(badge_padding),
            badge_rect.y.saturating_add(badge_padding),
            badge_scale,
            badge_text,
            badge_fg,
        );

        draw_text(
            buffer,
            width,
            height,
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
    fn new(snapshot: RecentWorkRegistry, current_trust_state: String) -> Self {
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

#[derive(Debug)]
struct OverlayKeyOutcome {
    handled: bool,
    command_decision: Option<CommandOverlayDecision>,
    workspace_switcher_decision: Option<WorkspaceSwitcherDecision>,
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
    WorkspaceSwitcher(WorkspaceSwitcherOverlay),
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
        frame: &mut DesktopFrame,
        keybinding_runtime: &mut KeybindingRuntimeState,
    ) -> OverlayKeyOutcome {
        let mut command_decision = None;
        let mut workspace_switcher_decision = None;
        let handled = match (&mut self.kind, code) {
            (ShellOverlayKind::WorkspaceSwitcher(switcher), KeyCode::Escape) => {
                if matches!(switcher.mode, WorkspaceSwitcherOverlayMode::List) {
                    self.close(frame);
                } else {
                    switcher.mode = WorkspaceSwitcherOverlayMode::List;
                }
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
                                workspace_switcher_decision,
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
                                workspace_switcher_decision,
                            };
                        };

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

                        if !allows_open {
                            switcher.mode = WorkspaceSwitcherOverlayMode::RecoveryActions {
                                recent_work_id: entry.recent_work_id.clone(),
                                selection: 0,
                            };
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                workspace_switcher_decision,
                            };
                        }

                        if switcher.requires_switch_preview(entry) {
                            switcher.mode = WorkspaceSwitcherOverlayMode::ConfirmSwitch {
                                recent_work_id: entry.recent_work_id.clone(),
                            };
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                workspace_switcher_decision,
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
                                workspace_switcher_decision,
                            };
                        };

                        let Some(action) = entry.safe_recovery_actions.get(selection).copied()
                        else {
                            switcher.mode = WorkspaceSwitcherOverlayMode::List;
                            return OverlayKeyOutcome {
                                handled: true,
                                command_decision,
                                workspace_switcher_decision,
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
            if let ShellOverlayKind::WorkspaceSwitcher(switcher) = &mut self.kind {
                if !matches!(switcher.mode, WorkspaceSwitcherOverlayMode::List) {
                    return OverlayKeyOutcome {
                        handled,
                        command_decision,
                        workspace_switcher_decision,
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
                            workspace_switcher_decision,
                        };
                    }
                }
            }
        }

        OverlayKeyOutcome {
            handled,
            command_decision,
            workspace_switcher_decision,
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
                        "{} — {}  ({}, {}, {}, {})",
                        entry.presentation_label,
                        subtitle,
                        entry.target_kind.as_str(),
                        entry.target_state.as_str(),
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
                let line = format!(
                    "{pin} {} — {}  last:{}  ({}, {}, {}, {})",
                    row.primary_label,
                    subtitle,
                    row.last_opened_at,
                    row.target_kind.as_str(),
                    row.target_state.as_str(),
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
    let packet_ref = card
        .open_in_browser_action()
        .and_then(|row| row.browser_handoff_packet_ref.as_deref())
        .unwrap_or("missing");
    let action_label = card
        .open_in_browser_action()
        .map(|row| row.action_label.as_str())
        .unwrap_or("Open in browser");

    let mut lines = vec![
        format!("Owner: {}", card.owner_identity.label),
        format!("Publisher: {}", card.publisher_or_service_identity.label),
        format!(
            "Origin: {} ({})",
            card.origin_identity.origin_label, card.origin_identity.host_or_domain_label
        ),
        format!("Boundary: {}", card.data_boundary_label),
        format!("State: {}", card.boundary_state_label),
        format!("Permission: {}", card.permission_state.permission_label),
    ];
    if let Some(source_truth) = card.source_truth.as_ref() {
        lines.push(format!(
            "Source: {}",
            token_string(&source_truth.source_class)
        ));
        lines.push(format!(
            "Version: {}",
            token_string(&source_truth.version_match_state)
        ));
        lines.push(format!(
            "Freshness: {}",
            token_string(&source_truth.freshness_class)
        ));
        lines.push(format!(
            "Build: {}",
            source_truth.running_build_identity_ref
        ));
    }
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

fn token_string(value: &(impl Serialize + std::fmt::Debug)) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(ToString::to_string))
        .unwrap_or_else(|| format!("{value:?}"))
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
                assert!(info1.dirty, "expected tab1 to remain Modified after save failure");
                assert!(info2.dirty, "expected tab2 to remain Modified after save failure");
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
