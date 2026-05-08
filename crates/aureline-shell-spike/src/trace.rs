//! Trace-record format for the fixture scene.
//!
//! Each hook that fires becomes one record. Records are plain JSON with
//! a fixed key order so the samples committed under
//! `artifacts/render/spike_trace_samples/` stay diffable.

use std::fmt::Write as _;

use crate::capabilities::quote;
use crate::fixture_scene::{FixtureRunResult, FIXTURE_SCENE_ID};
use crate::frame_timing::TimingMark;

/// Sample bundle: one per fixture run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceSample {
    pub scene_id: &'static str,
    pub marks: Vec<TimingMark>,
}

impl TraceSample {
    pub fn from_run(run: &FixtureRunResult) -> Self {
        Self {
            scene_id: run.scene_id,
            marks: run.marks.clone(),
        }
    }

    /// Render as deterministic pretty JSON.
    pub fn to_json(&self) -> String {
        let mut out = String::new();
        out.push_str("{\n");
        writeln_kv_string(&mut out, 1, "scene_id", self.scene_id, false);
        writeln_kv_string(&mut out, 1, "schema", "shell_spike.trace_v1", false);

        indent(&mut out, 1);
        out.push_str("\"marks\": [\n");
        for (i, mark) in self.marks.iter().enumerate() {
            let last = i + 1 == self.marks.len();
            indent(&mut out, 2);
            out.push_str("{\n");
            writeln_kv_string(&mut out, 3, "hook", mark.hook.name(), false);
            writeln_kv_number(&mut out, 3, "tick", mark.tick.0, false);
            let note_value = match &mark.note {
                Some(note) => quote(note),
                None => "null".to_owned(),
            };
            indent(&mut out, 3);
            let _ = write!(out, "\"note\": {note_value}\n");
            indent(&mut out, 2);
            if last {
                out.push_str("}\n");
            } else {
                out.push_str("},\n");
            }
        }
        indent(&mut out, 1);
        out.push_str("]\n");
        out.push_str("}\n");
        out
    }
}

/// Build one trace-sample file per label in the fixture script plus an
/// aggregate "full_scene" sample. The per-label samples are what the
/// benchmark lab consumes to check a single hook in isolation.
pub fn per_label_samples(run: &FixtureRunResult) -> Vec<(String, TraceSample)> {
    let mut out: Vec<(String, TraceSample)> = Vec::new();
    for frame in &run.frames {
        if let Some(mark) = &frame.mark {
            let sample = TraceSample {
                scene_id: FIXTURE_SCENE_ID,
                marks: vec![mark.clone()],
            };
            let filename = format!("{}.json", frame.label);
            out.push((filename, sample));
        }
    }
    out.push(("full_scene.json".to_owned(), TraceSample::from_run(run)));
    out
}

fn indent(out: &mut String, depth: usize) {
    for _ in 0..depth {
        out.push_str("  ");
    }
}

fn writeln_kv_string(out: &mut String, depth: usize, key: &str, value: &str, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\": {}", quote(value));
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

fn writeln_kv_number(out: &mut String, depth: usize, key: &str, value: u64, last: bool) {
    indent(out, depth);
    let _ = write!(out, "\"{key}\": {value}");
    if last {
        out.push('\n');
    } else {
        out.push_str(",\n");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::fixture_scene::run;
    use crate::frame_timing::CountingClock;
    use crate::zones::ShellFrame;

    #[test]
    fn trace_sample_round_trips_every_mark() {
        let result = run(&ShellFrame::fixture(), &CountingClock::new());
        let sample = TraceSample::from_run(&result);
        let json = sample.to_json();
        for mark in &result.marks {
            assert!(json.contains(mark.hook.name()));
        }
    }

    #[test]
    fn per_label_samples_include_full_scene_plus_one_per_labeled_mark() {
        let result = run(&ShellFrame::fixture(), &CountingClock::new());
        let samples = per_label_samples(&result);
        let names: Vec<_> = samples.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"full_scene.json"));
        assert!(names.contains(&"warm_start_first_paint.json"));
        assert!(names.contains(&"scale_change.json"));
    }
}
