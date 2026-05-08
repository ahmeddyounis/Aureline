//! Buffer-smoke bench binary.
//!
//! Drives the prototype buffer through the frozen scenario table and
//! emits a structural metrics JSON record. Counts only, no wall-clock
//! times, so the committed seed under `artifacts/buffer/` is byte-
//! stable across hosts. The benchmark lab wraps this with wall-clock
//! timing when it scores against protected-hot-path budgets.
//!
//! Usage:
//!   bench_buffer [--emit PATH] [--emit-undo-examples DIR]
//!
//! Defaults:
//!   --emit                 <stdout>
//!   --emit-undo-examples   (skipped)

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use aureline_bench::buffer::{
    render_undo_example, report_to_json, run_harness, undo_example_labels,
};

#[derive(Debug, Default)]
struct Args {
    emit: Option<PathBuf>,
    emit_undo_examples: Option<PathBuf>,
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
            "--emit-undo-examples" => {
                out.emit_undo_examples =
                    Some(PathBuf::from(iter.next().ok_or_else(|| {
                        "--emit-undo-examples requires a directory".to_owned()
                    })?));
            }
            "--help" | "-h" => return Err(usage()),
            other => return Err(format!("unknown argument: {other}\n\n{}", usage())),
        }
    }
    Ok(out)
}

fn usage() -> String {
    "bench_buffer — prototype buffer smoke harness\n\n\
     Usage:\n\
     \tbench_buffer [--emit PATH] [--emit-undo-examples DIR]\n\n\
     Defaults:\n\
     \t--emit                  <stdout>\n\
     \t--emit-undo-examples    <skipped>\n"
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
            let _ = writeln!(io::stderr(), "bench_buffer: {err}");
            ExitCode::from(1)
        }
    }
}

fn run(args: &Args) -> Result<(), String> {
    let report = run_harness();
    let json = report_to_json(&report);
    match &args.emit {
        Some(path) => {
            write_file(path, json.as_bytes()).map_err(|e| format!("writing {:?}: {e}", path))?
        }
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            handle
                .write_all(json.as_bytes())
                .map_err(|e| format!("writing stdout: {e}"))?;
        }
    }
    if let Some(dir) = &args.emit_undo_examples {
        for label in undo_example_labels() {
            let text = render_undo_example(label)
                .ok_or_else(|| format!("unknown undo-example label {label:?}"))?;
            let path = dir.join(format!("{label}.txt"));
            write_file(&path, text.as_bytes()).map_err(|e| format!("writing {:?}: {e}", path))?;
        }
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
