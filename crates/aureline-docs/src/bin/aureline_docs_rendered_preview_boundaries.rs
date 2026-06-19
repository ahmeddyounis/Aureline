//! Headless emitter for the rendered-preview capability-boundary packet and fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-docs --bin aureline_docs_rendered_preview_boundaries -- packet
//! cargo run -q -p aureline-docs --bin aureline_docs_rendered_preview_boundaries -- support-export
//! cargo run -q -p aureline-docs --bin aureline_docs_rendered_preview_boundaries -- summary
//! cargo run -q -p aureline-docs --bin aureline_docs_rendered_preview_boundaries -- fixture source_mode_local
//! cargo run -q -p aureline-docs --bin aureline_docs_rendered_preview_boundaries -- validate
//! ```

use aureline_docs::{
    not_applicable_capability_boundaries, seeded_capability_boundaries,
    seeded_stable_rendered_preview_boundary, AccessibilityParity, CapabilityRequestState,
    DocsArtifactKind, DocsExternalOpenState, DocsFreshnessClass, DocsMaintenanceAction,
    DocsMirrorOfflinePosture, DocsPreviewMode, DocsPreviewSanitizationState, PreviewCapabilityKind,
    PreviewRenderPosture, RenderedPreviewBoundary, VersionMatchState,
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
            print_json(&seeded_stable_rendered_preview_boundary())?
        }
        Some("summary") => print!(
            "{}",
            seeded_stable_rendered_preview_boundary().render_markdown_summary()
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
        "rendered_mirrored_offline" => rendered_mirrored_offline_fixture(),
        "rendered_raw_html_disclosed" => rendered_raw_html_disclosed_fixture(),
        "rendered_degraded_accessibility" => rendered_degraded_accessibility_fixture(),
        other => return Err(format!("unknown fixture: {other}").into()),
    };
    print_json(&fixture)
}

