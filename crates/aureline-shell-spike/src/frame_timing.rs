//! Named frame-timing marks.
//!
//! The spike records a mark each time a hook fires. Marks are the shape
//! the benchmark lab consumes. The recorder is injectable so tests use a
//! deterministic monotonic counter and the binary uses `Instant`.

use crate::hooks::Hook;

/// A monotonically-increasing timestamp, measured in "ticks". The unit
/// is opaque on purpose: the benchmark lab maps ticks to wall time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tick(pub u64);

/// The clock interface the timing recorder uses. Exposed as a trait so
/// tests inject a deterministic counter.
pub trait Clock {
    fn now(&self) -> Tick;
}

/// Counts-up-by-one clock. Used by the fixture-scene tests.
#[derive(Debug, Default)]
pub struct CountingClock {
    counter: std::cell::Cell<u64>,
}

impl CountingClock {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Clock for CountingClock {
    fn now(&self) -> Tick {
        let t = self.counter.get();
        self.counter.set(t + 1);
        Tick(t)
    }
}

/// Wall clock backed by [`std::time::Instant`]. Used by the binary.
#[derive(Debug)]
pub struct WallClock {
    origin: std::time::Instant,
}

impl WallClock {
    pub fn new() -> Self {
        Self {
            origin: std::time::Instant::now(),
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

/// A single timing mark. The pair of (hook, tick) is the minimum the
/// benchmark lab needs to compute latencies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingMark {
    pub hook: Hook,
    pub tick: Tick,
    /// Optional per-mark note. Free-form; not structured.
    pub note: Option<String>,
}

/// Collects marks during a scene run. Not thread-safe; the shell runs
/// on a single render thread.
#[derive(Debug, Default)]
pub struct TimingRecorder {
    marks: Vec<TimingMark>,
}

impl TimingRecorder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn mark(&mut self, hook: Hook, tick: Tick) {
        self.marks.push(TimingMark {
            hook,
            tick,
            note: None,
        });
    }

    pub fn mark_with_note(&mut self, hook: Hook, tick: Tick, note: impl Into<String>) {
        self.marks.push(TimingMark {
            hook,
            tick,
            note: Some(note.into()),
        });
    }

    pub fn marks(&self) -> &[TimingMark] {
        &self.marks
    }

    pub fn into_marks(self) -> Vec<TimingMark> {
        self.marks
    }

    pub fn len(&self) -> usize {
        self.marks.len()
    }

    pub fn is_empty(&self) -> bool {
        self.marks.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_clock_produces_monotonic_ticks() {
        let clock = CountingClock::new();
        assert_eq!(clock.now(), Tick(0));
        assert_eq!(clock.now(), Tick(1));
        assert_eq!(clock.now(), Tick(2));
    }

    #[test]
    fn recorder_preserves_arrival_order() {
        let mut recorder = TimingRecorder::new();
        recorder.mark(Hook::FrameSubmit, Tick(10));
        recorder.mark(Hook::CaretMove, Tick(3));
        let marks = recorder.marks();
        assert_eq!(marks[0].hook, Hook::FrameSubmit);
        assert_eq!(marks[1].hook, Hook::CaretMove);
    }
}
