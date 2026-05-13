//! CLI mirror for inspecting Git commit preview/apply packets.

use std::path::PathBuf;

use aureline_git::{GitCommitAuthorInput, GitCommitMode, GitCommitRequest, GitCommitService};

fn main() {
    let mut args = std::env::args().skip(1);
    let mut root = None;
    let mut workspace_ref = None;
    let mut mode = GitCommitMode::Normal;
    let mut message = None;
    let mut author_name = None;
    let mut author_email = None;
    let mut apply = false;
    let mut ack_history_guardrail = false;
    let mut squash_target = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = args.next().map(PathBuf::from),
            "--workspace-ref" => workspace_ref = args.next(),
            "--mode" => {
                mode = match args.next().as_deref().and_then(parse_mode) {
                    Some(mode) => mode,
                    None => {
                        eprintln!("--mode requires normal|amend|squash");
                        std::process::exit(2);
                    }
                };
            }
            "--message" | "-m" => message = args.next(),
            "--author-name" => author_name = args.next(),
            "--author-email" => author_email = args.next(),
            "--ack-history-guardrail" => ack_history_guardrail = true,
            "--squash-target" => squash_target = args.next(),
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
    let Some(message) = message else {
        eprintln!("missing --message");
        print_usage();
        std::process::exit(2);
    };

    let mut request = if let Some(workspace_ref) = workspace_ref {
        GitCommitRequest::with_observed_at(workspace_ref, root, mode, message, "cli:git:commit")
    } else {
        let mut request = GitCommitRequest::for_message(root, message);
        request.mode = mode;
        request.requested_at = "cli:git:commit".to_string();
        request
    };
    if author_name.is_some() || author_email.is_some() {
        request = request.with_author(GitCommitAuthorInput {
            display_name: author_name,
            email: author_email,
        });
    }
    if ack_history_guardrail {
        request = request.acknowledge_history_guardrail();
    }
    if let Some(target) = squash_target {
        request = request.with_squash_target(target);
    }

    let service = GitCommitService::default();
    let preview = service.preview(&request);
    let result = if apply {
        serde_json::to_string_pretty(&service.apply(&preview, "cli:git:commit:applied"))
    } else {
        serde_json::to_string_pretty(&preview)
    };

    match result {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("Git commit serialization failed: {err}");
            std::process::exit(7);
        }
    }
}

fn parse_mode(value: &str) -> Option<GitCommitMode> {
    match value {
        "normal" => Some(GitCommitMode::Normal),
        "amend" => Some(GitCommitMode::Amend),
        "squash" => Some(GitCommitMode::Squash),
        _ => None,
    }
}

fn print_usage() {
    eprintln!(
        "usage: aureline_git_commit --message <text> [--root <repo>] [--mode normal|amend|squash] [--author-name <name> --author-email <email>] [--ack-history-guardrail] [--squash-target <rev>] [--apply]"
    );
}
