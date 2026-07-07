//! Headless emitter for the M5 write-scope-preview-tree primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-write-scope-preview-tree-primitive-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-write-scope-preview-tree-primitive/`. Rename, refactor, search/replace,
//! import, AI-apply, and repair previews all read this primitive so one write-scope tree
//! names its scope class, file-count bucket, workspace-root grouping, actor provenance, and
//! generated/read-only/conflict/exclusion truth, and one file node names its change type,
//! disposition, exclusion reason, and diff-jump path.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-history --bin aureline_history_write_scope_preview_tree_primitive -- support-export
//! cargo run -q -p aureline-history --bin aureline_history_write_scope_preview_tree_primitive -- report
//! cargo run -q -p aureline-history --bin aureline_history_write_scope_preview_tree_primitive -- csv
//! cargo run -q -p aureline-history --bin aureline_history_write_scope_preview_tree_primitive -- fixture-import-preview-preview-narrowed
//! cargo run -q -p aureline-history --bin aureline_history_write_scope_preview_tree_primitive -- fixture-ai-apply-preview-beta-narrowed
//! cargo run -q -p aureline-history --bin aureline_history_write_scope_preview_tree_primitive -- validate
//! ```

use aureline_history::implement_write_scope_preview_trees_with_file_count_buckets_actor_provenance_selectable_scope_diff_jump_and_generated_read_only_conflict_exclusion_truth_across_claimed_m5_multi_file_change_flows::{
    seeded_m5_write_scope_preview_tree_ai_apply_preview_beta_narrowed,
    seeded_m5_write_scope_preview_tree_import_preview_preview_narrowed,
    seeded_m5_write_scope_preview_tree_packet, M5WriteScopePreviewTreePacket,
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
            let packet = seeded_m5_write_scope_preview_tree_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_write_scope_preview_tree_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_write_scope_preview_tree_packet().render_matrix_csv()
            );
        }
        Some("fixture-import-preview-preview-narrowed") => {
            let packet = seeded_m5_write_scope_preview_tree_import_preview_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-ai-apply-preview-beta-narrowed") => {
            let packet = seeded_m5_write_scope_preview_tree_ai_apply_preview_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_write_scope_preview_tree_packet(),
                seeded_m5_write_scope_preview_tree_import_preview_preview_narrowed(),
                seeded_m5_write_scope_preview_tree_ai_apply_preview_beta_narrowed(),
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

fn assert_valid(packet: &M5WriteScopePreviewTreePacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("primitive failed validation: {}", tokens.join(",")).into())
    }
}
