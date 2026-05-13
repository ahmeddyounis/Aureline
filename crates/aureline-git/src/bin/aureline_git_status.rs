//! CLI mirror for inspecting the canonical Git service snapshot.

use std::path::PathBuf;

use aureline_git::status::{ConsumerProjectionBundle, GitStatusRequest, GitStatusService};

fn main() {
    let mut emit_bundle = false;
    let mut root = None;

    for arg in std::env::args().skip(1) {
        if arg == "--bundle" {
            emit_bundle = true;
        } else {
            root = Some(PathBuf::from(arg));
        }
    }

    let root = match root {
        Some(root) => root,
        None => match std::env::current_dir() {
            Ok(dir) => dir,
            Err(err) => {
                eprintln!("current directory unavailable: {err}");
                std::process::exit(5);
            }
        },
    };

    let request = GitStatusRequest::for_root(root);
    let service = GitStatusService::default();
    let snapshot = service.snapshot(&request);
    let result = if emit_bundle {
        serde_json::to_string_pretty(&ConsumerProjectionBundle::from_snapshot(
            snapshot.observed_at.clone(),
            &snapshot,
        ))
    } else {
        serde_json::to_string_pretty(&snapshot)
    };

    match result {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("Git status serialization failed: {err}");
            std::process::exit(7);
        }
    }
}
