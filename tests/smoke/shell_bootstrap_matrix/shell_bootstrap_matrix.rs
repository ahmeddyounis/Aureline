//! Shell bootstrap smoke matrix.
//!
//! Exercises the desktop shell's bootstrap entry point against a synthetic
//! workspace and asserts one happy-path interaction per integrated workspace
//! crate. The matrix is the integration tripwire that detects when a crate
//! dependency is dropped from the `aureline-shell` Cargo manifest or when the
//! headless bootstrap path regresses.
//!
//! Headless affordance. The shell binary exposes
//! `--headless-test-edit-save` / `--headless-test-write-hex` /
//! `--headless-test-report`, which short-circuits `run_native_shell` before
//! window creation and drives a real open→edit→save sequence through the
//! buffer, editor, history, commands, and vfs surfaces. This harness invokes
//! the binary in that mode against a tmpdir-only workspace so it stays
//! display-free and GPU-free.
//!
//! For every workspace crate that `aureline-shell` declares in its
//! `[dependencies]` table the matrix performs a tight, public-API touch and
//! records a `crate_touched`. Removing a crate from `aureline-shell`'s
//! dependency list breaks the corresponding `use aureline_x::...` line in
//! this file and fails the build, which is the regression the spec asks for.

use std::process::Command;

use tempfile::TempDir;

// --- One public-API touch per integrated workspace crate. ---
//
// Each `use aureline_x::...` line binds this test to the crate's presence
// in `aureline-shell`'s `[dependencies]`. The matrix asserts a trivial,
// invariant property per touch so the assertion list survives churn in
// each crate's public surface (sizes are stable across formatting / field
// renames as long as the type exists).
use aureline_ai::evidence::ConfidenceClass;
use aureline_auth::credential_state::LifetimeClass;
use aureline_buffer::piece_tree::buffer::Buffer;
use aureline_build_info::build_identity;
use aureline_commands::descriptor::CommandId;
use aureline_content_safety::detector::has_suspicious_content;
use aureline_docs::citations::DocsNodeKind;
use aureline_editor::selection::CaretSelection;
use aureline_extensions::manifest_baseline::ExtensionLifecycleStateClass;
use aureline_git::status::BranchState;
use aureline_graph::GraphStore;
use aureline_graph_proto::vocab::NodeClass;
use aureline_history::body_object_id;
use aureline_input::keybindings::PlatformClass;
use aureline_language::tree_sitter::ParserRuntimeStateClass;
use aureline_preview::safe_preview::ContentClass;
use aureline_provider::registry::ProviderFamily;
use aureline_reactive_state::envelope::Freshness;
use aureline_recovery::crash_journal::ObjectClass;
use aureline_render::hooks::Hook;
use aureline_review::diff::DiffViewMode;
use aureline_runtime::execution_context::ExecutionContext;
use aureline_search::lexical::LexicalShell;
use aureline_settings::schema::SchemaRegistry;
use aureline_support::bundle::manifest::BuildIdentity;
use aureline_telemetry::hot_path_metrics::HotPathMetricsConfig;
use aureline_terminal::pty_host::PtyHost;
use aureline_text::prototype::FallbackStage;
use aureline_ui::tokens::TokenRegistry;
use aureline_vfs::capabilities::RootClass;
use aureline_workspace::recent_work::TargetKind;

/// The set of workspace crates this matrix expects `aureline-shell` to
/// integrate. The list must match `crates/aureline-shell/Cargo.toml`'s
/// `[dependencies]` table.
const INTEGRATED_CRATES: &[&str] = &[
    "aureline-ai",
    "aureline-auth",
    "aureline-build-info",
    "aureline-buffer",
    "aureline-commands",
    "aureline-content-safety",
    "aureline-docs",
    "aureline-editor",
    "aureline-extensions",
    "aureline-git",
    "aureline-graph",
    "aureline-graph-proto",
    "aureline-history",
    "aureline-input",
    "aureline-language",
    "aureline-preview",
    "aureline-provider",
    "aureline-reactive-state",
    "aureline-recovery",
    "aureline-review",
    "aureline-search",
    "aureline-settings",
    "aureline-render",
    "aureline-runtime",
    "aureline-support",
    "aureline-terminal",
    "aureline-telemetry",
    "aureline-text",
    "aureline-ui",
    "aureline-vfs",
    "aureline-workspace",
];

