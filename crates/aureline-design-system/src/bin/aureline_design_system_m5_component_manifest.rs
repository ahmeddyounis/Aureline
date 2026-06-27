//! Headless emitter for the M5 design-system component-manifest package.
//!
//! The bin is the only mint-from-truth path for the checked-in manifest fixtures under
//! `fixtures/ui/m5-component-gallery/` (the package file and one file per manifest) and the
//! release-packet proof at
//! `artifacts/release/m5-design-system-proof/component-manifest-release.json`. Shell code,
//! docs/help, QA, and extension guidance consume the manifests this bin mints, so anatomy, states,
//! keyboard, accessibility, and token dependencies read from one governed source.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_component_manifest -- package
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_component_manifest -- manifest <component_kind>
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_component_manifest -- release-packet
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_component_manifest -- validate
//! ```

use aureline_design_system::m5_component_manifest::{
    seeded_m5_component_manifest_package, M5ComponentKind, M5ComponentManifestPackage,
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
            let package = seeded_m5_component_manifest_package();
            assert_valid(&package)?;
            println!("{}", package.export_safe_json());
        }
        Some("manifest") => {
            let kind_token = args
                .get(1)
                .ok_or("manifest requires a component kind argument")?;
            let kind = parse_kind(kind_token)?;
            let package = seeded_m5_component_manifest_package();
            assert_valid(&package)?;
            let manifest = package
                .manifest(kind)
                .ok_or_else(|| format!("no manifest for kind: {kind_token}"))?;
            println!(
                "{}",
                serde_json::to_string_pretty(manifest).expect("manifest serializes")
            );
        }
        Some("release-packet") => {
            let package = seeded_m5_component_manifest_package();
            assert_valid(&package)?;
            println!("{}", package.release_packet().export_safe_json());
        }
        Some("validate") => {
            assert_valid(&seeded_m5_component_manifest_package())?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_kind(token: &str) -> Result<M5ComponentKind, Box<dyn std::error::Error>> {
    M5ComponentKind::ALL
        .into_iter()
        .find(|k| k.as_str() == token)
        .ok_or_else(|| format!("unknown component kind: {token}").into())
}

fn assert_valid(package: &M5ComponentManifestPackage) -> Result<(), Box<dyn std::error::Error>> {
    let violations = package.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "component manifest package failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
