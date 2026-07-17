//! Headless emitter for the frozen M5 change-intent matrix.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-change-intent-proof/`, its matrix CSV, the Markdown design report at
//! `artifacts/design/m5-change-intent-component-matrix.md`, the change-intent-health dashboard at
//! `dashboards/m5-change-intent-health.json`, and the narrowed fixtures under
//! `fixtures/teamwork/m5-change-intent/`. The work-item, start-work, review, provider handoff, help / docs, and
//! support / export surfaces read this matrix so a local handoff packet never masquerades as a provider-committed
//! update, no start-work side effect is created without disclosure, the four relation sources are never
//! flattened, tracked work is never auto-resolved while engineering blockers remain, and no local notes, handoff
//! packet, or linked evidence is dropped when a provider write fails.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- support-export
//! cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- report
//! cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- csv
//! cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- dashboard
//! cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- fixture-start-work-sheet-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- fixture-blocked-escalate-card-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_change_intent_matrix -- validate
//! ```

use aureline_ui::m5_change_intent_and_engineering_lifecycle_matrix::{
    seeded_m5_change_intent_matrix,
    seeded_m5_change_intent_matrix_blocked_escalate_card_preview_narrowed,
    seeded_m5_change_intent_matrix_start_work_sheet_beta_narrowed, M5ChangeIntentMatrixPacket,
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
            let packet = seeded_m5_change_intent_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_change_intent_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!("{}", seeded_m5_change_intent_matrix().render_matrix_csv());
        }
        Some("dashboard") => {
            println!(
                "{}",
                seeded_m5_change_intent_matrix().render_dashboard_json()
            );
        }
        Some("fixture-start-work-sheet-beta-narrowed") => {
            let packet = seeded_m5_change_intent_matrix_start_work_sheet_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-blocked-escalate-card-preview-narrowed") => {
            let packet = seeded_m5_change_intent_matrix_blocked_escalate_card_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_change_intent_matrix(),
                seeded_m5_change_intent_matrix_start_work_sheet_beta_narrowed(),
                seeded_m5_change_intent_matrix_blocked_escalate_card_preview_narrowed(),
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

fn assert_valid(packet: &M5ChangeIntentMatrixPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