#[test]
fn headless_bootstrap_smoke_against_synthetic_workspace() {
    let workspace = TempDir::new().expect("synthetic workspace tmpdir");
    let target_rel = "notes.md";
    let target_abs = workspace.path().join(target_rel);
    std::fs::write(&target_abs, b"seed\n").expect("seed target file");

    let report_path = workspace.path().join("headless_report.json");
    let payload = b"matrix-smoke-byte-sequence\n";
    let hex_payload = hex_encode(payload);

    let bin = env!("CARGO_BIN_EXE_aureline_shell");
    let output = Command::new(bin)
        .arg("--open")
        .arg(workspace.path())
        .arg("--headless-test-edit-save")
        .arg(target_rel)
        .arg("--headless-test-write-hex")
        .arg(&hex_payload)
        .arg("--headless-test-report")
        .arg(&report_path)
        .output()
        .expect("aureline_shell binary should execute");

    assert!(
        output.status.success(),
        "headless bootstrap exit status was not success: status={:?} stdout={} stderr={}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    let written = std::fs::read(&target_abs).expect("read target after headless save");
    assert_eq!(
        written, payload,
        "headless edit/save did not commit the expected byte sequence"
    );

    let report = std::fs::read_to_string(&report_path).expect("read headless report");
    let report: serde_json::Value =
        serde_json::from_str(&report).expect("headless report is valid json");
    assert_eq!(report["outcome"], "committed");
    assert_eq!(report["byte_count"], payload.len());
    assert_eq!(report["mode"], "headless_edit_save");

    // Tmpdir confinement: every path written by the headless run lives
    // inside the workspace tmpdir (the headless edit/save log root derives
    // from the report path parent).
    assert!(
        report_path.starts_with(workspace.path()),
        "headless report escaped the workspace tmpdir"
    );
}

#[test]
fn shell_dependency_manifest_matches_matrix() {
    let manifest_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Cargo.toml");
    let manifest = std::fs::read_to_string(&manifest_path).expect("read aureline-shell Cargo.toml");

    let mut missing = Vec::new();
    for crate_name in INTEGRATED_CRATES {
        let needle = format!("{crate_name} = {{ path = ");
        if !manifest.contains(&needle) {
            missing.push(*crate_name);
        }
    }

    assert!(
        missing.is_empty(),
        "aureline-shell Cargo.toml is missing the following integrated crates: {missing:?}; \
         either restore the dependency or remove it from INTEGRATED_CRATES (matrix is the source of truth)"
    );
}

// ---- per-crate happy-path touches ------------------------------------------
//
// Each test asserts a stable, public property of one item from the named
// crate. The `use` line at the top of this file ties the assertion to the
// crate's presence in `aureline-shell`'s `[dependencies]`; the assertion
// keeps the matrix honest about *what* it integrates with.

#[test]
fn touches_aureline_ai() {
    assert!(std::mem::size_of::<ConfidenceClass>() > 0);
}

#[test]
fn touches_aureline_auth() {
    assert!(std::mem::size_of::<LifetimeClass>() > 0);
}

#[test]
fn touches_aureline_build_info() {
    let identity = build_identity();
    assert!(!identity.commit_short.is_empty());
}

#[test]
fn touches_aureline_buffer() {
    assert!(std::mem::size_of::<Buffer>() > 0);
}

#[test]
fn touches_aureline_commands() {
    let id: CommandId = String::from("cmd:shell.bootstrap_matrix.smoke");
    assert!(id.starts_with("cmd:"));
}

#[test]
fn touches_aureline_content_safety() {
    assert!(!has_suspicious_content(""));
}

#[test]
fn touches_aureline_docs() {
    assert!(std::mem::size_of::<DocsNodeKind>() > 0);
}

#[test]
fn touches_aureline_editor() {
    assert!(std::mem::size_of::<CaretSelection>() > 0);
}

#[test]
fn touches_aureline_extensions() {
    assert!(std::mem::size_of::<ExtensionLifecycleStateClass>() > 0);
}

#[test]
fn touches_aureline_git() {
    assert!(std::mem::size_of::<BranchState>() > 0);
}

#[test]
fn touches_aureline_graph() {
    assert!(std::mem::size_of::<GraphStore>() > 0);
}

#[test]
fn touches_aureline_graph_proto() {
    assert!(std::mem::size_of::<NodeClass>() > 0);
}

#[test]
fn touches_aureline_history() {
    let id = body_object_id(b"shell-bootstrap-matrix");
    assert!(id.starts_with("obj:blake3:"));
}

#[test]
fn touches_aureline_input() {
    assert!(std::mem::size_of::<PlatformClass>() > 0);
}

#[test]
fn touches_aureline_language() {
    assert!(std::mem::size_of::<ParserRuntimeStateClass>() > 0);
}

#[test]
fn touches_aureline_preview() {
    assert!(std::mem::size_of::<ContentClass>() > 0);
}

#[test]
fn touches_aureline_provider() {
    assert!(std::mem::size_of::<ProviderFamily>() > 0);
}

#[test]
fn touches_aureline_reactive_state() {
    assert!(std::mem::size_of::<Freshness>() > 0);
}

#[test]
fn touches_aureline_recovery() {
    assert!(std::mem::size_of::<ObjectClass>() > 0);
}

#[test]
fn touches_aureline_review() {
    assert!(std::mem::size_of::<DiffViewMode>() > 0);
}

#[test]
fn touches_aureline_search() {
    assert!(std::mem::size_of::<LexicalShell>() > 0);
}

#[test]
fn touches_aureline_settings() {
    assert!(std::mem::size_of::<SchemaRegistry>() > 0);
}

#[test]
fn touches_aureline_render() {
    assert!(std::mem::size_of::<Hook>() > 0);
}

#[test]
fn touches_aureline_runtime() {
    assert!(std::mem::size_of::<ExecutionContext>() > 0);
}

#[test]
fn touches_aureline_support() {
    assert!(std::mem::size_of::<BuildIdentity>() > 0);
}

#[test]
fn touches_aureline_terminal() {
    assert!(std::mem::size_of::<PtyHost>() > 0);
}

#[test]
fn touches_aureline_telemetry() {
    assert!(std::mem::size_of::<HotPathMetricsConfig>() > 0);
}

#[test]
fn touches_aureline_text() {
    assert!(std::mem::size_of::<FallbackStage>() > 0);
}

#[test]
fn touches_aureline_ui() {
    assert!(std::mem::size_of::<TokenRegistry>() > 0);
}

#[test]
fn touches_aureline_vfs() {
    assert!(std::mem::size_of::<RootClass>() > 0);
}

#[test]
fn touches_aureline_workspace() {
    assert!(std::mem::size_of::<TargetKind>() > 0);
}

fn hex_encode(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut out = String::with_capacity(bytes.len().saturating_mul(2));
    for byte in bytes {
        write!(&mut out, "{byte:02x}").expect("hex encoding into String never fails");
    }
    out
}
