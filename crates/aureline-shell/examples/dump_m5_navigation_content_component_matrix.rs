//! Headless emitter for the frozen M5 navigation-content component matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-navigation-content-proof/`, its matrix CSV, the Markdown design report,
//! and the narrowed fixtures under `fixtures/ui/m5-navigation-content-components/`. Shell, explorer,
//! search, review, request/data, help, and support surfaces read this matrix so one tab strip names
//! the active context and overflow, one breadcrumb trail names the full or honestly truncated
//! hierarchy, one tree view names disclosure and selection, one list view names its count scopes and
//! hidden rows, one table/grid names counts and density, and one panel header names the active
//! context and a bounded action set.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_navigation_content_component_matrix -- support-export
//! cargo run -p aureline-shell --example dump_m5_navigation_content_component_matrix -- report
//! cargo run -p aureline-shell --example dump_m5_navigation_content_component_matrix -- csv
//! cargo run -p aureline-shell --example dump_m5_navigation_content_component_matrix -- fixture-table-grid-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_navigation_content_component_matrix -- fixture-tree-view-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_navigation_content_component_matrix -- validate
//! ```

use aureline_shell::freeze_the_m5_tab_strip_breadcrumbs_tree_view_list_view_table_grid_and_panel_header_component_matrix::{
    seeded_m5_navigation_content_component_matrix,
    seeded_m5_navigation_content_component_matrix_table_grid_beta_narrowed,
    seeded_m5_navigation_content_component_matrix_tree_view_preview_narrowed,
    M5NavigationContentComponentMatrixPacket,
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
            let packet = seeded_m5_navigation_content_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_navigation_content_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_navigation_content_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-table-grid-beta-narrowed") => {
            let packet = seeded_m5_navigation_content_component_matrix_table_grid_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-tree-view-preview-narrowed") => {
            let packet = seeded_m5_navigation_content_component_matrix_tree_view_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_navigation_content_component_matrix(),
                seeded_m5_navigation_content_component_matrix_table_grid_beta_narrowed(),
                seeded_m5_navigation_content_component_matrix_tree_view_preview_narrowed(),
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
    packet: &M5NavigationContentComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
