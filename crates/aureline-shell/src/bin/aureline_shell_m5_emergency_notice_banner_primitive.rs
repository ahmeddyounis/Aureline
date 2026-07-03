//! Headless emitter for the M5 emergency-notice banner primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-emergency-notice-banner-proof/`, its matrix CSV, the Markdown
//! report `artifacts/security/m5-emergency-notice-banner-primitive.md`, and the
//! narrowed fixtures under `fixtures/security/m5-emergency-notice-banner-primitive/`.
//! Every M5 surface that has to raise an emergency — update center, extension host,
//! native notification, and support — reads this primitive so the reason class,
//! affected capability, blast radius, local-work continuity, deadline / urgency, the
//! primary / recovery actions, and the acknowledge / snooze / dismiss rules stay
//! consistent, and so the support export reconstructs the emergency from one shared
//! banner model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_emergency_notice_banner_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_emergency_notice_banner_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_emergency_notice_banner_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_emergency_notice_banner_primitive -- fixture-forced-disable-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_emergency_notice_banner_primitive -- fixture-signed-emergency-bundle-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_emergency_notice_banner_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_emergency_notice_banner_primitive::{
    seeded_m5_emergency_notice_banner_primitive_forced_disable_beta_narrowed,
    seeded_m5_emergency_notice_banner_primitive_packet,
    seeded_m5_emergency_notice_banner_primitive_signed_emergency_bundle_preview_narrowed,
    M5EmergencyBannerPrimitivePacket,
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
            let packet = seeded_m5_emergency_notice_banner_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_emergency_notice_banner_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_emergency_notice_banner_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-forced-disable-beta-narrowed") => {
            let packet = seeded_m5_emergency_notice_banner_primitive_forced_disable_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-signed-emergency-bundle-preview-narrowed") => {
            let packet =
                seeded_m5_emergency_notice_banner_primitive_signed_emergency_bundle_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_emergency_notice_banner_primitive_packet(),
                seeded_m5_emergency_notice_banner_primitive_forced_disable_beta_narrowed(),
                seeded_m5_emergency_notice_banner_primitive_signed_emergency_bundle_preview_narrowed(),
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
    packet: &M5EmergencyBannerPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
