//! Fixture-scene repeatability test.
//!
//! The spike's central promise is that the same input, same frame, and
//! same deterministic clock produce the same trace. If this test fails,
//! the fixture scene has acquired non-determinism and every downstream
//! trace-sample diff becomes suspect.

use aureline_shell_spike::capabilities::{Backend, CapabilityManifest};
use aureline_shell_spike::fixture_scene::{run, structural_fingerprint, FIXTURE_SCENE_ID};
use aureline_shell_spike::frame_timing::CountingClock;
use aureline_shell_spike::timing_trace::SpikeTimingTrace;
use aureline_shell_spike::trace::{per_label_samples, TraceSample};
use aureline_shell_spike::zones::ShellFrame;

#[test]
fn fixture_scene_is_byte_stable() {
    let frame = ShellFrame::fixture();
    let a = run(&frame, &CountingClock::new());
    let b = run(&frame, &CountingClock::new());

    assert_eq!(structural_fingerprint(&a), structural_fingerprint(&b));

    let sample_a = TraceSample::from_run(&a);
    let sample_b = TraceSample::from_run(&b);
    assert_eq!(
        sample_a.to_json(),
        sample_b.to_json(),
        "trace-sample JSON diverged across two identical fixture runs"
    );
}

#[test]
fn per_label_samples_are_byte_stable() {
    let frame = ShellFrame::fixture();
    let a = run(&frame, &CountingClock::new());
    let b = run(&frame, &CountingClock::new());

    let samples_a = per_label_samples(&a);
    let samples_b = per_label_samples(&b);
    assert_eq!(samples_a.len(), samples_b.len());
    for ((name_a, sample_a), (name_b, sample_b)) in samples_a.iter().zip(samples_b.iter()) {
        assert_eq!(name_a, name_b);
        assert_eq!(sample_a.to_json(), sample_b.to_json());
    }
}

#[test]
fn capability_manifest_is_byte_stable() {
    let a = CapabilityManifest::new(Backend::Headless, ShellFrame::fixture(), FIXTURE_SCENE_ID);
    let b = CapabilityManifest::new(Backend::Headless, ShellFrame::fixture(), FIXTURE_SCENE_ID);
    assert_eq!(a.to_json(), b.to_json());
}

#[test]
fn timing_trace_full_scene_is_byte_stable() {
    let frame = ShellFrame::fixture();
    let a = run(&frame, &CountingClock::new());
    let b = run(&frame, &CountingClock::new());

    let trace_a = SpikeTimingTrace::from_full_run(&a, Backend::Headless);
    let trace_b = SpikeTimingTrace::from_full_run(&b, Backend::Headless);
    assert_eq!(
        trace_a.to_json(),
        trace_b.to_json(),
        "spike_timing full-scene trace diverged across two identical fixture runs"
    );
}

#[test]
fn timing_trace_per_label_is_byte_stable() {
    let frame = ShellFrame::fixture();
    let a = run(&frame, &CountingClock::new());
    let b = run(&frame, &CountingClock::new());

    let traces_a = SpikeTimingTrace::per_label_plus_full(&a, Backend::Headless);
    let traces_b = SpikeTimingTrace::per_label_plus_full(&b, Backend::Headless);
    assert_eq!(traces_a.len(), traces_b.len());
    for ((name_a, trace_a), (name_b, trace_b)) in traces_a.iter().zip(traces_b.iter()) {
        assert_eq!(name_a, name_b);
        assert_eq!(trace_a.to_json(), trace_b.to_json());
    }
}

#[test]
fn timing_trace_carries_protected_path_for_every_mark() {
    let frame = ShellFrame::fixture();
    let result = run(&frame, &CountingClock::new());
    let trace = SpikeTimingTrace::from_full_run(&result, Backend::Headless);
    let json = trace.to_json();
    // Every mark in the fixture scene maps to one of these three
    // protected-path buckets; assert each shows up at least once.
    assert!(json.contains("\"protected_path\": \"startup\""));
    assert!(json.contains("\"protected_path\": \"first_paint\""));
    assert!(json.contains("\"protected_path\": \"input_to_paint\""));
    assert!(json.contains("\"protected_path\": \"render_submission\""));
}
