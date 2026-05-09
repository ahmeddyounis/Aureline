use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

use aureline_editor::{
    open_document, ClassificationPolicy, DocumentOpenDisposition, DocumentOpenOutcome,
    LargeFileViewerConfig,
};
use aureline_vfs::{LocalFilesystemRoot, VfsUri};

#[derive(Debug, Default)]
struct Args {
    path: Option<PathBuf>,
    threshold_bytes: Option<u64>,
    disposition: DocumentOpenDisposition,
    open_anyway: bool,
    find: Option<String>,
}

fn main() -> ExitCode {
    let argv: Vec<String> = env::args().collect();
    let args = match parse_args(&argv) {
        Ok(a) => a,
        Err(message) => {
            eprintln!("{message}");
            return ExitCode::from(2);
        }
    };

    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("large_file_guard: {err}");
            ExitCode::from(1)
        }
    }
}

fn run(args: &Args) -> Result<(), String> {
    let Some(path) = &args.path else {
        return Err(usage());
    };
    let uri =
        VfsUri::file_url_for_path(path).ok_or_else(|| format!("invalid file path: {path:?}"))?;

    let root = LocalFilesystemRoot::host_root("ws-local", "host");
    let mut policy = ClassificationPolicy::default();
    if let Some(threshold) = args.threshold_bytes {
        policy.large_file_size_threshold = threshold;
    }

    let viewer_config = LargeFileViewerConfig::default();
    let outcome = open_document(
        &root,
        &uri,
        &policy,
        viewer_config,
        args.disposition,
    )
    .map_err(|e| e.to_string())?;

    match outcome {
        DocumentOpenOutcome::Normal(doc) => {
            println!("mode: normal");
            println!(
                "canonical_uri: {}",
                doc.identity.canonical_filesystem_object.canonical_uri
            );
            println!("buffer_len: {}", doc.buffer.len());
            if let Some(info) = doc.large_file_override {
                let trigger = info
                    .decision
                    .trigger
                    .map(|t| t.as_str())
                    .unwrap_or("unknown");
                println!("large_file_override_trigger: {trigger}");
                println!("large_file_override_reason: {}", info.decision.reason);
            }
        }
        DocumentOpenOutcome::LargeFile(mut doc) => {
            let decision = doc.viewer.decision();
            println!("mode: large_file");
            println!(
                "canonical_uri: {}",
                doc.identity.canonical_filesystem_object.canonical_uri
            );
            println!("bytes_on_disk: {}", decision.bytes_on_disk);
            println!(
                "trigger: {}",
                decision
                    .trigger
                    .map(|t| t.as_str())
                    .unwrap_or("unknown")
            );
            println!("reason: {}", decision.reason);

            let notice = doc.notice();
            println!("notice_title: {}", notice.title);
            println!("notice_escalation_label: {}", notice.escalation_label);

            if let Some(needle) = args.find.as_deref() {
                let found = doc
                    .viewer
                    .find_first(needle)
                    .map_err(|e| e.to_string())?;
                match found {
                    Some(offset) => println!("find_first_offset: {offset}"),
                    None => println!("find_first_offset: <none>"),
                }
            }

            if args.open_anyway {
                let normal = doc.open_anyway(&root).map_err(|e| e.to_string())?;
                println!();
                println!("override_mode: normal");
                println!("override_buffer_len: {}", normal.buffer.len());
            }
        }
    }

    Ok(())
}

fn parse_args(raw: &[String]) -> Result<Args, String> {
    let mut out = Args {
        disposition: DocumentOpenDisposition::Auto,
        ..Args::default()
    };
    let mut iter = raw.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--threshold-bytes" => {
                let value = iter
                    .next()
                    .ok_or_else(|| "--threshold-bytes requires a value".to_owned())?;
                out.threshold_bytes = Some(
                    value
                        .parse::<u64>()
                        .map_err(|_| format!("invalid threshold: {value}"))?,
                );
            }
            "--force-large-file" => out.disposition = DocumentOpenDisposition::ForceLargeFile,
            "--force-normal" => out.disposition = DocumentOpenDisposition::ForceNormal,
            "--open-anyway" => out.open_anyway = true,
            "--find" => {
                let needle = iter
                    .next()
                    .ok_or_else(|| "--find requires a needle string".to_owned())?;
                out.find = Some(needle.clone());
            }
            "--help" | "-h" => return Err(usage()),
            other if other.starts_with("--") => return Err(format!("unknown flag: {other}\n\n{}", usage())),
            other => {
                if out.path.is_some() {
                    return Err(format!("unexpected extra arg: {other}\n\n{}", usage()));
                }
                out.path = Some(PathBuf::from(other));
            }
        }
    }
    Ok(out)
}

fn usage() -> String {
    "large_file_guard — large-file classification + constrained viewer smoke\n\n\
     Usage:\n\
     \tlarge_file_guard <file_path> [--threshold-bytes N] [--force-large-file|--force-normal] [--find NEEDLE] [--open-anyway]\n"
        .to_owned()
}
