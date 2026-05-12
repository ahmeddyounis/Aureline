//! A single repeatable fixture scene.
//!
//! The scene is a deterministic sequence of input events with a
//! corresponding expected damage and timing trace. It is the only scene
//! the spike ships; later benchmark and conformance tasks extend the
//! idea, not this file.

use crate::frame_timing::{Clock, TimingMark, TimingRecorder};
use crate::hooks::Hook;
use crate::input_path::{
    dispatch, CaretMove, ImeComposition, InputAction, InputEvent, NamedKey, SelectionDelta,
};
use crate::render_path::{classify, CompositedFrame, DamageRecord};
use crate::zones::ShellFrame;

/// Canonical fixture scene id.
pub const FIXTURE_SCENE_ID: &str = "shell_spike.fixture_v1";

/// The sequence of input events the fixture scene plays. Each entry is
/// wrapped in a label so trace samples can name the step that produced
/// a hook.
#[derive(Debug, Clone, PartialEq)]
pub struct FixtureStep {
    pub label: &'static str,
    pub event: InputEvent,
}

/// The full fixture-scene script. Eight steps that collectively exercise
/// every hot-path hook with a deterministic ordering.
pub fn script() -> Vec<FixtureStep> {
    vec![
        FixtureStep {
            label: "warm_start_first_paint",
            event: InputEvent::TextInput("hello".to_owned()),
        },
        FixtureStep {
            label: "caret_left",
            event: InputEvent::KeyPress(NamedKey::ArrowLeft),
        },
        FixtureStep {
            label: "selection_click",
            event: InputEvent::MouseButton {
                button: crate::input_path::MouseButton::Left,
                pressed: true,
                x: 320,
                y: 120,
            },
        },
        FixtureStep {
            label: "ime_compose",
            event: InputEvent::ImeComposition(ImeComposition {
                text: "漢".to_owned(),
                caret_byte_offset: 3,
            }),
        },
        FixtureStep {
            label: "scroll_down",
            event: InputEvent::Scroll { dx: 0, dy: 3 },
        },
        FixtureStep {
            label: "scale_change",
            event: InputEvent::ScaleChange { new_scale: 2.0 },
        },
        FixtureStep {
            label: "text_burst_cjk",
            event: InputEvent::TextInput("字".to_owned()),
        },
        FixtureStep {
            label: "caret_right",
            event: InputEvent::KeyPress(NamedKey::ArrowRight),
        },
    ]
}

/// One step's resolved state. The recorder returns a vector of these so
/// the trace and the composited frame stay aligned.
#[derive(Debug, Clone, PartialEq)]
pub struct FixtureFrame {
    pub label: &'static str,
    pub action: InputAction,
    pub damage: Option<DamageRecord>,
    pub mark: Option<TimingMark>,
}

/// Run the fixture scene against the given frame and clock. Returns the
/// per-step resolved state plus an aggregate trace.
pub fn run(frame: &ShellFrame, clock: &impl Clock) -> FixtureRunResult {
    let mut recorder = TimingRecorder::new();
    let mut frames = Vec::new();
    // Warm-start-to-first-paint always fires before any input, per ADR 0002.
    recorder.mark_with_note(Hook::WarmStartToFirstPaint, clock.now(), "scene.begin");
    recorder.mark_with_note(Hook::FirstPaint, clock.now(), "scene.first_paint");

    for step in script() {
        let (action, hook) = dispatch(&step.event);
        let damage = classify(frame, &action);
        let mark = hook.map(|h| {
            let tick = clock.now();
            recorder.mark_with_note(h, tick, step.label);
            TimingMark {
                hook: h,
                tick,
                note: Some(step.label.to_owned()),
            }
        });
        frames.push(FixtureFrame {
            label: step.label,
            action,
            damage,
            mark,
        });
    }

    // Every scene ends with one compositor submission so the benchmark
    // lab sees a frame_submit to close the trace.
    recorder.mark_with_note(Hook::FrameSubmit, clock.now(), "scene.end");

    FixtureRunResult {
        scene_id: FIXTURE_SCENE_ID,
        frames,
        marks: recorder.into_marks(),
    }
}