fn validate_packet() {
    let packet = seeded_stable_rendered_preview_boundary();
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

/// Source mode: nothing renders, so every capability boundary stays inert.
fn source_mode_local_fixture() -> RenderedPreviewBoundary {
    let mut packet = seeded_stable_rendered_preview_boundary();
    packet.boundary_id = "docs-preview-boundary:changelog:source:0001".to_owned();
    packet.boundary_label = "Changelog rendered-preview boundary".to_owned();
    packet.workspace_ref = "docs-workspace:changelog:source:0001".to_owned();
    packet.artifact_kind = DocsArtifactKind::Changelog;
    packet.artifact_ref = "CHANGELOG.md".to_owned();
    packet.active_mode = DocsPreviewMode::Source;
    packet.sanitization_state = DocsPreviewSanitizationState::NotApplicable;
    packet.sanitization_note = None;
    packet.capability_boundaries = not_applicable_capability_boundaries();
    packet
}

/// Rendered mode over a pinned offline pack: remote assets cannot be reached.
fn rendered_mirrored_offline_fixture() -> RenderedPreviewBoundary {
    let mut packet = seeded_stable_rendered_preview_boundary();
    packet.boundary_id = "docs-preview-boundary:help:rendered:offline:0001".to_owned();
    packet.boundary_label = "Help article rendered-preview boundary".to_owned();
    packet.workspace_ref = "docs-workspace:help:rendered:offline:0001".to_owned();
    packet.artifact_kind = DocsArtifactKind::HelpArticle;
    packet.artifact_ref = "docs/help/rendered-preview-boundaries.md".to_owned();
    packet.active_mode = DocsPreviewMode::Rendered;
    packet.sanitization_state = DocsPreviewSanitizationState::SanitizedSafe;
    packet.sanitization_note = None;
    packet.source_version_badge.freshness_class = DocsFreshnessClass::DegradedCached;
    packet.source_version_badge.version_match_state = VersionMatchState::CompatibleMinorDrift;
    packet.mirror_offline_state = DocsMirrorOfflinePosture::OfflinePinnedPack;

    let mut boundaries = seeded_capability_boundaries();
    for boundary in &mut boundaries {
        match boundary.capability_kind {
            PreviewCapabilityKind::Diagrams => {
                boundary.request_state = CapabilityRequestState::RequestedAwaitingConsent;
                boundary.render_posture = PreviewRenderPosture::Blocked;
                boundary.boundary_cue = "Diagrams blocked offline".to_owned();
                boundary.consent_ref = None;
                boundary.note = Some(
                    "The diagram engine is unavailable on a pinned offline pack; the fenced \
                     source stays readable."
                        .to_owned(),
                );
            }
            PreviewCapabilityKind::Math => {
                boundary.request_state = CapabilityRequestState::NotRequested;
                boundary.render_posture = PreviewRenderPosture::Disabled;
                boundary.boundary_cue = "Math disabled".to_owned();
                boundary.consent_ref = None;
            }
            PreviewCapabilityKind::RemoteAssets => {
                boundary.request_state = CapabilityRequestState::RequestedAwaitingConsent;
                boundary.render_posture = PreviewRenderPosture::Blocked;
                boundary.boundary_cue = "Remote assets unreachable offline".to_owned();
                boundary.external_open_state = DocsExternalOpenState::Unavailable;
                boundary.open_externally_action = None;
                boundary.note = Some(
                    "Remote assets cannot be fetched or opened while the source is a pinned \
                     offline pack."
                        .to_owned(),
                );
            }
            _ => {}
        }
    }
    packet.capability_boundaries = boundaries;
    packet
}

/// Rendered mode with disclosed raw HTML and a blocked custom component.
fn rendered_raw_html_disclosed_fixture() -> RenderedPreviewBoundary {
    let mut packet = seeded_stable_rendered_preview_boundary();
    packet.boundary_id = "docs-preview-boundary:module_doc:rendered:rawhtml:0001".to_owned();
    packet.boundary_label = "Module doc rendered-preview boundary".to_owned();
    packet.workspace_ref = "docs-workspace:module_doc:rendered:rawhtml:0001".to_owned();
    packet.artifact_kind = DocsArtifactKind::ModuleDoc;
    packet.artifact_ref = "crates/aureline-docs/src/authoring/safe_rendered_preview.rs".to_owned();
    packet.active_mode = DocsPreviewMode::Rendered;
    packet.sanitization_state = DocsPreviewSanitizationState::RawHtmlAllowedDisclosed;
    packet.sanitization_note = Some(
        "This document opts into raw embedded HTML. The HTML renders under an explicit \
         disclosure; scripts, iframes, and event handlers stay stripped."
            .to_owned(),
    );

    let mut boundaries = seeded_capability_boundaries();
    for boundary in &mut boundaries {
        if boundary.capability_kind == PreviewCapabilityKind::CustomComponents {
            boundary.request_state = CapabilityRequestState::DeniedByPolicy;
            boundary.render_posture = PreviewRenderPosture::Blocked;
            boundary.boundary_cue = "Custom components blocked".to_owned();
            boundary.note = Some(
                "Custom components are denied by policy for this artifact; they render inert as \
                 source text."
                    .to_owned(),
            );
        }
    }
    packet.capability_boundaries = boundaries;
    packet
}

/// Rendered mode that degrades zoom and motion parity honestly with a disclosure.
fn rendered_degraded_accessibility_fixture() -> RenderedPreviewBoundary {
    let mut packet = seeded_stable_rendered_preview_boundary();
    packet.boundary_id = "docs-preview-boundary:readme:rendered:a11y:0001".to_owned();
    packet.boundary_label = "README rendered-preview boundary (degraded parity)".to_owned();
    packet.workspace_ref = "docs-workspace:readme:rendered:0001".to_owned();
    packet.active_mode = DocsPreviewMode::Rendered;
    packet.accessibility_parity = AccessibilityParity {
        theme_parity: true,
        zoom_parity: false,
        density_parity: true,
        reduced_motion_parity: false,
        keyboard_parity: true,
        parity_note: Some(
            "Sandboxed diagrams render at a fixed zoom and may animate; the preview discloses the \
             gap and the raw source stays available at the active zoom and reduced-motion setting."
                .to_owned(),
        ),
    };

    let mut boundaries = seeded_capability_boundaries();
    for boundary in &mut boundaries {
        if boundary.capability_kind == PreviewCapabilityKind::Diagrams {
            boundary.boundary_cue = "Diagrams render sandboxed (fixed zoom)".to_owned();
        }
    }
    packet.capability_boundaries = boundaries;

    // The open-externally action keeps a keyboard-reachable escape for the remote asset.
    packet.open_source_action =
        DocsMaintenanceAction::new("docs.preview.open_source", "Open source");
    packet
}

fn print_json<T: Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
