//! Headless emitter for the M5 spacing / sizing / radii / border / elevation geometry and hit-target
//! registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-spacing-sizing-radii-elevation-and-hit-target-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-spacing-sizing-radii-elevation-and-hit-target-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_geometry_hit_target_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_geometry_hit_target_registries -- report
//! cargo run -p aureline-ui --example dump_m5_geometry_hit_target_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_geometry_hit_target_registries -- fixture-shell-ui-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_geometry_hit_target_registries -- fixture-data-ui-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_geometry_hit_target_registries -- validate
//! ```

use aureline_ui::m5_spacing_sizing_radii_elevation_and_hit_target_registries::{
    seeded_m5_geometry_hit_target_registries,
    seeded_m5_geometry_hit_target_registries_data_ui_preview_narrowed,
    seeded_m5_geometry_hit_target_registries_shell_ui_beta_narrowed,
    M5GeometryHitTargetRegistriesPacket,
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
            let packet = seeded_m5_geometry_hit_target_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_geometry_hit_target_registries().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_geometry_hit_target_registries().render_matrix_csv()
            );
        }
        Some("fixture-shell-ui-beta-narrowed") => {
            let packet = seeded_m5_geometry_hit_target_registries_shell_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-data-ui-preview-narrowed") => {
            let packet = seeded_m5_geometry_hit_target_registries_data_ui_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_geometry_hit_target_registries(),
                seeded_m5_geometry_hit_target_registries_shell_ui_beta_narrowed(),
                seeded_m5_geometry_hit_target_registries_data_ui_preview_narrowed(),
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
    packet: &M5GeometryHitTargetRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
