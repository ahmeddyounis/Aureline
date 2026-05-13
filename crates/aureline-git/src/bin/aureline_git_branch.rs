//! CLI mirror for inspecting Git branch preview/apply packets.

use std::path::PathBuf;

use aureline_git::{GitBranchOperationKind, GitBranchRequest, GitBranchService};

fn main() {
    let mut args = std::env::args().skip(1);
    let mut root = None;
    let mut workspace_ref = None;
    let mut operation = None;
    let mut target = None;
    let mut start_point = None;
    let mut track_remote = false;
    let mut apply = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = args.next().map(PathBuf::from),
            "--workspace-ref" => workspace_ref = args.next(),
            "--operation" | "--kind" => {
                operation = args.next().and_then(|value| parse_operation(&value));
                if operation.is_none() {
                    eprintln!("--operation requires switch|create|checkout");
                    std::process::exit(2);
                }
            }
            "--target" => target = args.next(),
            "--start-point" => start_point = args.next(),
            "--track-remote" => track_remote = true,
            "--apply" => apply = true,
            "--help" | "-h" => {
                print_usage();
                return;
            }
            other => {
                eprintln!("unexpected argument: {other}");
                print_usage();
                std::process::exit(2);
            }
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
    let Some(operation) = operation else {
        eprintln!("missing --operation switch|create|checkout");
        print_usage();
        std::process::exit(2);
    };
    let Some(target) = target else {
        eprintln!("missing --target");
        print_usage();
        std::process::exit(2);
    };

    let mut request = if let Some(workspace_ref) = workspace_ref {
        GitBranchRequest::with_observed_at(workspace_ref, root, operation, target, "cli:git:branch")
    } else {
        let mut request = GitBranchRequest::for_target(root, operation, target);
        request.requested_at = "cli:git:branch".to_string();
        request
    };
    if let Some(start_point) = start_point {
        request = request.with_start_point(start_point);
    }
    if track_remote {
        request = request.with_track_remote(true);
    }

    let service = GitBranchService::default();
    let preview = service.preview(&request);
    let result = if apply {
        serde_json::to_string_pretty(&service.apply(&preview, "cli:git:branch:applied"))
    } else {
        serde_json::to_string_pretty(&preview)
    };

    match result {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("Git branch serialization failed: {err}");
            std::process::exit(7);
        }
    }
}

fn parse_operation(value: &str) -> Option<GitBranchOperationKind> {
    match value {
        "switch" => Some(GitBranchOperationKind::Switch),
        "create" => Some(GitBranchOperationKind::Create),
        "checkout" => Some(GitBranchOperationKind::Checkout),
        _ => None,
    }
}

fn print_usage() {
    eprintln!(
        "usage: aureline_git_branch --operation switch|create|checkout --target <branch-or-rev> [--root <repo>] [--start-point <rev>] [--track-remote] [--apply]"
    );
}
