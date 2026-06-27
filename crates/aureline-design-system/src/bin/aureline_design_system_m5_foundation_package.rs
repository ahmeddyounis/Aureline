//! Headless emitter for the M5 design-system foundation package.
//!
//! The bin is the only mint-from-truth path for the checked-in foundation-package fixtures under
//! `fixtures/ui/m5-foundation-package/` and the release-packet proof at
//! `artifacts/release/m5-design-system-proof/foundation-package-release.json`. Shell code,
//! docs/help, screenshots, and extension guidance consume the package this bin mints, so the
//! density, reduced-motion, power-saving, and high-contrast rows read from one governed source.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_foundation_package -- package
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_foundation_package -- package-next
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_foundation_package -- diff
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_foundation_package -- release-packet
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_foundation_package -- validate
//! ```

use aureline_design_system::m5_foundation_package::{
    seeded_m5_foundation_package, seeded_m5_foundation_package_next, M5FoundationPackage,
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
            let package = seeded_m5_foundation_package();
            assert_valid(&package)?;
            println!("{}", package.export_safe_json());
        }
        Some("package-next") => {
            let package = seeded_m5_foundation_package_next();
            assert_valid(&package)?;
            println!("{}", package.export_safe_json());
        }
        Some("diff") => {
            let diff = seeded_m5_foundation_package().diff(&seeded_m5_foundation_package_next());
            println!("{}", diff.export_safe_json());
        }
        Some("release-packet") => {
            let package = seeded_m5_foundation_package();
            assert_valid(&package)?;
            println!("{}", package.release_packet().export_safe_json());
        }
        Some("validate") => {
            for package in [
                seeded_m5_foundation_package(),
                seeded_m5_foundation_package_next(),
            ] {
                assert_valid(&package)?;
            }
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn assert_valid(package: &M5FoundationPackage) -> Result<(), Box<dyn std::error::Error>> {
    let violations = package.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("foundation package failed validation: {}", tokens.join(",")).into())
    }
}