/// Resolved state of one fixture run.
#[derive(Debug, Clone, PartialEq)]
pub struct FixtureRunResult {
    pub scene_id: &'static str,
    pub frames: Vec<FixtureFrame>,
    pub marks: Vec<TimingMark>,
}

impl FixtureRunResult {
    /// Build a composited frame from the recorded damage. The frame
    /// index is set to 0 because the fixture scene is single-frame;
    /// coalescing many scenes is the benchmark lab's job.
    pub fn as_composited_frame(&self) -> CompositedFrame {
        let mut cf = CompositedFrame::new(0);
        for frame in &self.frames {
            if let Some(dmg) = frame.damage.clone() {
                cf.push(dmg);
            }
        }
        cf
    }

    pub fn hooks_fired(&self) -> Vec<Hook> {
        self.marks.iter().map(|m| m.hook).collect()
    }
}

/// A structural fingerprint of the fixture run. Two runs of the same
/// scene against the same frame MUST produce identical fingerprints;
/// this is the test the repeatability suite relies on.
pub fn structural_fingerprint(result: &FixtureRunResult) -> String {
    let mut out = String::new();
    out.push_str("scene=");
    out.push_str(result.scene_id);
    for frame in &result.frames {
        out.push_str(";step=");
        out.push_str(frame.label);
        out.push(':');
        out.push_str(match &frame.action {
            InputAction::InsertText(_) => "insert_text",
            InputAction::MoveCaret(CaretMove::Left) => "caret_left",
            InputAction::MoveCaret(CaretMove::Right) => "caret_right",
            InputAction::MoveCaret(CaretMove::Up) => "caret_up",
            InputAction::MoveCaret(CaretMove::Down) => "caret_down",
            InputAction::MoveCaret(CaretMove::LineStart) => "caret_line_start",
            InputAction::MoveCaret(CaretMove::LineEnd) => "caret_line_end",
            InputAction::ChangeSelection(SelectionDelta::Cleared) => "selection_clear",
            InputAction::ChangeSelection(SelectionDelta::ExtendedLeft) => "selection_left",
            InputAction::ChangeSelection(SelectionDelta::ExtendedRight) => "selection_right",
            InputAction::UpdateComposition(_) => "ime_update",
            InputAction::Scroll { .. } => "scroll",
            InputAction::ScaleChange { .. } => "scale",
            InputAction::None => "none",
        });
        if let Some(dmg) = &frame.damage {
            out.push_str("/layer=");
            out.push_str(dmg.layer.name());
            out.push_str("/hook=");
            out.push_str(dmg.hook.name());
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame_timing::CountingClock;

    #[test]
    fn scene_exercises_every_hot_path_hook() {
        let result = run(&ShellFrame::fixture(), &CountingClock::new());
        let fired: std::collections::BTreeSet<&'static str> =
            result.hooks_fired().into_iter().map(Hook::name).collect();
        for hook in Hook::ALL {
            if !hook.is_hot_path() {
                continue;
            }
            // These are not exercised by this fixture scene; they belong to
            // the benchmark lab and accessibility lanes.
            if matches!(
                *hook,
                Hook::FallbackGlyphResolution
                    | Hook::AtlasShardRebind
                    | Hook::AccessibilityTreeUpdate
            ) {
                continue;
            }
            assert!(
                fired.contains(hook.name()),
                "fixture scene never fires hot-path hook {}",
                hook.name()
            );
        }
    }

    #[test]
    fn scene_is_repeatable_within_a_process() {
        let frame = ShellFrame::fixture();
        let a = run(&frame, &CountingClock::new());
        let b = run(&frame, &CountingClock::new());
        assert_eq!(
            structural_fingerprint(&a),
            structural_fingerprint(&b),
            "two runs of the fixture scene diverged structurally"
        );
    }

    #[test]
    fn composited_frame_has_one_record_per_damaging_step() {
        let result = run(&ShellFrame::fixture(), &CountingClock::new());
        let cf = result.as_composited_frame();
        // Every scripted step produces damage; the Escape/Enter path would not,
        // but the script never sends those.
        assert_eq!(cf.damage.len(), result.frames.len());
    }
}
