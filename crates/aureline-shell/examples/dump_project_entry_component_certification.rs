//! Regenerates the checked-in M05-843 project-entry component surface
//! certification proof artifacts (support export, matrix CSV, and Markdown
//! report) from the seeded builder so the on-disk evidence stays byte-aligned
//! with the Rust source.
//!
//! Run with:
//! `cargo run -p aureline-shell --example dump_project_entry_component_certification`

use std::fs;
use std::path::PathBuf;

use aureline_shell::m5_project_entry_component_certification::seeded_m5_project_entry_component_certification_packet;

fn main() {
    let packet = seeded_m5_project_entry_component_certification_packet();
    assert!(
        packet.validate().is_empty(),
        "refusing to write an invalid packet: {:?}",
        packet.validate()
    );

    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../artifacts/release/m5-project-entry-component-certification-proof");
    fs::create_dir_all(&dir).expect("create artifact dir");

    fs::write(
        dir.join("support_export.json"),
        format!("{}\n", packet.export_safe_json()),
    )
    .expect("write support export");
    fs::write(dir.join("matrix.csv"), packet.render_matrix_csv()).expect("write matrix csv");
    fs::write(dir.join("report.md"), packet.render_markdown_summary()).expect("write report");

    println!(
        "wrote {} certified surfaces to {}",
        packet.rows.len(),
        dir.display()
    );
}
