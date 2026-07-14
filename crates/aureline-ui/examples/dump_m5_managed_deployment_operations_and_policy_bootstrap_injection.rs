//! Headless emitter for the M5 managed-deployment operations and policy-bootstrap-injection registries packet.
//!
//! The example is the only mint-from-truth path for the support export checked in under
//! `artifacts/release/m5-managed-deployment-operations-and-policy-bootstrap-injection-proof/`, its matrix CSV,
//! the Markdown summary, and the narrowed fixtures under
//! `fixtures/install/m5-managed-deployment-operations-and-policy-bootstrap-injection/`.
//!
//! ```text
//! cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- support-export
//! cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- report
//! cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- csv
//! cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- receipt-table
//! cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- fixture-per-machine-managed-beta-narrowed
//! cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- fixture-offline-airgap-bundle-preview-narrowed
//! cargo run -p aureline-ui --example dump_m5_managed_deployment_operations_and_policy_bootstrap_injection -- validate
//! ```

use aureline_ui::m5_managed_deployment_operations_and_policy_bootstrap_injection::{
    seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection,
    seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_offline_airgap_bundle_preview_narrowed,
    seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_per_machine_managed_beta_narrowed,
    M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket,
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
            let packet = seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("report") => {
            print!(
                "{}",
                seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection()
                    .render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection()
                    .render_matrix_csv()
            );
        }
        Some("receipt-table") => {
            print!(
                "{}",
                seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection()
                    .render_managed_operation_receipt_table()
            );
        }
        Some("fixture-per-machine-managed-beta-narrowed") => {
            let packet =
                seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_per_machine_managed_beta_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("fixture-offline-airgap-bundle-preview-narrowed") => {
            let packet =
                seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_offline_airgap_bundle_preview_narrowed();
            assert_valid(&packet)?;
            println!("{}", packet.export_safe_json());
        }
        Some("validate") => {
            for packet in [
                seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection(),
                seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_per_machine_managed_beta_narrowed(),
                seeded_m5_managed_deployment_operations_and_policy_bootstrap_injection_offline_airgap_bundle_preview_narrowed(),
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
    packet: &M5ManagedDeploymentOperationsAndPolicyBootstrapInjectionPacket,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("registries packet failed validation: {}", tokens.join(",")).into())
    }
}
