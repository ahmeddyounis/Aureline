//! Fixture-scene repeatability test.
//!
//! The spike's central promise is that the same input, same frame, and
//! same deterministic clock produce the same trace. If this test fails,
//! the fixture scene has acquired non-determinism and every downstream
//! trace-sample diff becomes suspect.

use aureline_shell_spike::capabilities::{Backend, CapabilityManifest};
use aureline_shell_spike::fixture_scene::{run, structural_fingerprint, FIXTURE_SCENE_ID};
use aureline_shell_spike::frame_timing::CountingClock;
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
    let a = CapabilityManifest::new(
        Backend::Headless,
        ShellFrame::fixture(),
        FIXTURE_SCENE_ID,
    );
    let b = CapabilityManifest::new(
        Backend::Headless,
        ShellFrame::fixture(),
        FIXTURE_SCENE_ID,
    );
    assert_eq!(a.to_json(), b.to_json());
}
