//! Headless inspector and regeneration path for the frozen M5 presentation
//! qualification matrix and its canonical example artifacts.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_presentation_qualification -- manifest
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_presentation_qualification -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_presentation_qualification -- write-all
//! ```
//!
//! `write-all` regenerates the checked-in support export, the Markdown
//! qualification matrix, and the three schema example artifacts so they stay
//! byte-aligned with the in-crate builder.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_shell::freeze_the_m5_presentation_session_walkthrough_waypoint_speaker_note_and_audience_follow_matrix::{
    seeded_presentation_qualification_matrix_packet, M5PresentationQualificationMatrixPacket,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let packet = seeded_presentation_qualification_matrix_packet();
    let violations = packet.validate();
    if !violations.is_empty() {
        return Err(format!(
            "seeded packet does not validate: {:?}",
            violations.iter().map(|v| v.as_str()).collect::<Vec<_>>()
        )
        .into());
    }

    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("manifest") | None => {
            println!("{}", packet.export_safe_json());
        }
        Some("summary") => {
            print!("{}", packet.render_markdown_summary());
        }
        Some("write-all") => {
            write_all(&repo_root(), &packet)?;
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn write_all(
    repo_root: &Path,
    packet: &M5PresentationQualificationMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let support_export = repo_root
        .join("artifacts/presentation/m5-presentation-qualification-matrix/support_export.json");
    if let Some(parent) = support_export.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&support_export, format!("{}\n", packet.export_safe_json()))?;

    let summary = repo_root.join("artifacts/presentation/m5-presentation-qualification-matrix.md");
    fs::write(&summary, packet.render_markdown_summary())?;

    // Canonical example artifacts for the three boundary schemas, lifted from the
    // clean presenter-walkthrough row so docs/help/support ingest the same shapes.
    let row = packet
        .row("presentation-qual:presenter-walkthrough:local:0001")
        .ok_or("seed is missing the presenter walkthrough row")?;
    let waypoint = row
        .session
        .waypoints
        .first()
        .ok_or("presenter walkthrough session has no waypoint")?;
    let note = waypoint
        .speaker_note
        .as_ref()
        .ok_or("presenter walkthrough waypoint has no speaker note")?;

    write_example(
        repo_root,
        "artifacts/presentation/presentation-session.example.json",
        &row.session,
    )?;
    write_example(
        repo_root,
        "artifacts/presentation/follow-waypoint.example.json",
        waypoint,
    )?;
    write_example(
        repo_root,
        "artifacts/presentation/speaker-note.example.json",
        note,
    )?;

    eprintln!("wrote presentation qualification matrix + 3 example artifacts");
    Ok(())
}

fn write_example<T: serde::Serialize>(
    repo_root: &Path,
    rel: &str,
    value: &T,
) -> Result<(), Box<dyn std::error::Error>> {
    let path = repo_root.join(rel);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, format!("{}\n", serde_json::to_string_pretty(value)?))?;
    Ok(())
}
