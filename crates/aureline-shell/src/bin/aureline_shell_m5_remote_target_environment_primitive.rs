//! Headless emitter for the M5 remote-target-pill / environment-status-strip
//! primitive.
//!
//! The bin is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-remote-target-environment-proof/`, its matrix CSV, the
//! Markdown report `artifacts/components/m5-remote-target-environment-primitive.md`,
//! and the narrowed fixtures under
//! `fixtures/ui/m5-remote-target-environment-primitive/`. Every M5 run-capable
//! surface (the run console, the test runner, the debug session, the notebook
//! runtime, the request runner, the database session, the preview server, the
//! pipeline run, and the incident surface) reads this primitive so target identity,
//! host boundary, degraded / reconnect state, resolved runtime source, scope,
//! readiness, and the "Why this context?" entrypoint stay consistent, and so the
//! support export reconstructs target and runtime resolution from one shared model.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_remote_target_environment_primitive -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_remote_target_environment_primitive -- report
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_remote_target_environment_primitive -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_remote_target_environment_primitive -- fixture-incident-surface-beta-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_remote_target_environment_primitive -- fixture-pipeline-run-preview-narrowed
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_remote_target_environment_primitive -- validate
//! ```

use aureline_shell::implement_the_m5_remote_target_pill_and_environment_status_strip_runtime_source_readiness_and_context_entrypoint_primitive::{
    seeded_m5_remote_target_environment_primitive_incident_surface_beta_narrowed,
    seeded_m5_remote_target_environment_primitive_packet,
    seeded_m5_remote_target_environment_primitive_pipeline_run_preview_narrowed,
    M5RemoteTargetEnvironmentPrimitivePacket,
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
            let packet = seeded_m5_remote_target_environment_primitive_packet();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_remote_target_environment_primitive_packet().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_remote_target_environment_primitive_packet().render_matrix_csv()
            );
        }
        Some("fixture-incident-surface-beta-narrowed") => {
            let packet =
                seeded_m5_remote_target_environment_primitive_incident_surface_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-pipeline-run-preview-narrowed") => {
            let packet =
                seeded_m5_remote_target_environment_primitive_pipeline_run_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_remote_target_environment_primitive_packet(),
                seeded_m5_remote_target_environment_primitive_incident_surface_beta_narrowed(),
                seeded_m5_remote_target_environment_primitive_pipeline_run_preview_narrowed(),
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
    packet: &M5RemoteTargetEnvironmentPrimitivePacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("packet failed validation: {}", tokens.join(",")).into())
    }
}
