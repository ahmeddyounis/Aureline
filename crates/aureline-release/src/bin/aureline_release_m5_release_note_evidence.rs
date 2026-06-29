//! Headless emitter for the release-note evidence set.
//!
//! The bin is the only mint-from-truth path for the published evidence-set inventory checked in at
//! `artifacts/release/m5-release-note-evidence.json`, the release-grade parity proof under
//! `artifacts/release/m5-release-note-proof/` (and its Markdown report), the machine-readable per-note
//! CSV export at `artifacts/release/m5-release-note-evidence.csv`, and the per-state evidence-set
//! fixtures under `fixtures/release/whats-new-and-migration/`. It separates marketing prose from a
//! controlled change class, the affected scope, support sensitivity, and the direct evidence /
//! migration / rollback / setting links every behavior-changing or security-sensitive M5 release note
//! must carry, and asserts every what's-new card is dismissible, reopenable, and non-blocking.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- proof
//! cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- csv
//! cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- variant <canonical|dismissed|docs_only|security_migration>
//! cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- consumer <consumer-token>
//! cargo run -q -p aureline-release --bin aureline_release_m5_release_note_evidence -- validate
//! ```

use aureline_release::m5_release_note_evidence::{
    seeded_m5_release_note_evidence_set, seeded_m5_release_note_evidence_set_dismissed,
    seeded_m5_release_note_evidence_set_docs_only,
    seeded_m5_release_note_evidence_set_security_and_migration, ReleaseNoteEvidenceSet,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("registry") | Some("proof") | None => {
            let packet = seeded_m5_release_note_evidence_set();
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("markdown") => {
            let packet = seeded_m5_release_note_evidence_set();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("csv") => {
            let packet = seeded_m5_release_note_evidence_set();
            assert_packet_valid(&packet)?;
            print!("{}", packet.render_note_csv());
        }
        Some("variant") => {
            let packet = parse_variant(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_packet_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("consumer") => {
            let packet = seeded_m5_release_note_evidence_set();
            assert_packet_valid(&packet)?;
            let token = args.get(1).map(String::as_str).unwrap_or("");
            let consumer = packet
                .consumers
                .iter()
                .find(|c| c.consumer.as_str() == token)
                .ok_or_else(|| format!("unknown consumer token: {token}"))?;
            println!("{}", serde_json::to_string_pretty(consumer)?);
        }
        Some("validate") => {
            let packet = seeded_m5_release_note_evidence_set();
            assert_packet_valid(&packet)?;
            assert_packet_valid(&seeded_m5_release_note_evidence_set_dismissed())?;
            assert_packet_valid(&seeded_m5_release_note_evidence_set_docs_only())?;
            assert_packet_valid(&seeded_m5_release_note_evidence_set_security_and_migration())?;
            println!(
                "ok: release-note evidence valid ({} notes, {} consumers)",
                packet.notes.len(),
                packet.consumers.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_variant(token: &str) -> Result<ReleaseNoteEvidenceSet, Box<dyn std::error::Error>> {
    match token {
        "canonical" | "" => Ok(seeded_m5_release_note_evidence_set()),
        "dismissed" => Ok(seeded_m5_release_note_evidence_set_dismissed()),
        "docs_only" => Ok(seeded_m5_release_note_evidence_set_docs_only()),
        "security_migration" => Ok(seeded_m5_release_note_evidence_set_security_and_migration()),
        other => Err(format!(
            "unknown variant: {other} (canonical|dismissed|docs_only|security_migration)"
        )
        .into()),
    }
}

fn assert_packet_valid(packet: &ReleaseNoteEvidenceSet) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("packet failed validation: {}", tokens.join(",")).into())
}
