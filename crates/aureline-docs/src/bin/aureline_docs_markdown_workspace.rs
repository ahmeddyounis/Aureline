//! Headless emitter for the Markdown authoring workspace packet and fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_markdown_workspace -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_markdown_workspace -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_markdown_workspace -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_markdown_workspace -- fixture source_mode_local
//! cargo run -q -p aureline-docs --bin aureline_docs_markdown_workspace -- validate
//! ```

use aureline_docs::{
    seeded_stable_markdown_authoring_workspace, BrowserHandoffAvailability, DocsArtifactKind,
    DocsFreshnessClass, DocsMirrorOfflinePosture, DocsPreviewMode, DocsPreviewSanitizationState,
    MarkdownAuthoringWorkspace, RenderCapability, VersionMatchState, WorkspaceRenderCapabilities,
};
use serde::Serialize;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    match args.first().map(String::as_str) {
        Some("packet") | Some("support-export") | None => {
            print_json(&seeded_stable_markdown_authoring_workspace())?
        }
        Some("summary") => print!(
            "{}",
            seeded_stable_markdown_authoring_workspace().render_markdown_summary()
        ),
        Some("fixture") => emit_fixture(args.get(1).map(String::as_str))?,
        Some("validate") => validate_packet(),
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn emit_fixture(name: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let name = name.ok_or("fixture name is required")?;
    let fixture = match name {
        "source_mode_local" => source_mode_local_fixture(),
        "rendered_mode_mirrored_offline" => rendered_mode_mirrored_offline_fixture(),
        "rendered_raw_html_disclosed" => rendered_raw_html_disclosed_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet = seeded_stable_markdown_authoring_workspace();
    let violations = packet.validate();
    if violations.is_empty() {
        println!("ok");
    } else {
        for violation in &violations {
            eprintln!("{}", violation.as_str());
        }
        std::process::exit(3);
    }
}

/// Source mode: nothing renders, so sanitization and capabilities stay inert.
fn source_mode_local_fixture() -> MarkdownAuthoringWorkspace {
    let mut packet = seeded_stable_markdown_authoring_workspace();
    packet.workspace_id = "docs-workspace:changelog:source:0001".to_owned();
    packet.workspace_label = "Changelog authoring workspace".to_owned();
    packet.artifact_kind = DocsArtifactKind::Changelog;
    packet.artifact_ref = "CHANGELOG.md".to_owned();
    packet.active_mode = DocsPreviewMode::Source;
    packet.remembered_mode_preference = DocsPreviewMode::Source;
    packet.sanitization_state = DocsPreviewSanitizationState::NotApplicable;
    packet.sanitization_note = None;
    packet.render_capabilities = WorkspaceRenderCapabilities::all_not_applicable();
    packet.anchor_context = None;
    packet
}

/// Rendered mode over a pinned offline pack: browser handoff is unavailable.
fn rendered_mode_mirrored_offline_fixture() -> MarkdownAuthoringWorkspace {
    let mut packet = seeded_stable_markdown_authoring_workspace();
    packet.workspace_id = "docs-workspace:help:rendered:offline:0001".to_owned();
    packet.workspace_label = "Help article authoring workspace".to_owned();
    packet.artifact_kind = DocsArtifactKind::HelpArticle;
    packet.artifact_ref = "docs/help/markdown-authoring-workspace.md".to_owned();
    packet.active_mode = DocsPreviewMode::Rendered;
    packet.remembered_mode_preference = DocsPreviewMode::Rendered;
    packet.sanitization_state = DocsPreviewSanitizationState::SanitizedSafe;
    packet.sanitization_note = None;
    packet.render_capabilities = WorkspaceRenderCapabilities {
        diagrams: RenderCapability::Blocked,
        math: RenderCapability::Disabled,
        custom_components: RenderCapability::Disabled,
    };
    packet.source_version_badge.freshness_class = DocsFreshnessClass::DegradedCached;
    packet.source_version_badge.version_match_state = VersionMatchState::CompatibleMinorDrift;
    packet.mirror_offline_state = DocsMirrorOfflinePosture::OfflinePinnedPack;
    packet.browser_handoff_availability = BrowserHandoffAvailability::UnavailableOffline;
    packet.open_browser_action = None;
    packet.anchor_context = None;
    packet
}

/// Rendered mode with disclosed raw HTML and a blocked custom component.
fn rendered_raw_html_disclosed_fixture() -> MarkdownAuthoringWorkspace {
    let mut packet = seeded_stable_markdown_authoring_workspace();
    packet.workspace_id = "docs-workspace:module_doc:rendered:rawhtml:0001".to_owned();
    packet.workspace_label = "Module doc authoring workspace".to_owned();
    packet.artifact_kind = DocsArtifactKind::ModuleDoc;
    packet.artifact_ref = "crates/aureline-docs/src/authoring/markdown_workspace.rs".to_owned();
    packet.active_mode = DocsPreviewMode::Rendered;
    packet.remembered_mode_preference = DocsPreviewMode::Rendered;
    packet.sanitization_state = DocsPreviewSanitizationState::RawHtmlAllowedDisclosed;
    packet.sanitization_note = Some(
        "This document opts into raw embedded HTML. The HTML renders under an explicit \
         disclosure; scripts, iframes, and event handlers stay stripped."
            .to_owned(),
    );
    packet.render_capabilities = WorkspaceRenderCapabilities {
        diagrams: RenderCapability::SandboxedOptIn,
        math: RenderCapability::Disabled,
        custom_components: RenderCapability::Blocked,
    };
    packet
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
