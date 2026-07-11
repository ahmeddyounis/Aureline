//! Headless emitter for the M5 kernel-picker-row / kernel-origin-pill controls.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-kernel-picker-row-kernel-origin-pill-proof/`, its matrix CSV, the
//! Markdown design report, and the narrowed fixtures under
//! `fixtures/ui/m5-kernel-picker-row-kernel-origin-pill-controls/`. The notebook and kernel-manager
//! surfaces read these components so one kernel picker row names each candidate's kernel class,
//! environment identity, locality, compatibility state, trust / policy limits, and last-seen
//! availability — and offers choose / inspect / view-compatibility — and one kernel origin pill
//! names where the current kernel runs, how trusted it is, whether its provenance is exact or
//! degraded, and whether reattaching / rerunning keeps exact continuity — and offers inspect /
//! view-provenance / copy — so an incompatible, unavailable, or install-first candidate never reads
//! as a clean choice and a degraded or drifted origin never implies exact continuity.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_kernel_picker_row_kernel_origin_pill_primitive -- support-export
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_kernel_picker_row_kernel_origin_pill_primitive -- report
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_kernel_picker_row_kernel_origin_pill_primitive -- csv
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_kernel_picker_row_kernel_origin_pill_primitive -- fixture-kernel-picker-row-incompatible
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_kernel_picker_row_kernel_origin_pill_primitive -- fixture-kernel-origin-pill-degraded
//! cargo run -q -p aureline-notebook --bin aureline_notebook_m5_kernel_picker_row_kernel_origin_pill_primitive -- validate
//! ```

use aureline_notebook::implement_kernel_picker_rows_and_kernel_origin_pills_with_kernel_class_environment_identity_locality_trust_limits_exact_or_degraded_provenance_and_rerun_reattach_continuity_across_claimed_m5_notebook_surfaces::{
    seeded_kernel_picker_row_kernel_origin_pill_controls,
    seeded_kernel_picker_row_kernel_origin_pill_controls_kernel_origin_pill_degraded,
    seeded_kernel_picker_row_kernel_origin_pill_controls_kernel_picker_row_incompatible,
    KernelPickerRowKernelOriginPillControlsPacket,
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
            let packet = seeded_kernel_picker_row_kernel_origin_pill_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_kernel_picker_row_kernel_origin_pill_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_kernel_picker_row_kernel_origin_pill_controls().render_matrix_csv()
            );
        }
        Some("fixture-kernel-picker-row-incompatible") => {
            let packet =
                seeded_kernel_picker_row_kernel_origin_pill_controls_kernel_picker_row_incompatible(
                );
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-kernel-origin-pill-degraded") => {
            let packet =
                seeded_kernel_picker_row_kernel_origin_pill_controls_kernel_origin_pill_degraded();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_kernel_picker_row_kernel_origin_pill_controls(),
                seeded_kernel_picker_row_kernel_origin_pill_controls_kernel_picker_row_incompatible(
                ),
                seeded_kernel_picker_row_kernel_origin_pill_controls_kernel_origin_pill_degraded(),
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
    packet: &KernelPickerRowKernelOriginPillControlsPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "kernel picker row kernel origin pill primitive failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
