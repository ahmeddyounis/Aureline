//! Emits the seeded qualification matrix fixtures.
//!
//! ```sh
//! cargo run -q -p aureline-remote --example dump_qualification_matrix_fixtures -- page
//! cargo run -q -p aureline-remote --example dump_qualification_matrix_fixtures -- rows
//! cargo run -q -p aureline-remote --example dump_qualification_matrix_fixtures -- defects
//! cargo run -q -p aureline-remote --example dump_qualification_matrix_fixtures -- summary
//! cargo run -q -p aureline-remote --example dump_qualification_matrix_fixtures -- support-export
//! cargo run -q -p aureline-remote --example dump_qualification_matrix_fixtures -- drill-missing-row-preview
//! cargo run -q -p aureline-remote --example dump_qualification_matrix_fixtures -- drill-raw-material-withdrawn
//! cargo run -q -p aureline-remote --example dump_qualification_matrix_fixtures -- drill-no-local-core-continuity-beta
//! ```

use aureline_remote::{
    seeded_qualification_matrix_page, seeded_qualification_snapshot,
    QualificationMatrixPage, QualificationMatrixSupportExport,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_qualification_matrix_page();
    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("rows") => print_json(&page.rows)?,
        Some("defects") => print_json(&page.defects)?,
        Some("summary") => print_json(&page.summary)?,
        Some("support-export") => {
            let export = QualificationMatrixSupportExport::from_page(
                "remote:qualification-matrix:support-export:fixture-001",
                "2026-06-01T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("drill-missing-row-preview") => {
            let mut snapshot = seeded_qualification_snapshot();
            snapshot
                .records
                .retain(|r| r.row_key != "desktop_local:local_oss");
            let drill_page = QualificationMatrixPage::new(
                "remote:qualification-matrix:drill:missing-row",
                "Drill — required row absent (preview)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill_page)?;
        }
        Some("drill-raw-material-withdrawn") => {
            let mut snapshot = seeded_qualification_snapshot();
            for rec in snapshot.records.iter_mut() {
                if rec.row_key == "desktop_local:managed" {
                    rec.raw_private_material_excluded = false;
                }
            }
            let drill_page = QualificationMatrixPage::new(
                "remote:qualification-matrix:drill:raw-material-withdrawn",
                "Drill — raw private material exposed (withdrawn)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill_page)?;
        }
        Some("drill-no-local-core-continuity-beta") => {
            let mut snapshot = seeded_qualification_snapshot();
            for rec in snapshot.records.iter_mut() {
                if rec.row_key == "remote_helper:managed" {
                    rec.local_core_continuity_allowed = false;
                }
            }
            let drill_page = QualificationMatrixPage::new(
                "remote:qualification-matrix:drill:no-local-core-continuity",
                "Drill — no local-core continuity declared on remote_helper:managed (beta)",
                "2026-06-01T00:00:00Z",
                snapshot,
            );
            print_json(&drill_page)?;
        }
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
