//! Draw-queue vocabulary and queue ownership.
//!
//! The renderer consumes draw work through a single queue. Producers (shell
//! chrome, editor viewport, terminal placeholders) enqueue damage events using
//! stable vocabulary derived from the composition and damage-class packets.

/// A stable composition layer id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CompositionLayerId {
    WindowChromeBase,
    TextAndDecoration,
    OverlayEphemera,
    FloatingSurface,
    MenuSurface,
    DialogSurface,
    ToastSurface,
    CriticalSurface,
}

impl CompositionLayerId {
    /// Returns the canonical id string for this composition layer.
    pub const fn id(self) -> &'static str {
        match self {
            Self::WindowChromeBase => "render_layer.window_chrome_base",
            Self::TextAndDecoration => "render_layer.text_and_decoration",
            Self::OverlayEphemera => "render_layer.overlay_ephemera",
            Self::FloatingSurface => "render_layer.floating_surface",
            Self::MenuSurface => "render_layer.menu_surface",
            Self::DialogSurface => "render_layer.dialog_surface",
            Self::ToastSurface => "render_layer.toast_surface",
            Self::CriticalSurface => "render_layer.critical_surface",
        }
    }
}

/// A stable damage-class id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DamageClassId {
    StartupFirstPaint,
    TextReflowLocal,
    CaretOverlayOnly,
    SelectionOverlayOnly,
    ImeMarkedTextOverlay,
    ViewportScrollTranslate,
    ViewportResizeOrScaleChange,
    FloatingSurfaceToggle,
    AppearanceSessionFlip,
    WindowExposedRegionRefresh,
    DegradedFullWindowFallback,
}

impl DamageClassId {
    /// Returns the canonical id string for this damage class.
    pub const fn id(self) -> &'static str {
        match self {
            Self::StartupFirstPaint => "render_damage.startup_first_paint",
            Self::TextReflowLocal => "render_damage.text_reflow_local",
            Self::CaretOverlayOnly => "render_damage.caret_overlay_only",
            Self::SelectionOverlayOnly => "render_damage.selection_overlay_only",
            Self::ImeMarkedTextOverlay => "render_damage.ime_marked_text_overlay",
            Self::ViewportScrollTranslate => "render_damage.viewport_scroll_translate",
            Self::ViewportResizeOrScaleChange => "render_damage.viewport_resize_or_scale_change",
            Self::FloatingSurfaceToggle => "render_damage.floating_surface_toggle",
            Self::AppearanceSessionFlip => "render_damage.appearance_session_flip",
            Self::WindowExposedRegionRefresh => "render_damage.window_exposed_region_refresh",
            Self::DegradedFullWindowFallback => "render_damage.degraded_full_window_fallback",
        }
    }
}

/// A single queued damage event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DamageEvent {
    pub layer: CompositionLayerId,
    pub class: DamageClassId,
}

impl DamageEvent {
    /// Creates a new damage event.
    pub const fn new(layer: CompositionLayerId, class: DamageClassId) -> Self {
        Self { layer, class }
    }
}

/// A coalesced unit of draw work the renderer submits as one frame.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompositedFrame {
    pub frame_index: u64,
    pub events: Vec<DamageEvent>,
}

impl CompositedFrame {
    /// Returns true when there is no queued damage.
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

/// Owns draw-queue state and coalesces incoming damage events.
#[derive(Debug, Clone)]
pub struct DrawQueue {
    next_frame_index: u64,
    pending: Vec<DamageEvent>,
    dropped_events: u64,
    max_pending_events: usize,
}

impl Default for DrawQueue {
    fn default() -> Self {
        Self::new(2048)
    }
}

impl DrawQueue {
    /// Creates a new draw queue with a bounded pending-event cap.
    pub fn new(max_pending_events: usize) -> Self {
        Self {
            next_frame_index: 0,
            pending: Vec::new(),
            dropped_events: 0,
            max_pending_events: max_pending_events.max(1),
        }
    }

    /// Enqueues a damage event, coalescing adjacent duplicates.
    pub fn push(&mut self, event: DamageEvent) {
        if self.pending.last().is_some_and(|last| *last == event) {
            return;
        }
        if self.pending.len() >= self.max_pending_events {
            self.dropped_events = self.dropped_events.saturating_add(1);
            return;
        }
        self.pending.push(event);
    }

    /// Returns the number of pending events.
    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }

    /// Returns the number of dropped events due to queue pressure.
    pub fn dropped_events(&self) -> u64 {
        self.dropped_events
    }

    /// Drains pending events and advances the frame index.
    pub fn take_frame(&mut self) -> CompositedFrame {
        let events = std::mem::take(&mut self.pending);
        let frame_index = self.next_frame_index;
        self.next_frame_index = self.next_frame_index.saturating_add(1);
        CompositedFrame { frame_index, events }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adjacent_duplicate_damage_is_coalesced() {
        let mut queue = DrawQueue::new(8);
        let event = DamageEvent::new(
            CompositionLayerId::WindowChromeBase,
            DamageClassId::WindowExposedRegionRefresh,
        );
        queue.push(event);
        queue.push(event);
        assert_eq!(queue.pending_len(), 1);
    }

    #[test]
    fn queue_pressure_drops_events_without_panicking() {
        let mut queue = DrawQueue::new(2);
        let a = DamageEvent::new(
            CompositionLayerId::WindowChromeBase,
            DamageClassId::WindowExposedRegionRefresh,
        );
        let b = DamageEvent::new(
            CompositionLayerId::TextAndDecoration,
            DamageClassId::ViewportResizeOrScaleChange,
        );
        let c = DamageEvent::new(
            CompositionLayerId::OverlayEphemera,
            DamageClassId::CaretOverlayOnly,
        );
        queue.push(a);
        queue.push(b);
        queue.push(c);
        assert_eq!(queue.pending_len(), 2);
        assert_eq!(queue.dropped_events(), 1);
        let frame = queue.take_frame();
        assert_eq!(frame.events.len(), 2);
    }
}

