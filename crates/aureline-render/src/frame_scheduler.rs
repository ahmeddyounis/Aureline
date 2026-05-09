//! Frame scheduling and queue plumbing.
//!
//! The shell and editor invalidate surfaces by enqueueing damage events into a
//! shared draw queue. The frame scheduler coalesces that work and decides when
//! a window redraw should be requested.

use crate::draw_queue::{CompositedFrame, DamageEvent, DrawQueue};
use crate::hooks::{Clock, Hook, TimingMark, TimingRecorder};

/// Decision produced by the frame scheduler for the current event-loop turn.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameSchedulerDecision {
    /// The window should request a redraw.
    RequestRedraw,
    /// No redraw is required right now.
    NoAction,
    /// The window is currently occluded; defer draw work.
    Occluded,
}

/// Snapshot of frame-scheduler state for diagnostics and tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FrameSchedulerStats {
    pub frames_submitted: u64,
    pub dropped_events: u64,
    pub pending_events: usize,
    pub occluded: bool,
}

/// Owns frame-scheduling state and the draw queue consumed by the renderer.
#[derive(Debug, Default)]
pub struct FrameScheduler {
    queue: DrawQueue,
    recorder: TimingRecorder,
    redraw_requested: bool,
    occluded: bool,
    frames_submitted: u64,
    first_paint_marked: bool,
}

impl FrameScheduler {
    /// Creates a new scheduler with a default-bounded queue.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enqueues a damage event that requires a future redraw.
    pub fn invalidate(&mut self, event: DamageEvent) {
        self.queue.push(event);
    }

    /// Records that the window became occluded or visible again.
    pub fn set_occluded(&mut self, occluded: bool, _clock: &impl Clock) {
        if self.occluded == occluded {
            return;
        }
        self.occluded = occluded;
        if !occluded {
            self.redraw_requested = false;
        }
    }

    /// Returns the decision for whether a redraw should be requested.
    pub fn decision(&mut self) -> FrameSchedulerDecision {
        if self.occluded {
            return FrameSchedulerDecision::Occluded;
        }
        if self.queue.pending_len() == 0 || self.redraw_requested {
            return FrameSchedulerDecision::NoAction;
        }
        self.redraw_requested = true;
        FrameSchedulerDecision::RequestRedraw
    }

    /// Begins a new frame by draining the pending queue.
    ///
    /// Call this from the window's redraw handler. When the window is occluded
    /// the scheduler returns `None` so callers can skip GPU submission.
    pub fn begin_frame(&mut self) -> Option<CompositedFrame> {
        self.redraw_requested = false;
        if self.occluded {
            return None;
        }
        if self.queue.pending_len() == 0 {
            return None;
        }
        Some(self.queue.take_frame())
    }

    /// Records a successful frame submission.
    pub fn note_frame_submitted(&mut self, clock: &impl Clock) {
        if !self.first_paint_marked {
            self.first_paint_marked = true;
            self.recorder.mark(Hook::FirstPaint, clock.now());
        }
        self.frames_submitted = self.frames_submitted.saturating_add(1);
        self.recorder.mark(Hook::FrameSubmit, clock.now());
    }

    /// Records an input or render hook timing mark.
    ///
    /// This is used by higher layers (shell, editor) to emit protected-path
    /// hook marks without minting parallel recorder implementations.
    pub fn mark_hook(&mut self, hook: Hook, clock: &impl Clock) {
        self.recorder.mark(hook, clock.now());
    }

    /// Records a hook timing mark with an attached note.
    pub fn mark_hook_with_note(&mut self, hook: Hook, clock: &impl Clock, note: impl Into<String>) {
        self.recorder.mark_with_note(hook, clock.now(), note);
    }

    /// Records that the renderer entered an explicit degraded mode.
    pub fn note_degraded_renderer(&mut self, reason: impl Into<String>, clock: &impl Clock) {
        self.recorder
            .mark_with_note(Hook::DegradedRendererBanner, clock.now(), reason);
    }

    /// Returns a snapshot of current scheduler state.
    pub fn stats(&self) -> FrameSchedulerStats {
        FrameSchedulerStats {
            frames_submitted: self.frames_submitted,
            dropped_events: self.queue.dropped_events(),
            pending_events: self.queue.pending_len(),
            occluded: self.occluded,
        }
    }

    /// Returns the recorded timing marks.
    pub fn marks(&self) -> &[TimingMark] {
        self.recorder.marks()
    }

    /// Drains recorded timing marks.
    pub fn drain_marks(&mut self) -> Vec<TimingMark> {
        self.recorder.drain()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::draw_queue::{CompositionLayerId, DamageClassId};

    #[derive(Debug, Default)]
    struct CountingClock(std::cell::Cell<u64>);

    impl Clock for CountingClock {
        fn now(&self) -> crate::hooks::Tick {
            let current = self.0.get();
            self.0.set(current + 1);
            crate::hooks::Tick(current)
        }
    }

    #[test]
    fn coalesces_multiple_invalidations_into_one_redraw_request() {
        let clock = CountingClock::default();
        let mut scheduler = FrameScheduler::new();
        scheduler.invalidate(DamageEvent::new(
            CompositionLayerId::WindowChromeBase,
            DamageClassId::WindowExposedRegionRefresh,
        ));
        scheduler.invalidate(DamageEvent::new(
            CompositionLayerId::TextAndDecoration,
            DamageClassId::ViewportResizeOrScaleChange,
        ));
        assert_eq!(scheduler.decision(), FrameSchedulerDecision::RequestRedraw);
        assert_eq!(scheduler.decision(), FrameSchedulerDecision::NoAction);
        let frame = scheduler.begin_frame().expect("frame should start");
        assert_eq!(frame.events.len(), 2);
        scheduler.note_frame_submitted(&clock);
        assert!(scheduler.marks().iter().any(|m| m.hook == Hook::FrameSubmit));
    }

    #[test]
    fn occlusion_defers_frame_begin() {
        let clock = CountingClock::default();
        let mut scheduler = FrameScheduler::new();
        scheduler.set_occluded(true, &clock);
        scheduler.invalidate(DamageEvent::new(
            CompositionLayerId::WindowChromeBase,
            DamageClassId::WindowExposedRegionRefresh,
        ));
        assert_eq!(scheduler.decision(), FrameSchedulerDecision::Occluded);
        assert!(scheduler.begin_frame().is_none());
    }

    #[test]
    fn degraded_renderer_reason_is_emitted_as_a_timing_mark() {
        let clock = CountingClock::default();
        let mut scheduler = FrameScheduler::new();
        scheduler.note_degraded_renderer("reason", &clock);
        let mark = scheduler
            .marks()
            .iter()
            .find(|m| m.hook == Hook::DegradedRendererBanner)
            .expect("expected degraded renderer mark");
        assert_eq!(mark.note.as_deref(), Some("reason"));
    }
}
