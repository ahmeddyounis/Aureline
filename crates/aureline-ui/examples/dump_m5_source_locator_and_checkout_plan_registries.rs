//! Headless emitter for the M5 source-locator and checkout-plan registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-source-locator-and-checkout-plan-registries-proof/`, its matrix CSV, the Markdown
//! summary, and the narrowed fixtures under
//! `fixtures/workspaces/m5-source-locator-and-checkout-plan-registries/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- support-export
//! cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- report
//! cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- csv
//! cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- source-acquisition-table
//! cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- fixture-local-path-source-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- fixture-sparse-checkout-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_source_locator_and_checkout_plan_registries -- validate
//! ```

use aureline_ui::m5_source_locator_and_checkout_plan_registries::{
    seeded_m5_source_locator_and_checkout_plan_registries,
    seeded_m5_source_locator_and_checkout_plan_registries_local_path_source_beta_narrowed,
    seeded_m5_source_locator_and_checkout_plan_registries_sparse_checkout_preview_narrowed,
    M5SourceLocatorCheckoutPlanRegistriesPacket,
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
            let packet = seeded_m5_source_locator_and_checkout_plan_registries();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_source_locator_and_checkout_plan_registries().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_source_locator_and_checkout_plan_registries().render_matrix_csv()
            );
        }
        Some("source-acquisition-table") => {
            print!(
                "{}",
                seeded_m5_source_locator_and_checkout_plan_registries()
                    .render_source_acquisition_table()
            );
        }
        Some("fixture-local-path-source-beta-narrowed") => {
            let packet =
                seeded_m5_source_locator_and_checkout_plan_registries_local_path_source_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-sparse-checkout-preview-narrowed") => {
            let packet =
                seeded_m5_source_locator_and_checkout_plan_registries_sparse_checkout_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_source_locator_and_checkout_plan_registries(),
                seeded_m5_source_locator_and_checkout_plan_registries_local_path_source_beta_narrowed(),
                seeded_m5_source_locator_and_checkout_plan_registries_sparse_checkout_preview_narrowed(),
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
    packet: &M5SourceLocatorCheckoutPlanRegistriesPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
