//! Headless emitter for the M5 freeze-exception and go-no-go registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-freeze-exception-and-go-no-go-registries-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/release/m5-freeze-exception-and-go-no-go-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- report
//! cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- freeze-exception-table
//! cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- fixture-freeze-exception-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- fixture-go-no-go-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_freeze_exception_and_go_no_go_registries -- validate
//! ```

use aureline_ui::m5_freeze_exception_and_go_no_go_registries::{
    seeded_m5_freeze_exception_and_go_no_go_registries,
    seeded_m5_freeze_exception_and_go_no_go_registries_freeze_exception_beta_narrowed,
    seeded_m5_freeze_exception_and_go_no_go_registries_go_no_go_preview_narrowed,
    M5FreezeExceptionGoNoGoRegistriesPacket,
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
            let packet = seeded_m5_freeze_exception_and_go_no_go_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_freeze_exception_and_go_no_go_registries().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_freeze_exception_and_go_no_go_registries().render_matrix_csv()
            );
        }
        Some("freeze-exception-table") => {
            print!(
                "{}",
                seeded_m5_freeze_exception_and_go_no_go_registries()
                    .render_freeze_exception_table()
            );
        }
        Some("fixture-freeze-exception-beta-narrowed") => {
            let packet =
                seeded_m5_freeze_exception_and_go_no_go_registries_freeze_exception_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-go-no-go-preview-narrowed") => {
            let packet =
                seeded_m5_freeze_exception_and_go_no_go_registries_go_no_go_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_freeze_exception_and_go_no_go_registries(),
                seeded_m5_freeze_exception_and_go_no_go_registries_freeze_exception_beta_narrowed(),
                seeded_m5_freeze_exception_and_go_no_go_registries_go_no_go_preview_narrowed(),
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
    packet: &M5FreezeExceptionGoNoGoRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
