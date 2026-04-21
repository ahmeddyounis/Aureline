//! Spike entry point.
//!
//! The binary runs the fixture scene from `aureline_shell_spike` and
//! emits the capability manifest plus per-label trace samples. It can
//! run in three modes:
//!
//! * `--print` (default) — print the manifest JSON to stdout, then
//!   print the per-label trace samples one per blank-line-separated
//!   block. Useful for inspecting changes during development.
//! * `--emit-artifacts <dir>` — write the manifest to
//!   `<dir>/spike_capabilities.json` and the trace samples to
//!   `<dir>/spike_trace_samples/<label>.json`. This is how the
//!   committed fixtures under `artifacts/render/` are regenerated.
//! * `--scene-only` — just run the fixture scene and print the
//!   resulting hooks-fired list. Used by smoke-test invocations.
//!
//! The binary intentionally does not open a native window in this spike
//! revision. Wiring `winit` and a software-render or `wgpu`-backed
//! surface is a follow-up task; the seams in `lib.rs` (input_path,
//! render_path, frame_timing, zones) are the contract that wiring will
//! satisfy. See `docs/design/shell_spike_composition_notes.md` for the
//! composition-path notes that cover damage entry points, the text-layer
//! / overlay-layer boundary, placeholder surface ownership, and the
//! trace IDs emitted at startup and at input/render boundaries.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use aureline_shell_spike::capabilities::{Backend, CapabilityManifest};
use aureline_shell_spike::fixture_scene::{run, FixtureRunResult, FIXTURE_SCENE_ID};
use aureline_shell_spike::frame_timing::CountingClock;
use aureline_shell_spike::trace::per_label_samples;
use aureline_shell_spike::zones::ShellFrame;

#[derive(Debug)]
enum Mode {
    Print,
    EmitArtifacts(PathBuf),
    SceneOnly,
}

fn parse_mode(args: &[String]) -> Result<Mode, String> {
    let mut iter = args.iter().skip(1);
    let mut mode = Mode::Print;
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--print" => mode = Mode::Print,
            "--scene-only" => mode = Mode::SceneOnly,
            "--emit-artifacts" => {
                let dir = iter
                    .next()
                    .ok_or_else(|| "--emit-artifacts requires a directory path".to_owned())?;
                mode = Mode::EmitArtifacts(PathBuf::from(dir.as_str()));
            }
            "--help" | "-h" => return Err(usage()),
            other => return Err(format!("unknown argument: {other}\n\n{}", usage())),
        }
    }
    Ok(mode)
}

fn usage() -> String {
    "shell_spike — Aureline shell spike\n\n\
     Usage:\n\
     \tshell_spike [--print]\n\
     \tshell_spike --emit-artifacts <dir>\n\
     \tshell_spike --scene-only\n"
        .to_owned()
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let mode = match parse_mode(&args) {
        Ok(m) => m,
        Err(message) => {
            let _ = writeln!(io::stderr(), "{message}");
            return ExitCode::from(2);
        }
    };

    let frame = ShellFrame::fixture();
    let result = run(&frame, &CountingClock::new());

    match mode {
        Mode::Print => {
            let manifest = CapabilityManifest::new(Backend::Headless, frame, FIXTURE_SCENE_ID);
            print!("{}", manifest.to_json());
            for (label, sample) in per_label_samples(&result) {
                println!();
                println!("// {label}");
                print!("{}", sample.to_json());
            }
            ExitCode::SUCCESS
        }
        Mode::EmitArtifacts(dir) => match write_artifacts(&dir, &frame, &result) {
            Ok(()) => ExitCode::SUCCESS,
            Err(err) => {
                let _ = writeln!(io::stderr(), "shell_spike: {err}");
                ExitCode::from(1)
            }
        },
        Mode::SceneOnly => {
            for hook in result.hooks_fired() {
                println!("{}", hook.name());
            }
            ExitCode::SUCCESS
        }
    }
}

fn write_artifacts(
    dir: &Path,
    frame: &ShellFrame,
    result: &FixtureRunResult,
) -> io::Result<()> {
    fs::create_dir_all(dir)?;
    let manifest =
        CapabilityManifest::new(Backend::Headless, frame.clone(), FIXTURE_SCENE_ID);
    let manifest_path = dir.join("spike_capabilities.json");
    fs::write(&manifest_path, manifest.to_json())?;

    let trace_dir = dir.join("spike_trace_samples");
    fs::create_dir_all(&trace_dir)?;
    for (filename, sample) in per_label_samples(result) {
        fs::write(trace_dir.join(filename), sample.to_json())?;
    }
    Ok(())
}
