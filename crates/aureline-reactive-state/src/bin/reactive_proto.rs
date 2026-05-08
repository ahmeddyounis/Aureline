//! Reactive-state prototype smoke binary.
//!
//! Runs the frozen scenario table in
//! `crates/aureline-reactive-state/src/harness.rs` and emits
//! reviewable invalidation-trace records. Counts only, no
//! wall-clock times, so the committed artifacts under
//! `artifacts/state/invalidation_trace_examples/` stay byte-stable
//! across hosts.
//!
//! Modes:
//!
//! - Default: emit the aggregate report as one JSON blob (stdout
//!   or `--emit PATH`).
//! - `--emit-scenarios DIR`: emit one `<label>.json` per scenario
//!   into `DIR` plus a top-level `aggregate.json`.
//! - `--emit-order-audits DIR`: emit one condensed ordering audit
//!   per file into `DIR` plus a top-level `aggregate.json`.
//!
//! Usage:
//!   reactive_proto [--emit PATH] [--emit-scenarios DIR]
//!                 [--emit-order-audits DIR]

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use aureline_reactive_state::harness::{report_to_json, run_harness, scenario_to_json};
use aureline_reactive_state::verification::{
    invalidation_order_audit_to_json, invalidation_order_audits_to_json,
    run_invalidation_order_audits,
};

#[derive(Debug, Default)]
struct Args {
    emit: Option<PathBuf>,
    emit_scenarios: Option<PathBuf>,
    emit_order_audits: Option<PathBuf>,
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
                out.emit_scenarios =
                    Some(PathBuf::from(iter.next().ok_or_else(|| {
                        "--emit-scenarios requires a directory".to_owned()
                    })?));
            }
            "--emit-order-audits" => {
                out.emit_order_audits =
                    Some(PathBuf::from(iter.next().ok_or_else(|| {
                        "--emit-order-audits requires a directory".to_owned()
                    })?));
            }
            "--help" | "-h" => return Err(usage()),
            other => return Err(format!("unknown argument: {other}\n\n{}", usage())),
        }
    }
    Ok(out)
}

fn usage() -> String {
    "reactive_proto — reactive-state / subscription-envelope prototype smoke harness\n\n\
     Usage:\n\
     \treactive_proto [--emit PATH] [--emit-scenarios DIR] [--emit-order-audits DIR]\n\n\
     Defaults:\n\
     \t--emit              <stdout> when --emit-scenarios is unset\n\
     \t--emit-scenarios    off (aggregate-only emission)\n\
     \t--emit-order-audits off (no condensed order-audit emission)\n"
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
            let _ = writeln!(io::stderr(), "reactive_proto: {err}");
            ExitCode::from(1)
        }
    }
}

fn run(args: &Args) -> Result<(), String> {
    let report = run_harness();
    let aggregate_json = report_to_json(&report);
    let order_audits = run_invalidation_order_audits();
    let order_audits_json = invalidation_order_audits_to_json(&order_audits);

    if let Some(dir) = &args.emit_scenarios {
        fs::create_dir_all(dir).map_err(|e| format!("creating {dir:?}: {e}"))?;
        for scenario in &report.scenarios {
            let path = dir.join(format!("{}.json", scenario.label));
            let json = scenario_to_json(scenario);
            write_file(&path, json.as_bytes()).map_err(|e| format!("writing {path:?}: {e}"))?;
        }
        let agg_path = dir.join("aggregate.json");
        write_file(&agg_path, aggregate_json.as_bytes())
            .map_err(|e| format!("writing {agg_path:?}: {e}"))?;
    }

    if let Some(dir) = &args.emit_order_audits {
        fs::create_dir_all(dir).map_err(|e| format!("creating {dir:?}: {e}"))?;
        for audit in &order_audits {
            let path = dir.join(format!("{}.json", audit.file_stem));
            let json = invalidation_order_audit_to_json(audit);
            write_file(&path, json.as_bytes()).map_err(|e| format!("writing {path:?}: {e}"))?;
        }
        let agg_path = dir.join("aggregate.json");
        write_file(&agg_path, order_audits_json.as_bytes())
            .map_err(|e| format!("writing {agg_path:?}: {e}"))?;
    }

    match &args.emit {
        Some(path) => write_file(path, aggregate_json.as_bytes())
            .map_err(|e| format!("writing {path:?}: {e}"))?,
        None if args.emit_scenarios.is_none() && args.emit_order_audits.is_none() => {
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
