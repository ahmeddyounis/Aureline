//! Headless emitter for the frozen M5 notebook-kernel-output component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-notebook-kernel-output-proof/`, its matrix CSV, the Markdown design
//! report, and the narrowed fixtures under `fixtures/ui/m5-notebook-kernel-output-components/`.
//! Notebook, kernel-manager, output-viewer, debugger, AI-context, review, and CLI surfaces read
//! this matrix so one notebook document header names its canonical `.ipynb` identity and source,
//! one kernel state strip names where a kernel stands, one kernel picker row names its
//! candidates, one kernel origin pill never collapses local, SSH, container, managed, or
//! browser-bridge kernels into one unlabeled badge, one output trust banner never presents stale
//! output as live and never hides its trust class behind hover, one provenance chip group names
//! an output's producing run, one restart consequence card names preserved-versus-lost state, and
//! one kernel recovery card offers reconnect / restart-clean / choose-another-kernel recovery
//! without ever implying a rerun.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_notebook_kernel_output_component_matrix -- support-export
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_notebook_kernel_output_component_matrix -- report
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_notebook_kernel_output_component_matrix -- csv
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_notebook_kernel_output_component_matrix -- fixture-kernel-recovery-card-beta-narrowed
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_notebook_kernel_output_component_matrix -- fixture-output-trust-banner-preview-narrowed
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_notebook_kernel_output_component_matrix -- validate
//! ```

use aureline_notebook::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix::{
    seeded_m5_notebook_kernel_output_component_matrix,
    seeded_m5_notebook_kernel_output_component_matrix_kernel_recovery_card_beta_narrowed,
    seeded_m5_notebook_kernel_output_component_matrix_output_trust_banner_preview_narrowed,
    M5NotebookKernelOutputComponentMatrixPacket,
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
            let packet = seeded_m5_notebook_kernel_output_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_notebook_kernel_output_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_notebook_kernel_output_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-kernel-recovery-card-beta-narrowed") => {
            let packet =
                seeded_m5_notebook_kernel_output_component_matrix_kernel_recovery_card_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-output-trust-banner-preview-narrowed") => {
            let packet =
                seeded_m5_notebook_kernel_output_component_matrix_output_trust_banner_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_notebook_kernel_output_component_matrix(),
                seeded_m5_notebook_kernel_output_component_matrix_kernel_recovery_card_beta_narrowed(),
                seeded_m5_notebook_kernel_output_component_matrix_output_trust_banner_preview_narrowed(),
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
    packet: &M5NotebookKernelOutputComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
