//! Headless emitter for the frozen M5 embedded-boundary component matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-embedded-boundary-proof/`, its matrix CSV, the Markdown design report,
//! and the narrowed fixtures under `fixtures/ui/m5-embedded-boundary-components/`. Docs/help,
//! marketplace/account, remote/service dashboard, embedded webview, and browser/device-code
//! auth-handoff surfaces read this matrix so one docs-pane header names owner/origin and
//! freshness, one embedded-origin bar names owner and capability limits, one boundary-fact grid
//! names owner/origin, data boundary, and freshness together, one marketplace/account boundary
//! card names account scope, one auth-handoff card names the browser fallback and data boundary,
//! one remote/service dashboard header names provider health and freshness, one open-in-browser
//! handoff row names the browser fallback, and one embedded-state panel names a stale, offline,
//! or provider-blocked state explicitly.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_embedded_boundary_component_matrix -- support-export
//! cargo run -p aureline-shell --example dump_m5_embedded_boundary_component_matrix -- report
//! cargo run -p aureline-shell --example dump_m5_embedded_boundary_component_matrix -- csv
//! cargo run -p aureline-shell --example dump_m5_embedded_boundary_component_matrix -- fixture-docs-pane-header-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_embedded_boundary_component_matrix -- fixture-embedded-state-panel-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_embedded_boundary_component_matrix -- validate
//! ```

use aureline_shell::freeze_the_m5_docs_pane_header_embedded_origin_bar_boundary_fact_grid_marketplace_account_boundary_card_auth_handoff_card_remote_service_dashboard_header_open_in_browser_handoff_row_and_embedded_state_panel_component_matrix::{
    seeded_m5_embedded_boundary_component_matrix,
    seeded_m5_embedded_boundary_component_matrix_docs_pane_header_beta_narrowed,
    seeded_m5_embedded_boundary_component_matrix_embedded_state_panel_preview_narrowed,
    M5EmbeddedBoundaryComponentMatrixPacket,
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
            let packet = seeded_m5_embedded_boundary_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_embedded_boundary_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_embedded_boundary_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-docs-pane-header-beta-narrowed") => {
            let packet =
                seeded_m5_embedded_boundary_component_matrix_docs_pane_header_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-embedded-state-panel-preview-narrowed") => {
            let packet =
                seeded_m5_embedded_boundary_component_matrix_embedded_state_panel_preview_narrowed(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_embedded_boundary_component_matrix(),
                seeded_m5_embedded_boundary_component_matrix_docs_pane_header_beta_narrowed(),
                seeded_m5_embedded_boundary_component_matrix_embedded_state_panel_preview_narrowed(
                ),
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
    packet: &M5EmbeddedBoundaryComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
