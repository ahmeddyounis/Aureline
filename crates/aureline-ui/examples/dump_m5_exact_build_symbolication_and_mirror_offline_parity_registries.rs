//! Headless emitter for the M5 clean-room-rebuild-lane and artifact-diff-packet registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-exact-build-symbolication-and-mirror-offline-parity-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-exact-build-symbolication-and-mirror-offline-parity-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_exact_build_symbolication_and_mirror_offline_parity_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_exact_build_symbolication_and_mirror_offline_parity_registries -- report
//! cargo run -p aureline-ui --example dump_m5_exact_build_symbolication_and_mirror_offline_parity_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_exact_build_symbolication_and_mirror_offline_parity_registries -- remote-cache-integrity-finding-table
//! cargo run -p aureline-ui --example dump_m5_exact_build_symbolication_and_mirror_offline_parity_registries -- fixture-hermetic-rebuild-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_exact_build_symbolication_and_mirror_offline_parity_registries -- fixture-artifact-diff-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_exact_build_symbolication_and_mirror_offline_parity_registries -- validate
//! ```

use aureline_ui::m5_exact_build_symbolication_and_mirror_offline_parity_registries::{
    seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries,
    seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries_artifact_diff_preview_narrowed,
    seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries_hermetic_rebuild_beta_narrowed,
    M5SymbolicationMirrorParityRegistriesPacket,
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
            let packet = seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries()
                    .render_matrix_csv()
            );
        }
        Some("remote-cache-integrity-finding-table") => {
            print!(
                "{}",
                seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries()
                    .render_exact_build_symbolication_table()
            );
        }
        Some("fixture-hermetic-rebuild-beta-narrowed") => {
            let packet =
                seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries_hermetic_rebuild_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-artifact-diff-preview-narrowed") => {
            let packet =
                seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries_artifact_diff_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries(),
                seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries_hermetic_rebuild_beta_narrowed(),
                seeded_m5_exact_build_symbolication_and_mirror_offline_parity_registries_artifact_diff_preview_narrowed(),
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

fn assert_valid(
    packet: &M5SymbolicationMirrorParityRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
