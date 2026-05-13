//! CLI mirror for inspecting Git publish preview/apply packets.

use std::path::PathBuf;

use aureline_git::{
    GitPublishMode, GitPublishOriginScope, GitPublishRequest, GitPublishRouteClass,
    GitPublishService,
};

fn main() {
    let mut args = std::env::args().skip(1);
    let mut root = None;
    let mut workspace_ref = None;
    let mut mode = GitPublishMode::Push;
    let mut remote_name = None;
    let mut local_branch = None;
    let mut target_branch = None;
    let mut expected_remote_oid = None;
    let mut force_ack = false;
    let mut origin_scope = GitPublishOriginScope::LocalDesktop;
    let mut route_class = GitPublishRouteClass::DirectGitRemote;
    let mut apply = false;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--root" => root = args.next().map(PathBuf::from),
            "--workspace-ref" => workspace_ref = args.next(),
            "--mode" => {
                mode = match args.next().as_deref().and_then(parse_mode) {
                    Some(mode) => mode,
                    None => {
                        eprintln!("--mode requires push|force-with-lease");
                        std::process::exit(2);
                    }
                };
            }
            "--remote" => remote_name = args.next(),
            "--local-branch" => local_branch = args.next(),
            "--target-branch" => target_branch = args.next(),
            "--expected-remote-oid" => expected_remote_oid = args.next(),
            "--ack-force-review" => force_ack = true,
            "--origin-scope" => {
                origin_scope = match args.next().as_deref().and_then(parse_origin_scope) {
                    Some(origin_scope) => origin_scope,
                    None => {
                        eprintln!(
                            "--origin-scope requires local_desktop|remote_target|managed_workspace|headless_runner"
                        );
                        std::process::exit(2);
                    }
                };
            }
            "--route-class" => {
                route_class = match args.next().as_deref().and_then(parse_route_class) {
                    Some(route_class) => route_class,
                    None => {
                        eprintln!(
                            "--route-class requires direct_git_remote|mirror_or_proxy|provider_gateway|browser_handoff"
                        );
                        std::process::exit(2);
                    }
                };
            }
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

    let mut request = if let Some(workspace_ref) = workspace_ref {
        GitPublishRequest::with_observed_at(workspace_ref, root, "cli:git:publish")
    } else {
        let mut request = GitPublishRequest::for_root(root);
        request.requested_at = "cli:git:publish".to_string();
        request
    }
    .with_mode(mode)
    .with_route(origin_scope, route_class);

    if let Some(remote_name) = remote_name {
        request = request.with_remote_name(remote_name);
    }
    if let Some(local_branch) = local_branch {
        request = request.with_local_branch(local_branch);
    }
    if let Some(target_branch) = target_branch {
        request = request.with_target_branch(target_branch);
    }
    if force_ack {
        request = request.acknowledge_force_review();
    }
    if let Some(expected_remote_oid) = expected_remote_oid {
        request = request.with_expected_remote_oid(expected_remote_oid);
    }

    let service = GitPublishService::default();
    let preview = service.preview(&request);
    let result = if apply {
        serde_json::to_string_pretty(&service.apply(&preview, "cli:git:publish:applied"))
    } else {
        serde_json::to_string_pretty(&preview)
    };

    match result {
        Ok(json) => println!("{json}"),
        Err(err) => {
            eprintln!("Git publish serialization failed: {err}");
            std::process::exit(7);
        }
    }
}

fn parse_mode(value: &str) -> Option<GitPublishMode> {
    match value {
        "push" => Some(GitPublishMode::Push),
        "force-with-lease" | "force_with_lease" => Some(GitPublishMode::ForceWithLease),
        _ => None,
    }
}

fn parse_origin_scope(value: &str) -> Option<GitPublishOriginScope> {
    match value {
        "local_desktop" => Some(GitPublishOriginScope::LocalDesktop),
        "remote_target" => Some(GitPublishOriginScope::RemoteTarget),
        "managed_workspace" => Some(GitPublishOriginScope::ManagedWorkspace),
        "headless_runner" => Some(GitPublishOriginScope::HeadlessRunner),
        _ => None,
    }
}

fn parse_route_class(value: &str) -> Option<GitPublishRouteClass> {
    match value {
        "direct_git_remote" => Some(GitPublishRouteClass::DirectGitRemote),
        "mirror_or_proxy" => Some(GitPublishRouteClass::MirrorOrProxy),
        "provider_gateway" => Some(GitPublishRouteClass::ProviderGateway),
        "browser_handoff" => Some(GitPublishRouteClass::BrowserHandoff),
        _ => None,
    }
}

fn print_usage() {
    eprintln!(
        "usage: aureline_git_publish [--root <repo>] [--remote <name>] [--local-branch <branch>] [--target-branch <branch>] [--mode push|force-with-lease] [--ack-force-review --expected-remote-oid <oid>] [--origin-scope local_desktop|remote_target|managed_workspace|headless_runner] [--route-class direct_git_remote|mirror_or_proxy|provider_gateway|browser_handoff] [--apply]"
    );
}
