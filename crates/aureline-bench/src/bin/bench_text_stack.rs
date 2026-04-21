//! Text-stack bench binary.
//!
//! Drives the prototype text layer against the shaping smoke corpus
//! and emits a structural metrics JSON record. Structural means
//! "counts only, no wall-clock timings" so the committed seed under
//! `artifacts/bench/` can be diffed across machines. A future
//! benchmark-lab wrapper layers timing on top of these counts.
//!
//! Usage:
//!   bench_text_stack [--corpus PATH] [--iterations N] [--emit PATH]
//!
//! Defaults: corpus = fixtures/text/shaping_smoke_cases.txt;
//! iterations = 2 (so shape+raster caches are exercised); emit = stdout.

use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use aureline_bench::text_stack::{parse_corpus, report_to_json, run_harness};

#[derive(Debug)]
struct Args {
    corpus: PathBuf,
    iterations: u32,
    emit: Option<PathBuf>,
}

fn parse_args(raw: &[String]) -> Result<Args, String> {
    let mut corpus = PathBuf::from("fixtures/text/shaping_smoke_cases.txt");
    let mut iterations: u32 = 2;
    let mut emit: Option<PathBuf> = None;
    let mut iter = raw.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--corpus" => {
                corpus = PathBuf::from(
                    iter.next()
                        .ok_or_else(|| "--corpus requires a path".to_owned())?,
                );
            }
            "--iterations" => {
                let n = iter
                    .next()
                    .ok_or_else(|| "--iterations requires a positive integer".to_owned())?;
                iterations = n
                    .parse::<u32>()
                    .map_err(|e| format!("invalid --iterations value {n:?}: {e}"))?;
                if iterations == 0 {
                    return Err("--iterations must be >= 1".to_owned());
                }
            }
            "--emit" => {
                emit = Some(PathBuf::from(
                    iter.next()
                        .ok_or_else(|| "--emit requires a path".to_owned())?,
                ));
            }
            "--help" | "-h" => return Err(usage()),
            other => return Err(format!("unknown argument: {other}\n\n{}", usage())),
        }
    }
    Ok(Args {
        corpus,
        iterations,
        emit,
    })
}

fn usage() -> String {
    "bench_text_stack — prototype text-stack smoke harness\n\n\
     Usage:\n\
     \tbench_text_stack [--corpus PATH] [--iterations N] [--emit PATH]\n\n\
     Defaults:\n\
     \t--corpus      fixtures/text/shaping_smoke_cases.txt\n\
     \t--iterations  2\n\
     \t--emit        <stdout>\n"
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
            let _ = writeln!(io::stderr(), "bench_text_stack: {err}");
            ExitCode::from(1)
        }
    }
}

fn run(args: &Args) -> Result<(), String> {
    let contents = fs::read_to_string(&args.corpus)
        .map_err(|e| format!("reading corpus {:?}: {e}", args.corpus))?;
    let cases = parse_corpus(&contents).map_err(|e| format!("corpus parse error: {e}"))?;
    if cases.is_empty() {
        return Err("corpus is empty; nothing to shape".to_owned());
    }
    let report = run_harness(&cases, args.iterations);
    let json = report_to_json(&report);
    match &args.emit {
        Some(path) => write_file(path, json.as_bytes())
            .map_err(|e| format!("writing {:?}: {e}", path)),
        None => {
            let stdout = io::stdout();
            let mut handle = stdout.lock();
            handle
                .write_all(json.as_bytes())
                .map_err(|e| format!("writing stdout: {e}"))
        }
    }
}

fn write_file(path: &Path, bytes: &[u8]) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    fs::write(path, bytes)
}
