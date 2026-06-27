//! Headless emitter for the M5 design-system reference-layout package.
//!
//! The bin is the only mint-from-truth path for the checked-in reference-layout fixtures under
//! `fixtures/ui/m5-reference-layout/` (the package file and one file per workspace), the
//! release-packet proof at
//! `artifacts/release/m5-design-system-proof/reference-layout-release.json`, and the shell-slot
//! conformance packet at
//! `artifacts/release/m5-design-system-proof/reference-layout-conformance.json`. Shell code,
//! docs/help, QA, and extension guidance consume the layouts this bin mints, so zone occupancy,
//! responsive collapse, missing-dependency placeholders, and reopen/reset routes read from one
//! governed source.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_reference_layout -- package
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_reference_layout -- workspace <workspace_kind>
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_reference_layout -- release-packet
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_reference_layout -- conformance
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_reference_layout -- validate
//! ```

use aureline_design_system::m5_reference_layout::{
    seeded_m5_reference_layout_package, M5ReferenceLayoutPackage, M5WorkspaceKind,
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
        Some("package") | None => {
            let package = seeded_m5_reference_layout_package();
            assert_valid(&package)?;
            println!("{}", package.export_safe_json());
        }
        Some("workspace") => {
            let kind_token = args
                .get(1)
                .ok_or("workspace requires a workspace kind argument")?;
            let kind = parse_kind(kind_token)?;
            let package = seeded_m5_reference_layout_package();
            assert_valid(&package)?;
            let layout = package
                .layout(kind)
                .ok_or_else(|| format!("no layout for kind: {kind_token}"))?;
            println!(
                "{}",
                serde_json::to_string_pretty(layout).expect("layout serializes")
            );
        }
        Some("release-packet") => {
            let package = seeded_m5_reference_layout_package();
            assert_valid(&package)?;
            println!("{}", package.release_packet().export_safe_json());
        }
        Some("conformance") => {
            let package = seeded_m5_reference_layout_package();
            assert_valid(&package)?;
            println!("{}", package.conformance_packet().export_safe_json());
        }
        Some("validate") => {
            assert_valid(&seeded_m5_reference_layout_package())?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_kind(token: &str) -> Result<M5WorkspaceKind, Box<dyn std::error::Error>> {
    M5WorkspaceKind::ALL
        .into_iter()
        .find(|k| k.as_str() == token)
        .ok_or_else(|| format!("unknown workspace kind: {token}").into())
}

fn assert_valid(package: &M5ReferenceLayoutPackage) -> Result<(), Box<dyn std::error::Error>> {
    let violations = package.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "reference layout package failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
