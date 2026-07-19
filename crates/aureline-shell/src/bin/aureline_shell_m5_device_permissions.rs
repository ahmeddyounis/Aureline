//! Headless emitter for the M5 device-permission set.
//!
//! The bin is the headless stdout projection path for the support export checked in at
//! `artifacts/help/m5-device-permission-proof/permission_set.json`, the
//! governance Markdown summary
//! `artifacts/help/m5-device-permission-governance.md`, the matrix CSV
//! `artifacts/help/m5-device-permission-rows.csv`, and the narrowed fixtures
//! under `fixtures/help/device-permissions/`. Voice, help, and support surfaces
//! read this set so users can see what device class is accessible, whether
//! capture is local or provider-backed, and what transcript or media will be
//! retained or exported. The guarded `generate_artifacts` module test refreshes
//! every checked projection from the same seed builders in one operation.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_device_permissions -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_device_permissions -- governance
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_device_permissions -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_device_permissions -- fixture-high-impact-confirmation-pill
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_device_permissions -- fixture-provider-backed-capture-review
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_device_permissions -- validate
//! ```

use aureline_shell::m5_device_permissions::{
    seeded_high_impact_confirmation_pill, seeded_m5_device_permission_set,
    seeded_provider_backed_capture_review,
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
            let set = seeded_m5_device_permission_set();
            set.validate()?;
            println!("{}", set.export_safe_json());
        }
        Some("governance") => {
            let set = seeded_m5_device_permission_set();
            set.validate()?;
            print!("{}", set.render_markdown_summary());
        }
        Some("csv") => {
            let set = seeded_m5_device_permission_set();
            set.validate()?;
            print!("{}", set.render_matrix_csv());
        }
        Some("fixture-high-impact-confirmation-pill") => {
            let pill = seeded_high_impact_confirmation_pill();
            pill.validate()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&pill).expect("pill serializes")
            );
        }
        Some("fixture-provider-backed-capture-review") => {
            let review = seeded_provider_backed_capture_review();
            review.validate()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&review).expect("review serializes")
            );
        }
        Some("validate") => {
            seeded_m5_device_permission_set().validate()?;
            seeded_high_impact_confirmation_pill().validate()?;
            seeded_provider_backed_capture_review().validate()?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}
