//! Renderer timing hook vocabulary.
//!
//! This module defines the canonical hook ids used by the renderer hot path.
//! Higher layers may record additional structured context, but they must not
//! mint parallel names for the same measurement.

use std::time::Instant;

/// A monotonic timestamp measured in opaque "ticks".
///
/// The unit is intentionally unspecified so test harnesses can use a
/// deterministic counter while production code can map ticks onto wall time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tick(pub u64);

/// Clock interface used by render scheduling and trace recorders.
pub trait Clock {
    /// Returns the current monotonic tick.
    fn now(&self) -> Tick;
}

/// Wall clock backed by [`Instant`].
#[derive(Debug)]
pub struct WallClock {
    origin: Instant,
}

impl WallClock {
    /// Creates a new wall-clock instance.
    pub fn new() -> Self {
        Self {
            origin: Instant::now(),
        }
    }
}

impl Default for WallClock {
    fn default() -> Self {
        Self::new()
    }
}

impl Clock for WallClock {
    fn now(&self) -> Tick {
        let nanos = self.origin.elapsed().as_nanos();
        Tick(u64::try_from(nanos).unwrap_or(u64::MAX))
    }
}

/// A trace-facing renderer hook id.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Hook {
    WarmStartToFirstPaint,
    FirstPaint,
    ScrollFrame,
    CaretMove,
    SelectionChange,
    ImeCompositionUpdate,
    FallbackGlyphResolution,
    MultiMonitorScaleChange,
    AtlasShardRebind,
    AtlasEviction,
    FrameSubmit,
    ReflowLineRange,
    DegradedRendererBanner,
    AccessibilityTreeUpdate,
}

impl Hook {
    /// Returns the canonical string id for this hook.
    pub const fn id(self) -> &'static str {
        match self {
            Self::WarmStartToFirstPaint => "warm_start_to_first_paint",
            Self::FirstPaint => "first_paint",
            Self::ScrollFrame => "scroll_frame",
            Self::CaretMove => "caret_move",
            Self::SelectionChange => "selection_change",
            Self::ImeCompositionUpdate => "ime_composition_update",
            Self::FallbackGlyphResolution => "fallback_glyph_resolution",
            Self::MultiMonitorScaleChange => "multi_monitor_scale_change",
            Self::AtlasShardRebind => "atlas_shard_rebind",
            Self::AtlasEviction => "atlas_eviction",
            Self::FrameSubmit => "frame_submit",
            Self::ReflowLineRange => "reflow_line_range",
            Self::DegradedRendererBanner => "degraded_renderer_banner",
            Self::AccessibilityTreeUpdate => "accessibility_tree_update",
        }
    }

    /// Returns whether this hook is treated as protected hot-path work.
    pub const fn is_protected_hot_path(self) -> bool {
        match self {
            Self::AtlasEviction | Self::DegradedRendererBanner => false,
            Self::WarmStartToFirstPaint
            | Self::FirstPaint
            | Self::ScrollFrame
            | Self::CaretMove
            | Self::SelectionChange
            | Self::ImeCompositionUpdate
            | Self::FallbackGlyphResolution
            | Self::MultiMonitorScaleChange
            | Self::AtlasShardRebind
            | Self::FrameSubmit
            | Self::ReflowLineRange
            | Self::AccessibilityTreeUpdate => true,
        }
    }

    /// The canonical stable ordering of all hook ids.
    pub const ALL: &'static [Hook] = &[
        Hook::WarmStartToFirstPaint,
        Hook::FirstPaint,
        Hook::ScrollFrame,
        Hook::CaretMove,
        Hook::SelectionChange,
        Hook::ImeCompositionUpdate,
        Hook::FallbackGlyphResolution,
        Hook::MultiMonitorScaleChange,
        Hook::AtlasShardRebind,
        Hook::AtlasEviction,
        Hook::FrameSubmit,
        Hook::ReflowLineRange,
        Hook::DegradedRendererBanner,
        Hook::AccessibilityTreeUpdate,
    ];
}

/// A single timing mark.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingMark {
    pub hook: Hook,
    pub tick: Tick,
    pub note: Option<String>,
}

/// Collects timing marks during a run.
#[derive(Debug, Default)]
pub struct TimingRecorder {
    marks: Vec<TimingMark>,
}

impl TimingRecorder {
    /// Creates a new recorder instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Records a mark for the given hook.
    pub fn mark(&mut self, hook: Hook, tick: Tick) {
        self.marks.push(TimingMark {
            hook,
            tick,
            note: None,
        });
    }

    /// Records a mark for the given hook with an attached note.
    pub fn mark_with_note(&mut self, hook: Hook, tick: Tick, note: impl Into<String>) {
        self.marks.push(TimingMark {
            hook,
            tick,
            note: Some(note.into()),
        });
    }

    /// Returns the recorded marks in arrival order.
    pub fn marks(&self) -> &[TimingMark] {
        &self.marks
    }

    /// Drains the recorded marks.
    pub fn drain(&mut self) -> Vec<TimingMark> {
        std::mem::take(&mut self.marks)
    }

    /// Returns the number of recorded marks.
    pub fn len(&self) -> usize {
        self.marks.len()
    }

    /// Returns true if the recorder is empty.
    pub fn is_empty(&self) -> bool {
        self.marks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct CountingClock(std::cell::Cell<u64>);

    impl Clock for CountingClock {
        fn now(&self) -> Tick {
            let current = self.0.get();
            self.0.set(current + 1);
            Tick(current)
        }
    }

    #[test]
    fn hook_ids_are_stable_strings() {
        assert_eq!(Hook::FrameSubmit.id(), "frame_submit");
        assert!(Hook::FrameSubmit.is_protected_hot_path());
        assert!(!Hook::AtlasEviction.is_protected_hot_path());
    }

    #[test]
    fn recorder_preserves_arrival_order() {
        let clock = CountingClock::default();
        let mut recorder = TimingRecorder::new();
        recorder.mark(Hook::FrameSubmit, clock.now());
        recorder.mark_with_note(Hook::FirstPaint, clock.now(), "note");
        let marks = recorder.marks();
        assert_eq!(marks.len(), 2);
        assert_eq!(marks[0].hook, Hook::FrameSubmit);
        assert_eq!(marks[1].hook, Hook::FirstPaint);
        assert_eq!(marks[1].note.as_deref(), Some("note"));
    }
}

