//! Headless emitter for the M5 sequence-help-strip primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-sequence-help-strip-primitive-proof/`, its matrix CSV, the Markdown
//! design report, and the narrowed fixtures under `fixtures/ui/m5-sequence-help-strip-primitive/`.
//! The leader-sequence overlay, modal-operator strip, partial-command hint, command-palette
//! sequence hint, and support sequence export consumers read this matrix so one sequence-help
//! strip names the current mode or leader key, the valid next keys, the cancel key, an example
//! command, and a way to open the full cheat sheet — never letting a partial or ambiguous
//! sequence fail silently, never requiring pointer hover, and always carrying a screen-reader
//! announcement.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_sequence_help_strip_primitive -- support-export
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_sequence_help_strip_primitive -- report
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_sequence_help_strip_primitive -- csv
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_sequence_help_strip_primitive -- fixture-command-palette-sequence-hint-beta-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_sequence_help_strip_primitive -- fixture-support-sequence-export-preview-narrowed
//! cargo run -q -p aureline-learning --bin aureline_learning_m5_sequence_help_strip_primitive -- validate
//! ```

use aureline_learning::implement_sequence_help_strips_with_current_mode_next_key_guidance_cancel_hints_and_keyboard_only_parity_across_claimed_m5_modal_and_command_language_surfaces::{
    seeded_m5_sequence_help_strip_command_palette_sequence_hint_beta_narrowed,
    seeded_m5_sequence_help_strip_packet,
    seeded_m5_sequence_help_strip_support_sequence_export_preview_narrowed,
    M5SequenceHelpStripPacket,
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
            let packet = seeded_m5_sequence_help_strip_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_sequence_help_strip_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_sequence_help_strip_packet().render_matrix_csv()
            );
        }
        Some("fixture-command-palette-sequence-hint-beta-narrowed") => {
            let packet =
                seeded_m5_sequence_help_strip_command_palette_sequence_hint_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-support-sequence-export-preview-narrowed") => {
            let packet = seeded_m5_sequence_help_strip_support_sequence_export_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_sequence_help_strip_packet(),
                seeded_m5_sequence_help_strip_command_palette_sequence_hint_beta_narrowed(),
                seeded_m5_sequence_help_strip_support_sequence_export_preview_narrowed(),
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

fn assert_valid(packet: &M5SequenceHelpStripPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "sequence help strip primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
