//! CLI mirror for inspecting Git conflict handoff packets.

use std::path::PathBuf;

use aureline_git::{GitConflictHandoffRequest, GitConflictHandoffService};

fn main() {
    let mut args = std::env::args().skip(1);
    let mut root = None;
    let mut workspace_ref = None;
    let mut path = None;
    let mut filesystem_identity_ref = None;
    let mut rollback_checkpoint_ref = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = args.next().map(PathBuf::from),
            "--workspace-ref" => workspace_ref = args.next(),
            "--path" => path = args.next().map(PathBuf::from),
            "--filesystem-identity-ref" => filesystem_identity_ref = args.next(),
            "--rollback-checkpoint-ref" => rollback_checkpoint_ref = args.next(),
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => path = Some(PathBuf::from(other)),
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
    let Some(path) = path else {
        eprintln!("missing --path <repo-path>");
        print_usage();
        std::process::exit(2);
    };

    let mut request = if let Some(workspace_ref) = workspace_ref {
        GitConflictHandoffRequest::with_observed_at(
            workspace_ref,
            root,
            path,
            "cli:git:conflict-handoff",
        )
    } else {
        GitConflictHandoffRequest::for_path(root, path)
    };
    if let Some(filesystem_identity_ref) = filesystem_identity_ref {
        request = request.with_filesystem_identity_ref(filesystem_identity_ref);
    }
    if let Some(rollback_checkpoint_ref) = rollback_checkpoint_ref {
        request = request.with_rollback_checkpoint_ref(rollback_checkpoint_ref);
    }

    let service = GitConflictHandoffService::default();
    match serde_json::to_string_pretty(&service.preview_git_conflict(&request)) {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("Git conflict handoff serialization failed: {err}");
            std::process::exit(7);
        }
    }
}

fn print_usage() {
    eprintln!(
        "usage: aureline_git_conflict_handoff --path <repo-path> [--root <repo>] [--workspace-ref <ref>] [--filesystem-identity-ref <ref>] [--rollback-checkpoint-ref <ref>]"
    );
}
