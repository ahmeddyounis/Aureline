use std::num::NonZeroU32;
use std::sync::Arc;

use aureline_build_info as build_info;
use aureline_shell::app_frame::desktop_frame::DesktopFrame;
use aureline_shell::layout::zone_registry::{Rect, ShellZoneId};

use softbuffer::{Context, Surface};
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{KeyCode, PhysicalKey};
use winit::window::WindowBuilder;

type SoftbufferSurface = Surface<Arc<winit::window::Window>, Arc<winit::window::Window>>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        WindowBuilder::new()
            .with_title(window_title(None))
            .with_inner_size(LogicalSize::new(1920.0, 1080.0))
            .build(&event_loop)?,
    );

    let context = Context::new(window.clone())?;
    let mut surface = Surface::new(&context, window.clone())?;

    let mut frame = {
        let logical = window.inner_size().to_logical::<u32>(window.scale_factor());
        DesktopFrame::new(logical.width, logical.height)
    };

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
                    if handle_key_event(&window, &mut frame, event) {
                        window.request_redraw();
                    }
                }
                WindowEvent::RedrawRequested => {
                    if let Err(err) = draw(&window, &mut surface, &frame) {
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

fn window_title(focused: Option<ShellZoneId>) -> String {
    let identity = build_info::build_identity();
    let focus_suffix = focused.map(|z| format!(" — focus: {}", z.name())).unwrap_or_default();
    format!("Aureline Shell{}{}", focus_suffix, format!(" ({})", identity.commit_short))
}

fn handle_key_event(window: &winit::window::Window, frame: &mut DesktopFrame, event: KeyEvent) -> bool {
    if event.state != ElementState::Pressed || event.repeat {
        return false;
    }

    let PhysicalKey::Code(code) = event.physical_key else {
        return false;
    };

    match code {
        KeyCode::Tab => {
            frame.focus_next();
            window.set_title(&window_title(Some(frame.focused_zone())));
            true
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
    frame: &DesktopFrame,
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

        for (slot_id, slot_rect) in frame.slot_rects_within_zone(zone, logical_rect) {
            let slot_rect = to_physical_rect(slot_rect, scale);
            let slot_color = slot_color(slot_id);
            fill_rect(&mut buffer, physical.width, physical.height, slot_rect, slot_color);
        }

        if zone == frame.focused_zone() {
            stroke_rect(&mut buffer, physical.width, physical.height, rect, 2, 0x00ffffff);
        }
    }

    buffer.present()?;
    Ok(())
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
