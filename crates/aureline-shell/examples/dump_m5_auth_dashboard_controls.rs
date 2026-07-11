//! Headless emitter for the M5 auth handoff-card and remote/service dashboard-header controls
//! packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-auth-handoff-card-remote-service-dashboard-header-controls-proof/`, its
//! matrix CSV, the Markdown summary, and the narrowed fixtures under
//! `fixtures/ui/m5-auth-handoff-card-remote-service-dashboard-header-controls/`.
//!
//! ```text
//! cargo run -p aureline-shell --example dump_m5_auth_dashboard_controls -- support-export
//! cargo run -p aureline-shell --example dump_m5_auth_dashboard_controls -- report
//! cargo run -p aureline-shell --example dump_m5_auth_dashboard_controls -- csv
//! cargo run -p aureline-shell --example dump_m5_auth_dashboard_controls -- fixture-auth-handoff-beta-narrowed
//! cargo run -p aureline-shell --example dump_m5_auth_dashboard_controls -- fixture-remote-dashboard-preview-narrowed
//! cargo run -p aureline-shell --example dump_m5_auth_dashboard_controls -- validate
//! ```

use aureline_shell::implement_the_m5_auth_handoff_card_and_remote_service_dashboard_header_provider_domain_reason_fallback_local_continuity_device_code_expiry_target_service_identity_freshness_export_open_console_and_no_embedded_high_risk_approval_primitive::{
    seeded_m5_auth_dashboard_controls, seeded_m5_auth_dashboard_controls_auth_handoff_beta_narrowed,
    seeded_m5_auth_dashboard_controls_remote_dashboard_preview_narrowed,
    M5AuthDashboardControlsPacket,
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
            let packet = seeded_m5_auth_dashboard_controls();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_auth_dashboard_controls().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_auth_dashboard_controls().render_matrix_csv()
            );
        }
        Some("fixture-auth-handoff-beta-narrowed") => {
            let packet = seeded_m5_auth_dashboard_controls_auth_handoff_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-remote-dashboard-preview-narrowed") => {
            let packet = seeded_m5_auth_dashboard_controls_remote_dashboard_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_auth_dashboard_controls(),
                seeded_m5_auth_dashboard_controls_auth_handoff_beta_narrowed(),
                seeded_m5_auth_dashboard_controls_remote_dashboard_preview_narrowed(),
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

fn assert_valid(packet: &M5AuthDashboardControlsPacket) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("controls packet failed validation: {}", tokens.join(",")).into())
    }
}
