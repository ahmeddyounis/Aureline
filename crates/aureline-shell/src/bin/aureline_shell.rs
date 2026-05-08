use std::num::NonZeroU32;
use std::sync::Arc;

use aureline_build_info as build_info;
use aureline_commands::registry::seeded_registry;
use aureline_commands::{CommandRegistry, CommandRegistryEntryRecord};
use aureline_shell::app_frame::desktop_frame::{DesktopFrame, NewEditorGroupOutcome, SplitViolation};
use aureline_shell::layout::split_tree::PaneId;
use aureline_shell::layout::zone_registry::{Rect, ShellZoneId};

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

    window.request_redraw();

    event_loop.run(move |event, elwt| {
        match event {
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
                        &held_modifiers,
                        event,
                    ) {
                        window.request_redraw();
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Err(err) = draw(&window, &mut surface, registry, &frame, &palette, overlay.as_ref()) {
                        eprintln!("aureline_shell: draw failed: {err}");
                        elwt.exit();
                    }
                }
                _ => {}
            },
            _ => {}
        }
    })?;
    Ok(())
}

fn window_title(focused: Option<ShellZoneId>, palette_selected: Option<&CommandRegistryEntryRecord>) -> String {
    let identity = build_info::build_identity();
    let focus_suffix = focused.map(|z| format!(" — focus: {}", z.name())).unwrap_or_default();
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

fn handle_key_event(
    window: &winit::window::Window,
    registry: &CommandRegistry,
    frame: &mut DesktopFrame,
    palette: &mut CommandPaletteState,
    overlay: &mut Option<ShellOverlayState>,
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
        if state.handle_key(code, frame) {
            if state.closed {
                *overlay = None;
            }
            window.set_title(&window_title(Some(frame.focused_zone()), None));
            return true;
        }
        return false;
    }

    match code {
        KeyCode::Tab => {
            frame.focus_next();
            window.set_title(&window_title(Some(frame.focused_zone()), None));
            true
        }
        KeyCode::KeyP => {
            if modifiers.ctrl_or_logo() && modifiers.shift {
                palette.open();
                window.set_title(&window_title(
                    Some(frame.focused_zone()),
                    palette.selected_entry(registry),
                ));
                true
            } else {
                false
            }
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

    if let (Some(w), Some(h)) = (NonZeroU32::new(physical.width), NonZeroU32::new(physical.height))
    {
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
                    if group.group_id == frame.focused_editor_group() && frame.focused_zone() == ShellZoneId::MainWorkspace {
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
                        if group.tabbed_compare_active { "  [tabbed compare]" } else { "" }
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
                    fill_rect(&mut buffer, physical.width, physical.height, slot_rect, slot_color);
                }
            }
        }

        if zone == frame.focused_zone() {
            stroke_rect(&mut buffer, physical.width, physical.height, rect, 2, 0x00ffffff);
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
        draw_shell_overlay(&mut buffer, physical.width, physical.height, window.scale_factor(), frame, overlay);
    }

    let modes = frame
        .responsive_fallback_modes()
        .into_iter()
        .map(|m| m.name())
        .collect::<Vec<_>>()
        .join(", ");
    let status = to_physical_rect(frame.layout().status_bar, scale);
    if !status.is_empty() {
        let text = format!("fallback_modes: [{}]   keys: Ctrl+\\ split, Ctrl+G next group, Ctrl+O add tab, Ctrl+W close group, Ctrl+I inspector (sheet)", modes);
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
enum ShellOverlayKind {
    InspectorSheet,
    SplitChoice { violation: SplitViolation, selection: usize },
    StagedPeek,
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

    fn close(&mut self, frame: &mut DesktopFrame) {
        self.closed = true;
        frame.focus_zone(self.focus_return_zone);
        if self.focus_return_zone == ShellZoneId::MainWorkspace {
            frame.focus_editor_group(self.focus_return_group);
        }
    }

    fn handle_key(&mut self, code: KeyCode, frame: &mut DesktopFrame) -> bool {
        match (&mut self.kind, code) {
            (_, KeyCode::Escape) => {
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
        ShellOverlayKind::SplitChoice { violation, selection } => {
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
                    if idx == *selection { 0x00ffffff } else { 0x00c9d3de },
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
                let visible_in_palette = entry.descriptor.palette_visibility != "hidden_palette_callable_only";
                (desktop_ok && visible_in_palette).then_some(idx)
            })
            .collect();
        self.selection = self.selection.min(self.visible_entry_indices.len().saturating_sub(1));
    }

    fn is_open(&self) -> bool {
        self.open
    }

    fn open(&mut self) {
        self.open = true;
        self.selection = self.selection.min(self.visible_entry_indices.len().saturating_sub(1));
    }

    fn close(&mut self) {
        self.open = false;
    }

    fn selected_entry<'a>(&self, registry: &'a CommandRegistry) -> Option<&'a CommandRegistryEntryRecord> {
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
    Rect::new(scale(rect.x), scale(rect.y), scale(rect.width), scale(rect.height))
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
    fill_rect(
        buffer,
        width,
        height,
        overlay_physical,
        0x00101010,
    );

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
        "Command Palette (Ctrl+Shift+P)",
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
        "Up/Down: select   Esc: close",
        0x00aab7c4,
    );
    cursor_y = cursor_y.saturating_add(line_h + 6);

    for (row, entry) in palette_rows(registry).iter().enumerate() {
        if cursor_y.saturating_add(line_h) > panel.bottom().saturating_sub(12) {
            break;
        }
        let selected = row == palette.selection;
        if selected {
            let highlight = Rect::new(panel.x.saturating_add(6), cursor_y.saturating_sub(2), panel.width.saturating_sub(12), line_h);
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
            entry.descriptor.client_scopes.iter().any(|scope| scope == "desktop_product")
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
            KeyCode::ShiftLeft | KeyCode::ShiftRight => self.shift = pressed,
            KeyCode::SuperLeft | KeyCode::SuperRight => self.logo = pressed,
            _ => {}
        }
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
