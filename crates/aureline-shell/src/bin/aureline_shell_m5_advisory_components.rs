//! Headless emitter for the frozen M5 security-advisory, emergency-notice,
//! affected-install, and disclosure-link component matrix.
//!
//! The bin is the only mint-from-truth path for the support export checked in
//! under `artifacts/release/m5-advisory-proof/`, its matrix CSV, the Markdown
//! report `artifacts/security/m5-advisory-component-matrix.md`, and the narrowed
//! fixtures under `fixtures/security/m5-advisory-scenarios/`. Update, marketplace,
//! Help/About, support bundles, native notifications, and mirror/offline drills
//! read this matrix so one advisory model names the affected object, severity,
//! exposure, fix/mitigation, signer/source state, and primary actions without
//! hiding local continuity, one emergency-notice model stays explicit about blast
//! radius and dismissal rules, and mirror lag or unsigned distribution
//! auto-narrows the claim.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_components -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_components -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_components -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_components -- fixture-emergency-notice-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_components -- fixture-affected-install-panel-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_advisory_components -- validate
//! ```

use aureline_shell::freeze_the_m5_security_advisory_emergency_notice_affected_install_and_disclosure_link_matrix::{
    seeded_m5_advisory_component_matrix,
    seeded_m5_advisory_component_matrix_affected_install_panel_preview_narrowed,
    seeded_m5_advisory_component_matrix_emergency_notice_beta_narrowed,
    M5AdvisoryComponentMatrixPacket,
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
            let packet = seeded_m5_advisory_component_matrix();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_advisory_component_matrix().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_advisory_component_matrix().render_matrix_csv()
            );
        }
        Some("fixture-emergency-notice-beta-narrowed") => {
            let packet = seeded_m5_advisory_component_matrix_emergency_notice_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-affected-install-panel-preview-narrowed") => {
            let packet =
                seeded_m5_advisory_component_matrix_affected_install_panel_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_advisory_component_matrix(),
                seeded_m5_advisory_component_matrix_emergency_notice_beta_narrowed(),
                seeded_m5_advisory_component_matrix_affected_install_panel_preview_narrowed(),
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
    packet: &M5AdvisoryComponentMatrixPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}
