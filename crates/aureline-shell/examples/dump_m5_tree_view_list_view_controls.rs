//! Headless emitter for the M5 tree-view / list-view controls packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-tree-view-list-view-controls-proof/`, its matrix CSV, the Markdown summary,
//! and the narrowed fixtures under `fixtures/ui/m5-tree-view-list-view-controls/`.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_tree_view_list_view_controls -- support-export
//! cargo run -p aureline-shell --example dump_m5_tree_view_list_view_controls -- report
//! cargo run -p aureline-shell --example dump_m5_tree_view_list_view_controls -- csv
//! cargo run -p aureline-shell --example dump_m5_tree_view_list_view_controls -- fixture-explorer-ui-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_tree_view_list_view_controls -- fixture-review-ui-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_tree_view_list_view_controls -- validate
//! ```

use aureline_shell::implement_the_m5_tree_view_and_list_view_virtualization_disclosure_selection_focus_inline_action_budget_and_exact_loaded_hidden_scope_primitive::{
    seeded_m5_tree_view_list_view_controls,
    seeded_m5_tree_view_list_view_controls_explorer_ui_beta_narrowed,
    seeded_m5_tree_view_list_view_controls_review_ui_preview_narrowed,
    M5TreeListControlsPacket,
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
            let packet = seeded_m5_tree_view_list_view_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_tree_view_list_view_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_tree_view_list_view_controls().render_matrix_csv()
            );
        }
        Some("fixture-explorer-ui-beta-narrowed") => {
            let packet = seeded_m5_tree_view_list_view_controls_explorer_ui_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-review-ui-preview-narrowed") => {
            let packet = seeded_m5_tree_view_list_view_controls_review_ui_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_tree_view_list_view_controls(),
                seeded_m5_tree_view_list_view_controls_explorer_ui_beta_narrowed(),
                seeded_m5_tree_view_list_view_controls_review_ui_preview_narrowed(),
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

fn assert_valid(packet: &M5TreeListControlsPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
