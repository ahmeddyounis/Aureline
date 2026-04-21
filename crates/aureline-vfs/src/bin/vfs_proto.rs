//! VFS prototype smoke binary.
//!
//! Runs the frozen scenario table and emits reviewable save-plan
//! records. Counts only, no wall-clock times, so the committed
//! seeds under `artifacts/fs/save_plan_examples/` stay byte-stable
//! across hosts.
//!
//! Modes:
//!
//! - Default: emit the aggregate report as one JSON blob (stdout
//!   or `--emit PATH`).
//! - `--emit-scenarios DIR`: emit one `<label>.json` record per
//!   scenario into `DIR` (plus a top-level `aggregate.json`).
//!
//! Usage:
//!   vfs_proto [--emit PATH] [--emit-scenarios DIR]

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use aureline_vfs::harness::{report_to_json, run_harness, scenario_to_json};

#[derive(Debug, Default)]
struct Args {
    emit: Option<PathBuf>,
    emit_scenarios: Option<PathBuf>,
}

fn parse_args(raw: &[String]) -> Result<Args, String> {
    let mut out = Args::default();
    let mut iter = raw.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--emit" => {
                out.emit = Some(PathBuf::from(
                    iter.next()
                        .ok_or_else(|| "--emit requires a path".to_owned())?,
                ));
            }
            "--emit-scenarios" => {
                out.emit_scenarios = Some(PathBuf::from(
                    iter.next()
                        .ok_or_else(|| "--emit-scenarios requires a directory".to_owned())?,
                ));
            }
            "--help" | "-h" => return Err(usage()),
            other => return Err(format!("unknown argument: {other}\n\n{}", usage())),
        }
    }
    Ok(out)
}

fn usage() -> String {
    "vfs_proto — VFS / save prototype smoke harness\n\n\
     Usage:\n\
     \tvfs_proto [--emit PATH] [--emit-scenarios DIR]\n\n\
     Defaults:\n\
     \t--emit              <stdout> when --emit-scenarios is unset\n\
     \t--emit-scenarios    off (aggregate-only emission)\n"
        .to_owned()
}

fn main() -> ExitCode {
    let argv: Vec<String> = env::args().collect();
    let args = match parse_args(&argv) {
        Ok(a) => a,
        Err(message) => {
            let _ = writeln!(io::stderr(), "{message}");
            return ExitCode::from(2);
        }
    };
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            let _ = writeln!(io::stderr(), "vfs_proto: {err}");
            ExitCode::from(1)
        }
    }
}

fn run(args: &Args) -> Result<(), String> {
    let report = run_harness();
    let aggregate_json = report_to_json(&report);

    if let Some(dir) = &args.emit_scenarios {
        fs::create_dir_all(dir).map_err(|e| format!("creating {dir:?}: {e}"))?;
        for scenario in &report.scenarios {
            let path = dir.join(format!("{}.json", scenario.label));
            let json = scenario_to_json(scenario);
            write_file(&path, json.as_bytes())
                .map_err(|e| format!("writing {path:?}: {e}"))?;
        }
        let agg_path = dir.join("aggregate.json");
        write_file(&agg_path, aggregate_json.as_bytes())
            .map_err(|e| format!("writing {agg_path:?}: {e}"))?;
    }

    match &args.emit {
        Some(path) => write_file(path, aggregate_json.as_bytes())
            .map_err(|e| format!("writing {path:?}: {e}"))?,
        None if args.emit_scenarios.is_none() => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            handle
                .write_all(aggregate_json.as_bytes())
                .map_err(|e| format!("writing stdout: {e}"))?;
        }
        None => {}
    }
    Ok(())
}

fn write_file(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, bytes)
}
