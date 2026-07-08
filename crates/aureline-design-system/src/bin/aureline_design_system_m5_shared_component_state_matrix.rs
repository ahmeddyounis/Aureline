//! Headless emitter for the frozen M5 shared-component-state matrix.
//!
//! The bin is the only mint-from-truth path for the checked-in support export at
//! `artifacts/release/m5-shared-state-taxonomy-proof/support_export.json`, the machine-readable
//! matrix CSV at `artifacts/release/m5-shared-state-taxonomy-proof/matrix.csv`, the Markdown
//! design report at `artifacts/design/m5-shared-state-taxonomy-component-matrix.md`, and the two
//! narrowed fixtures under `fixtures/ui/m5-shared-state-taxonomy/`. Controls, collections,
//! prompts, recovery surfaces, shell status/progress, and settings sheets read the state
//! semantics this bin mints, so component-state truth stays one governed vocabulary.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_shared_component_state_matrix -- export
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_shared_component_state_matrix -- csv
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_shared_component_state_matrix -- report
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_shared_component_state_matrix -- fixture interactive-state-beta
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_shared_component_state_matrix -- fixture degraded-state-application-preview
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_shared_component_state_matrix -- validate
//! ```

use aureline_design_system::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix::{
    seeded_m5_shared_component_state_matrix,
    seeded_m5_shared_component_state_matrix_degraded_state_application_preview_narrowed,
    seeded_m5_shared_component_state_matrix_interactive_state_beta_narrowed,
    M5SharedComponentStateMatrixPacket,
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
        Some("export") | None => {
            let packet = seeded_m5_shared_component_state_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("csv") => {
            let packet = seeded_m5_shared_component_state_matrix();
            assert_valid(&packet)?;
            print!("{}", packet.render_matrix_csv());
        }
        Some("report") => {
            let packet = seeded_m5_shared_component_state_matrix();
            assert_valid(&packet)?;
            print!("{}", packet.render_markdown_summary());
        }
        Some("fixture") => {
            let name = args.get(1).ok_or("fixture requires a variant argument")?;
            let packet = match name.as_str() {
                "interactive-state-beta" => {
                    seeded_m5_shared_component_state_matrix_interactive_state_beta_narrowed()
                }
                "degraded-state-application-preview" => {
                    seeded_m5_shared_component_state_matrix_degraded_state_application_preview_narrowed()
                }
                other => return Err(format!("unknown fixture variant: {other}").into()),
            };
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            assert_valid(&seeded_m5_shared_component_state_matrix())?;
            assert_valid(
                &seeded_m5_shared_component_state_matrix_interactive_state_beta_narrowed(),
            )?;
            assert_valid(
                &seeded_m5_shared_component_state_matrix_degraded_state_application_preview_narrowed(),
            )?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(
    packet: &M5SharedComponentStateMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "shared-component-state matrix failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
