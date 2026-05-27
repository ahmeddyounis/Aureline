//! CLI mirror for the daily Git loop snapshot/preview/apply packets.

use std::path::PathBuf;

use aureline_git::{
    DailyLoopOperationKind, DailyLoopRequest, DailyLoopService, DailyLoopSnapshot,
    DailyLoopPreview, DailyLoopResult,
};
use serde::Serialize;

#[derive(Serialize)]
struct DailyLoopPacket {
    snapshot: Option<DailyLoopSnapshot>,
    preview: Option<DailyLoopPreview>,
    result: Option<DailyLoopResult>,
}

fn main() {
    let mut args = std::env::args().skip(1);
    let mut root = None;
    let mut kind = None;
    let mut paths = Vec::new();
    let mut preview_only = false;
    let mut apply = false;
    let mut message = None;
    let mut stash_entry_ref = None;
    let mut commit_ref = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = args.next().map(PathBuf::from),
            "--kind" => kind = args.next().and_then(|v| parse_kind(&v)),
            "--path" => {
                if let Some(path) = args.next() {
                    paths.push(PathBuf::from(path));
                }
            }
            "--preview" => preview_only = true,
            "--apply" => apply = true,
            "--message" | "-m" => message = args.next(),
            "--stash-entry" => stash_entry_ref = args.next(),
            "--commit-ref" => commit_ref = args.next(),
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
        eprintln!("missing --kind");
        print_usage();
        std::process::exit(2);
    };

    let mut request = DailyLoopRequest::for_worktree(root, kind, paths);
    if let Some(msg) = message {
        request = request.with_message(msg);
    }
    if let Some(stash_ref) = stash_entry_ref {
        request = request.with_stash_entry_ref(stash_ref);
    }
    if let Some(cref) = commit_ref {
        request = request.with_commit_ref(cref);
    }
    request.preview_only = preview_only;

    let service = DailyLoopService::default();

    let packet = if apply {
        if preview_only {
            eprintln!("--apply and --preview are mutually exclusive");
            std::process::exit(2);
        }
        let preview = service.preview(&request);
        let result = service.apply(&preview, "cli:git:daily_loop");
        DailyLoopPacket {
            snapshot: None,
            preview: Some(preview),
            result: Some(result),
        }
    } else if preview_only {
        let preview = service.preview(&request);
        DailyLoopPacket {
            snapshot: None,
            preview: Some(preview),
            result: None,
        }
    } else if kind.is_mutation() {
        let preview = service.preview(&request);
        DailyLoopPacket {
            snapshot: None,
            preview: Some(preview),
            result: None,
        }
    } else {
        let snapshot = service.snapshot(&request);
        DailyLoopPacket {
            snapshot: Some(snapshot),
            preview: None,
            result: None,
        }
    };

    match serde_json::to_string_pretty(&packet) {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("daily loop serialization failed: {err}");
            std::process::exit(7);
        }
    }
}

fn parse_kind(value: &str) -> Option<DailyLoopOperationKind> {
    match value {
        "status" => Some(DailyLoopOperationKind::Status),
        "diff" => Some(DailyLoopOperationKind::Diff),
        "stage" => Some(DailyLoopOperationKind::Stage),
        "unstage" => Some(DailyLoopOperationKind::Unstage),
        "commit" => Some(DailyLoopOperationKind::Commit),
        "amend" => Some(DailyLoopOperationKind::Amend),
        "stash_capture" => Some(DailyLoopOperationKind::StashCapture),
        "stash_apply" => Some(DailyLoopOperationKind::StashApply),
        "stash_pop" => Some(DailyLoopOperationKind::StashPop),
        "stash_drop" => Some(DailyLoopOperationKind::StashDrop),
        "stash_branch_from" => Some(DailyLoopOperationKind::StashBranchFrom),
        "blame" => Some(DailyLoopOperationKind::Blame),
        "history" => Some(DailyLoopOperationKind::History),
        _ => None,
    }
}

fn print_usage() {
    eprintln!(
        "usage: aureline_git_daily_loop --kind <operation> [--root <repo>] [--path <path> ...] [--preview] [--apply] [--message <msg>] [--stash-entry <ref>] [--commit-ref <ref>]"
    );
}
