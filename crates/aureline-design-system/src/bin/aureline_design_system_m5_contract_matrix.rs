//! Headless emitter for the M5 design-system contract matrix.
//!
//! The bin is the only mint-from-truth path for the matrix support export and Markdown proof
//! checked in under `artifacts/release/m5-design-system-proof/`, the published dashboard at
//! `artifacts/design-system/m5-design-system-dashboard.json`, the component-gallery demo
//! fixtures under `fixtures/ui/m5-component-gallery/`, and the missing-object / stale-proof /
//! waiver drill fixtures under `fixtures/ui/m5-design-system-contract-matrix/`. Shell, help,
//! onboarding, presentation, the extension SDK, release center, QA, and the stable-claim
//! matrix consume this matrix so each claimed surface either maps a current contract object
//! or is auto-narrowed / blocked before Stable promotion, with the gap named rather than left
//! invisible.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- support-export
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- dashboard
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- markdown
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- gallery-foundations
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- gallery-reference-layout
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- gallery-component <surface>
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- fixture-missing-object
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- fixture-stale-proof-retest-pending
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- fixture-waived-narrowed
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_contract_matrix -- validate
//! ```

use aureline_design_system::m5_design_system_contract::{
    seeded_m5_component_contract_gallery, seeded_m5_design_system_contract_matrix,
    seeded_m5_design_system_contract_matrix_missing_object,
    seeded_m5_design_system_contract_matrix_stale_proof_retest_pending,
    seeded_m5_design_system_contract_matrix_waived_narrowed, seeded_m5_foundations_artifact,
    seeded_m5_reference_layout_artifact, M5DesignSystemContractMatrix,
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
            let matrix = seeded_m5_design_system_contract_matrix();
            assert_valid(&matrix)?;
            println!("{}", matrix.export_safe_json());
        }
        Some("dashboard") => {
            let matrix = seeded_m5_design_system_contract_matrix();
            assert_valid(&matrix)?;
            println!("{}", matrix.dashboard_json());
        }
        Some("markdown") => {
            print!(
                "{}",
                seeded_m5_design_system_contract_matrix().render_markdown_summary()
            );
        }
        Some("gallery-foundations") => {
            print_json(&seeded_m5_foundations_artifact())?;
        }
        Some("gallery-reference-layout") => {
            print_json(&seeded_m5_reference_layout_artifact())?;
        }
        Some("gallery-component") => {
            let surface = args
                .get(1)
                .ok_or("gallery-component requires a <surface> argument")?;
            let contract = seeded_m5_component_contract_gallery()
                .into_iter()
                .find(|c| c.surface_class.as_str() == surface)
                .ok_or_else(|| format!("unknown component surface: {surface}"))?;
            print_json(&contract)?;
        }
        Some("fixture-missing-object") => {
            let matrix = seeded_m5_design_system_contract_matrix_missing_object();
            assert_valid(&matrix)?;
            println!("{}", matrix.export_safe_json());
        }
        Some("fixture-stale-proof-retest-pending") => {
            let matrix = seeded_m5_design_system_contract_matrix_stale_proof_retest_pending();
            assert_valid(&matrix)?;
            println!("{}", matrix.export_safe_json());
        }
        Some("fixture-waived-narrowed") => {
            let matrix = seeded_m5_design_system_contract_matrix_waived_narrowed();
            assert_valid(&matrix)?;
            println!("{}", matrix.export_safe_json());
        }
        Some("validate") => {
            for matrix in [
                seeded_m5_design_system_contract_matrix(),
                seeded_m5_design_system_contract_matrix_missing_object(),
                seeded_m5_design_system_contract_matrix_stale_proof_retest_pending(),
                seeded_m5_design_system_contract_matrix_waived_narrowed(),
            ] {
                assert_valid(&matrix)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(matrix: &M5DesignSystemContractMatrix) -> Result<(), Box<dyn std::error::Error>> {
    let violations = matrix.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("matrix failed validation: {}", tokens.join(",")).into())
    }
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
