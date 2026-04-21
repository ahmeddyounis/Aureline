//! Large-file prototype smoke binary.
//!
//! Materialises the embedded fixtures into a scratch directory,
//! runs the frozen scenario table, and emits a structural metrics
//! JSON record. Counts only, no wall-clock times, so the
//! committed seed under
//! `artifacts/bench/large_file_proto_metrics.json` is byte-stable
//! across hosts.
//!
//! Usage:
//!   largefile_proto [--emit PATH] [--scratch-dir DIR]
//!
//! Defaults:
//!   --emit          <stdout>
//!   --scratch-dir   <a fresh temp dir>

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use aureline_largefile_proto::harness::{
    report_to_json, run_harness, write_fixtures,
};

#[derive(Debug, Default)]
struct Args {
    emit: Option<PathBuf>,
    scratch_dir: Option<PathBuf>,
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
            "--scratch-dir" => {
                out.scratch_dir = Some(PathBuf::from(
                    iter.next()
                        .ok_or_else(|| "--scratch-dir requires a directory".to_owned())?,
                ));
            }
            "--help" | "-h" => return Err(usage()),
            other => return Err(format!("unknown argument: {other}\n\n{}", usage())),
        }
    }
    Ok(out)
}

fn usage() -> String {
    "largefile_proto — large-file path prototype smoke harness\n\n\
     Usage:\n\
     \tlargefile_proto [--emit PATH] [--scratch-dir DIR]\n\n\
     Defaults:\n\
     \t--emit          <stdout>\n\
     \t--scratch-dir   <fresh temp dir>\n"
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
            let _ = writeln!(io::stderr(), "largefile_proto: {err}");
            ExitCode::from(1)
        }
    }
}

fn run(args: &Args) -> Result<(), String> {
    let (scratch, owned) = match &args.scratch_dir {
        Some(p) => (p.clone(), false),
        None => (fresh_scratch_dir(), true),
    };
    write_fixtures(&scratch).map_err(|e| format!("writing fixtures to {scratch:?}: {e}"))?;
    let report = run_harness(&scratch).map_err(|e| format!("running harness: {e}"))?;
    let json = report_to_json(&report);
    match &args.emit {
        Some(path) => {
            write_file(path, json.as_bytes()).map_err(|e| format!("writing {path:?}: {e}"))?
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            handle
                .write_all(json.as_bytes())
                .map_err(|e| format!("writing stdout: {e}"))?;
        }
    }
    if owned {
        let _ = fs::remove_dir_all(&scratch);
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

fn fresh_scratch_dir() -> PathBuf {
    let mut p = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    p.push(format!(
        "aureline-largefile-proto-bench-{nanos}-{}",
        std::process::id()
    ));
    p
}
