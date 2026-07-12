//! Headless emitter for the M5 tab-strip / breadcrumbs controls packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-tab-strip-breadcrumbs-controls-proof/`, its matrix CSV, the Markdown
//! summary, and the narrowed fixtures under `fixtures/ui/m5-tab-strip-breadcrumbs-controls/`.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_tab_strip_breadcrumbs_controls -- support-export
//! cargo run -p aureline-shell --example dump_m5_tab_strip_breadcrumbs_controls -- report
//! cargo run -p aureline-shell --example dump_m5_tab_strip_breadcrumbs_controls -- csv
//! cargo run -p aureline-shell --example dump_m5_tab_strip_breadcrumbs_controls -- fixture-shell-ui-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_tab_strip_breadcrumbs_controls -- fixture-search-ui-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_tab_strip_breadcrumbs_controls -- validate
//! ```

use aureline_shell::implement_the_m5_tab_strip_and_breadcrumbs_active_context_item_state_hierarchy_path_source_aware_context_and_no_top_level_navigation_drift_primitive::{
    seeded_m5_tab_strip_breadcrumbs_controls,
    seeded_m5_tab_strip_breadcrumbs_controls_search_ui_preview_narrowed,
    seeded_m5_tab_strip_breadcrumbs_controls_shell_ui_beta_narrowed,
    M5TabBreadcrumbsControlsPacket,
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
            let packet = seeded_m5_tab_strip_breadcrumbs_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_tab_strip_breadcrumbs_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_tab_strip_breadcrumbs_controls().render_matrix_csv()
            );
        }
        Some("fixture-shell-ui-beta-narrowed") => {
            let packet = seeded_m5_tab_strip_breadcrumbs_controls_shell_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-search-ui-preview-narrowed") => {
            let packet = seeded_m5_tab_strip_breadcrumbs_controls_search_ui_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_tab_strip_breadcrumbs_controls(),
                seeded_m5_tab_strip_breadcrumbs_controls_shell_ui_beta_narrowed(),
                seeded_m5_tab_strip_breadcrumbs_controls_search_ui_preview_narrowed(),
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

fn assert_valid(packet: &M5TabBreadcrumbsControlsPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
