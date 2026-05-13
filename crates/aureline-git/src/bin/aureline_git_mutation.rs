//! CLI mirror for inspecting Git mutation preview/apply packets.

use std::path::PathBuf;

use aureline_git::{GitMutationOperationKind, GitMutationRequest, GitMutationService};
use serde::Serialize;

#[derive(Serialize)]
struct RevertDrill<TPreview, TResult> {
    forward_preview: TPreview,
    forward_result: TResult,
    revert_preview: TPreview,
    revert_result: TResult,
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut root = None;
    let mut workspace_ref = None;
    let mut kind = None;
    let mut paths = Vec::new();
    let mut apply = false;
    let mut revert_after_apply = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = args.next().map(PathBuf::from),
            "--workspace-ref" => workspace_ref = args.next(),
            "--kind" => kind = args.next().and_then(|value| parse_kind(&value)),
            "--path" => {
                if let Some(path) = args.next() {
                    paths.push(PathBuf::from(path));
                }
            }
            "--apply" => apply = true,
            "--revert-after-apply" => {
                apply = true;
                revert_after_apply = true;
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => paths.push(PathBuf::from(other)),
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
    let Some(kind) = kind else {
        eprintln!("missing --kind stage|unstage|discard");
        print_usage();
        std::process::exit(2);
    };
    if paths.is_empty() {
        eprintln!("at least one --path is required");
        print_usage();
        std::process::exit(2);
    }

    let request = if let Some(workspace_ref) = workspace_ref {
        GitMutationRequest::with_observed_at(workspace_ref, root, kind, paths, "cli:git:mutation")
    } else {
        GitMutationRequest::for_paths(root, kind, paths)
    };
    let service = GitMutationService::default();
    let preview = service.preview(&request);
    let result = if revert_after_apply {
        let forward_result = service.apply(&preview, "cli:git:mutation:applied");
        let revert_preview =
            service.preview_revert(&forward_result, "cli:git:mutation:revert-preview");
        let revert_result = service.apply(&revert_preview, "cli:git:mutation:reverted");
        serde_json::to_string_pretty(&RevertDrill {
            forward_preview: preview,
            forward_result,
            revert_preview,
            revert_result,
        })
    } else if apply {
        serde_json::to_string_pretty(&service.apply(&preview, "cli:git:mutation:applied"))
    } else {
        serde_json::to_string_pretty(&preview)
    };

    match result {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("Git mutation serialization failed: {err}");
            std::process::exit(7);
        }
    }
}

fn parse_kind(value: &str) -> Option<GitMutationOperationKind> {
    match value {
        "stage" => Some(GitMutationOperationKind::Stage),
        "unstage" => Some(GitMutationOperationKind::Unstage),
        "discard" => Some(GitMutationOperationKind::Discard),
        _ => None,
    }
}

fn print_usage() {
    eprintln!(
        "usage: aureline_git_mutation --kind stage|unstage|discard --path <repo-path> [--root <repo>] [--apply] [--revert-after-apply]"
    );
}
