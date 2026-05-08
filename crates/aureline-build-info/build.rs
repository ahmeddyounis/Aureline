use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR missing"));
    let repo_root = manifest_dir
        .parent()
        .and_then(|p| p.parent())
        .expect("expected crate under <repo>/crates/<name>");

    println!(
        "cargo:rerun-if-changed={}",
        repo_root.join("rust-toolchain.toml").display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        repo_root.join("Cargo.toml").display()
    );
    println!("cargo:rerun-if-env-changed=SOURCE_DATE_EPOCH");
    emit_git_rerun_markers(repo_root);

    let commit = git(repo_root, &["rev-parse", "HEAD"]).unwrap_or_else(|| "unknown".to_owned());
    let commit_short = git(repo_root, &["rev-parse", "--short=12", "HEAD"])
        .unwrap_or_else(|| "unknown".to_owned());
    let dirty = match Command::new("git")
        .args(["diff-index", "--quiet", "HEAD", "--"])
        .current_dir(repo_root)
        .status()
    {
        Ok(status) => !status.success(),
        Err(_) => true,
    };

    let toolchain_channel = read_quoted_key(repo_root.join("rust-toolchain.toml"), "channel")
        .unwrap_or_else(|| "unknown".to_owned());

    let rustc_cmd = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let cargo_cmd = env::var("CARGO").unwrap_or_else(|_| "cargo".to_owned());

    let rustc_version = run_stdout_trimmed(repo_root, &rustc_cmd, &["--version"])
        .unwrap_or_else(|| "unknown".to_owned());
    let cargo_version = run_stdout_trimmed(repo_root, &cargo_cmd, &["--version"])
        .unwrap_or_else(|| "unknown".to_owned());

    let host_triple = env::var("HOST").unwrap_or_else(|_| "unknown".to_owned());
    let target_triple = env::var("TARGET").unwrap_or_else(|_| host_triple.clone());

    let profile = match env::var("PROFILE").as_deref() {
        Ok("release") => "release".to_owned(),
        _ => "dev".to_owned(),
    };

    let workspace_version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "unknown".to_owned());

    let source_date_epoch = env::var("SOURCE_DATE_EPOCH")
        .ok()
        .and_then(|raw| raw.parse::<i64>().ok())
        .or_else(|| {
            git(repo_root, &["log", "-1", "--pretty=%ct"]).and_then(|s| s.parse::<i64>().ok())
        })
        .unwrap_or(0);
    let build_timestamp_utc = epoch_seconds_to_utc_rfc3339(source_date_epoch);

    emit_env("AURELINE_BUILD_IDENTITY_COMMIT", &commit);
    emit_env("AURELINE_BUILD_IDENTITY_COMMIT_SHORT", &commit_short);
    emit_env(
        "AURELINE_BUILD_IDENTITY_DIRTY",
        if dirty { "true" } else { "false" },
    );
    emit_env(
        "AURELINE_BUILD_IDENTITY_TOOLCHAIN_CHANNEL",
        &toolchain_channel,
    );
    emit_env("AURELINE_BUILD_IDENTITY_RUSTC_VERSION", &rustc_version);
    emit_env("AURELINE_BUILD_IDENTITY_CARGO_VERSION", &cargo_version);
    emit_env("AURELINE_BUILD_IDENTITY_HOST_TRIPLE", &host_triple);
    emit_env("AURELINE_BUILD_IDENTITY_TARGET_TRIPLE", &target_triple);
    emit_env("AURELINE_BUILD_IDENTITY_PROFILE", &profile);
    emit_env(
        "AURELINE_BUILD_IDENTITY_WORKSPACE_VERSION",
        &workspace_version,
    );
    emit_env(
        "AURELINE_BUILD_IDENTITY_SOURCE_DATE_EPOCH",
        &source_date_epoch.to_string(),
    );
    emit_env(
        "AURELINE_BUILD_IDENTITY_BUILD_TIMESTAMP_UTC",
        &build_timestamp_utc,
    );
}

fn emit_env(key: &str, value: &str) {
    println!("cargo:rustc-env={key}={value}");
}

fn run_stdout_trimmed(repo_root: &Path, cmd: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(cmd)
        .args(args)
        .current_dir(repo_root)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn git(repo_root: &Path, args: &[&str]) -> Option<String> {
    run_stdout_trimmed(repo_root, "git", args)
}

fn read_quoted_key(path: PathBuf, key: &str) -> Option<String> {
    let contents = fs::read_to_string(path).ok()?;
    for raw in contents.lines() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some(rest) = line.strip_prefix(key) else {
            continue;
        };
        let rest = rest.trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let rest = rest.trim();
        let Some(start) = rest.find('"') else {
            continue;
        };
        let rest = &rest[start + 1..];
        let Some(end) = rest.find('"') else {
            continue;
        };
        return Some(rest[..end].to_owned());
    }
    None
}

fn emit_git_rerun_markers(repo_root: &Path) {
    let git_dir = repo_root.join(".git");
    if !git_dir.exists() {
        return;
    }

    let head_path = git_dir.join("HEAD");
    println!("cargo:rerun-if-changed={}", head_path.display());

    if let Ok(head_contents) = fs::read_to_string(&head_path) {
        if let Some(ref_path) = head_contents.trim().strip_prefix("ref:") {
            let ref_path = ref_path.trim();
            if !ref_path.is_empty() {
                println!(
                    "cargo:rerun-if-changed={}",
                    git_dir.join(ref_path).display()
                );
            }
        }
    }

    let index_path = git_dir.join("index");
    if index_path.exists() {
        println!("cargo:rerun-if-changed={}", index_path.display());
    }
}

fn epoch_seconds_to_utc_rfc3339(seconds: i64) -> String {
    let secs_per_day = 86_400i64;
    let days = seconds.div_euclid(secs_per_day);
    let secs_of_day = seconds.rem_euclid(secs_per_day);
    let (year, month, day) = civil_from_days(days);
    let hour = (secs_of_day / 3600) as u32;
    let minute = ((secs_of_day % 3600) / 60) as u32;
    let second = (secs_of_day % 60) as u32;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

/// Convert a day count (days since 1970-01-01) into a UTC civil date.
///
/// Algorithm: Howard Hinnant's civil_from_days, adapted for i64.
fn civil_from_days(days_since_unix_epoch: i64) -> (i32, u32, u32) {
    let z = days_since_unix_epoch + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 }.div_euclid(146_097);
    let doe = z - era * 146_097; // [0, 146096]
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096).div_euclid(365); // [0, 399]
    let mut y = (yoe + era * 400) as i32;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100); // [0, 365]
    let mp = (5 * doy + 2).div_euclid(153); // [0, 11]
    let d = (doy - (153 * mp + 2).div_euclid(5) + 1) as u32; // [1, 31]
    let m = (mp + if mp < 10 { 3 } else { -9 }) as i32; // [1, 12]
    y += if m <= 2 { 1 } else { 0 };
    (y, m as u32, d)
}
