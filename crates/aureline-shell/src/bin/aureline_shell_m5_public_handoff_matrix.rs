//! Headless emitter for the frozen M5 post-install notice/provenance,
//! community-handoff, reproduction-packet, and device-permission/auth-boundary
//! matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/help/m5-public-handoff/`, the governance Markdown summary
//! `artifacts/help/m5-public-handoff-governance.md`, the matrix CSV
//! `artifacts/help/m5-public-handoff-matrix.csv`, and the narrowed fixtures under
//! `fixtures/help/m5-public-handoff/`. Help/About, marketplace, update/service-
//! health, community-handoff, repro-packet, and capture/auth automation read this
//! matrix so claimed M5 rows cannot harden a handoff or boundary promise that
//! lacks a governed source.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_matrix -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_matrix -- governance
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_matrix -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_matrix -- fixture-repro-redaction-held
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_matrix -- fixture-provenance-unverified-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_public_handoff_matrix -- validate
//! ```

use aureline_shell::freeze_the_m5_public_handoff_and_capture_boundary_matrix::{
    seeded_m5_public_handoff_matrix,
    seeded_m5_public_handoff_matrix_provenance_unverified_narrowed,
    seeded_m5_public_handoff_matrix_repro_redaction_held, M5PublicHandoffMatrixPacket,
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
        Some("support-export") | None => {
            let packet = seeded_m5_public_handoff_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("governance") => {
            print!(
                "{}",
                seeded_m5_public_handoff_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_public_handoff_matrix().render_matrix_csv());
        }
        Some("fixture-repro-redaction-held") => {
            let packet = seeded_m5_public_handoff_matrix_repro_redaction_held();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-provenance-unverified-narrowed") => {
            let packet = seeded_m5_public_handoff_matrix_provenance_unverified_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_public_handoff_matrix(),
                seeded_m5_public_handoff_matrix_repro_redaction_held(),
                seeded_m5_public_handoff_matrix_provenance_unverified_narrowed(),
            ] {
                assert_valid(&packet)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(packet: &M5PublicHandoffMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
