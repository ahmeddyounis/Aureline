use std::fmt::Write as _;
use std::fs;
use std::io;
use std::path::Path;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildIdentityRecord {
    pub schema_version: u32,
    pub commit: String,
    pub commit_short: String,
    pub dirty: bool,
    pub toolchain_channel: String,
    pub rustc_version: String,
    pub cargo_version: String,
    pub host_triple: String,
    pub target_triple: String,
    pub profile: String,
    pub workspace_version: String,
    pub source_date_epoch: i64,
    pub build_timestamp_utc: String,
}

impl BuildIdentityRecord {
    pub fn to_json_pretty(&self) -> String {
        let mut out = String::new();
        out.push_str("{\n");
        push_json_kv_u32(&mut out, "schema_version", self.schema_version, true, 2);
        push_json_kv_str(&mut out, "commit", &self.commit, true, 2);
        push_json_kv_str(&mut out, "commit_short", &self.commit_short, true, 2);
        push_json_kv_bool(&mut out, "dirty", self.dirty, true, 2);
        push_json_kv_str(
            &mut out,
            "toolchain_channel",
            &self.toolchain_channel,
            true,
            2,
        );
        push_json_kv_str(&mut out, "rustc_version", &self.rustc_version, true, 2);
        push_json_kv_str(&mut out, "cargo_version", &self.cargo_version, true, 2);
        push_json_kv_str(&mut out, "host_triple", &self.host_triple, true, 2);
        push_json_kv_str(&mut out, "target_triple", &self.target_triple, true, 2);
        push_json_kv_str(&mut out, "profile", &self.profile, true, 2);
        push_json_kv_str(
            &mut out,
            "workspace_version",
            &self.workspace_version,
            true,
            2,
        );
        push_json_kv_i64(
            &mut out,
            "source_date_epoch",
            self.source_date_epoch,
            true,
            2,
        );
        push_json_kv_str(
            &mut out,
            "build_timestamp_utc",
            &self.build_timestamp_utc,
            false,
            2,
        );
        out.push_str("}\n");
        out
    }
}

pub fn build_identity() -> BuildIdentityRecord {
    BuildIdentityRecord {
        schema_version: 1,
        commit: env!("AURELINE_BUILD_IDENTITY_COMMIT").to_owned(),
        commit_short: env!("AURELINE_BUILD_IDENTITY_COMMIT_SHORT").to_owned(),
        dirty: parse_bool(env!("AURELINE_BUILD_IDENTITY_DIRTY")),
        toolchain_channel: env!("AURELINE_BUILD_IDENTITY_TOOLCHAIN_CHANNEL").to_owned(),
        rustc_version: env!("AURELINE_BUILD_IDENTITY_RUSTC_VERSION").to_owned(),
        cargo_version: env!("AURELINE_BUILD_IDENTITY_CARGO_VERSION").to_owned(),
        host_triple: env!("AURELINE_BUILD_IDENTITY_HOST_TRIPLE").to_owned(),
        target_triple: env!("AURELINE_BUILD_IDENTITY_TARGET_TRIPLE").to_owned(),
        profile: env!("AURELINE_BUILD_IDENTITY_PROFILE").to_owned(),
        workspace_version: env!("AURELINE_BUILD_IDENTITY_WORKSPACE_VERSION").to_owned(),
        source_date_epoch: parse_i64(env!("AURELINE_BUILD_IDENTITY_SOURCE_DATE_EPOCH")),
        build_timestamp_utc: env!("AURELINE_BUILD_IDENTITY_BUILD_TIMESTAMP_UTC").to_owned(),
    }
}

/// Exact-build identity ref for runtime/support/export join points.
///
/// The string format mirrors the examples under fixtures/build/exact_build_examples/.
pub fn exact_build_identity_ref() -> String {
    let record = build_identity();
    let channel_token = exact_build_channel_token(release_channel_class());
    format!(
        "build-id:aureline:{channel}:{version}:{target}:{profile}:{commit_short}",
        channel = channel_token,
        version = record.workspace_version,
        target = record.target_triple,
        profile = record.profile,
        commit_short = record.commit_short
    )
}

pub fn release_channel_class() -> &'static str {
    option_env!("AURELINE_RELEASE_CHANNEL_CLASS").unwrap_or("dev_local")
}

pub fn write_build_identity_json(path: &Path) -> io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, build_identity().to_json_pretty())
}

fn exact_build_channel_token(class: &str) -> &str {
    match class {
        "dev_local" => "dev",
        other => other,
    }
}

fn parse_bool(value: &str) -> bool {
    matches!(value.trim(), "true" | "1" | "yes" | "on")
}

fn parse_i64(value: &str) -> i64 {
    value.trim().parse::<i64>().unwrap_or(0)
}

fn json_escape_into(out: &mut String, value: &str) {
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if c.is_control() => {
                let _ = write!(out, "\\u{:04x}", c as u32);
            }
            c => out.push(c),
        }
    }
}

fn push_json_indent(out: &mut String, spaces: usize) {
    for _ in 0..spaces {
        out.push(' ');
    }
}

fn push_json_kv_str(out: &mut String, key: &str, value: &str, comma: bool, indent: usize) {
    push_json_indent(out, indent);
    out.push('"');
    json_escape_into(out, key);
    out.push_str("\": \"");
    json_escape_into(out, value);
    out.push('"');
    if comma {
        out.push(',');
    }
    out.push('\n');
}

fn push_json_kv_bool(out: &mut String, key: &str, value: bool, comma: bool, indent: usize) {
    push_json_indent(out, indent);
    out.push('"');
    json_escape_into(out, key);
    out.push_str("\": ");
    out.push_str(if value { "true" } else { "false" });
    if comma {
        out.push(',');
    }
    out.push('\n');
}

fn push_json_kv_u32(out: &mut String, key: &str, value: u32, comma: bool, indent: usize) {
    push_json_indent(out, indent);
    out.push('"');
    json_escape_into(out, key);
    let _ = write!(out, "\": {value}");
    if comma {
        out.push(',');
    }
    out.push('\n');
}

fn push_json_kv_i64(out: &mut String, key: &str, value: i64, comma: bool, indent: usize) {
    push_json_indent(out, indent);
    out.push('"');
    json_escape_into(out, key);
    let _ = write!(out, "\": {value}");
    if comma {
        out.push(',');
    }
    out.push('\n');
}
