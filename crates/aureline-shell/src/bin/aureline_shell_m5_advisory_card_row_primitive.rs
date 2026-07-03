//! Headless emitter for the M5 security-advisory card / row primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-advisory-card-row-proof/`, its matrix CSV, the Markdown
//! report `artifacts/security/m5-advisory-card-row-primitive.md`, and the narrowed
//! fixtures under `fixtures/security/m5-advisory-card-row-primitive/`. Every M5
//! channel that has to warn about a published vulnerability, revocation, or
//! security-impacting fix — update center, marketplace, Help / About, and support —
//! reads this primitive so advisory id, severity, affected surface, current exposure,
//! fixed version or mitigation, signer / source truth, and the primary action stay
//! consistent, and so the support export reconstructs the advisory from one shared
//! row model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_card_row_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_card_row_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_card_row_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_card_row_primitive -- fixture-extension-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_card_row_primitive -- fixture-signing-update-path-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_card_row_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_advisory_card_and_row_primitive::{
    seeded_m5_advisory_card_row_primitive_extension_beta_narrowed,
    seeded_m5_advisory_card_row_primitive_packet,
    seeded_m5_advisory_card_row_primitive_signing_update_path_preview_narrowed,
    M5AdvisoryRowPrimitivePacket,
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
            let packet = seeded_m5_advisory_card_row_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_advisory_card_row_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_advisory_card_row_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-extension-beta-narrowed") => {
            let packet = seeded_m5_advisory_card_row_primitive_extension_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-signing-update-path-preview-narrowed") => {
            let packet =
                seeded_m5_advisory_card_row_primitive_signing_update_path_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_advisory_card_row_primitive_packet(),
                seeded_m5_advisory_card_row_primitive_extension_beta_narrowed(),
                seeded_m5_advisory_card_row_primitive_signing_update_path_preview_narrowed(),
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

fn assert_valid(packet: &M5AdvisoryRowPrimitivePacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
