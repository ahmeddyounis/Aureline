//! Headless emitter for the M5 notebook-document-header / kernel-state-strip controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-notebook-document-header-kernel-state-strip-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-notebook-document-header-kernel-state-strip-controls/`. The notebook edit
//! surface and the kernel-manager surface read these components so one document header names its
//! canonical `.ipynb` identity, where the notebook came from, where its saved / unsaved /
//! conflicted / read-only / recovered identity stands, its paired export state, and its current
//! target / workspace context — and offers open / export / review — and one kernel-state strip
//! names its selected kernel origin, its execution and connection state, and its derived live
//! class — and offers select / inspect / continue-without-kernel — so an imported, scratch, or
//! unknown-source notebook never reads as a settled canonical source and a kernel-free or
//! disconnected notebook never reads as live while staying explicitly editable.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_notebook_document_header_kernel_state_strip_primitive -- support-export
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_notebook_document_header_kernel_state_strip_primitive -- report
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_notebook_document_header_kernel_state_strip_primitive -- csv
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_notebook_document_header_kernel_state_strip_primitive -- fixture-document-header-scratch
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_notebook_document_header_kernel_state_strip_primitive -- fixture-kernel-state-strip-no-kernel
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_notebook_document_header_kernel_state_strip_primitive -- validate
//! ```

use aureline_notebook::implement_notebook_document_headers_and_kernel_state_strips_with_canonical_ipynb_source_selected_kernel_origin_busy_queued_offline_truth_and_no_kernel_edit_parity_across_claimed_m5_notebook_surfaces::{
    seeded_notebook_document_header_kernel_state_strip_controls,
    seeded_notebook_document_header_kernel_state_strip_controls_document_header_scratch,
    seeded_notebook_document_header_kernel_state_strip_controls_kernel_state_strip_no_kernel,
    NotebookDocumentHeaderKernelStateStripControlsPacket,
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
            let packet = seeded_notebook_document_header_kernel_state_strip_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_notebook_document_header_kernel_state_strip_controls()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_notebook_document_header_kernel_state_strip_controls().render_matrix_csv()
            );
        }
        Some("fixture-document-header-scratch") => {
            let packet =
                seeded_notebook_document_header_kernel_state_strip_controls_document_header_scratch(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-kernel-state-strip-no-kernel") => {
            let packet =
                seeded_notebook_document_header_kernel_state_strip_controls_kernel_state_strip_no_kernel();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_notebook_document_header_kernel_state_strip_controls(),
                seeded_notebook_document_header_kernel_state_strip_controls_document_header_scratch(),
                seeded_notebook_document_header_kernel_state_strip_controls_kernel_state_strip_no_kernel(),
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
    packet: &NotebookDocumentHeaderKernelStateStripControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "notebook document header kernel strip primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
