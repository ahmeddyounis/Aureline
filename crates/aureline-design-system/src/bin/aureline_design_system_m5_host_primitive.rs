//! Headless emitter for the M5 host-rendered primitive library.
//!
//! The bin is the only mint-from-truth path for the checked-in primitive fixtures under
//! `fixtures/ui/m5-component-gallery/` (the library file and one file per primitive) and the
//! release-packet proof at
//! `artifacts/release/m5-design-system-proof/host-primitive-release.json`. Shell code, docs/help,
//! QA, and extension guidance consume the primitives this bin mints, so render plans, appearance,
//! and consumer conformance read from one governed source.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_host_primitive -- library
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_host_primitive -- primitive <component_kind>
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_host_primitive -- release-packet
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_host_primitive -- audit
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_host_primitive -- validate
//! ```

use aureline_design_system::m5_component_manifest::{
    seeded_m5_component_manifest_package, M5ComponentKind,
};
use aureline_design_system::m5_host_primitive::{
    audit_primitive_manifest_alignment, seeded_m5_host_primitive_library, M5HostPrimitiveLibrary,
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
        Some("library") | None => {
            let library = seeded_m5_host_primitive_library();
            assert_valid(&library)?;
            println!("{}", library.export_safe_json());
        }
        Some("primitive") => {
            let kind_token = args
                .get(1)
                .ok_or("primitive requires a component kind argument")?;
            let kind = parse_kind(kind_token)?;
            let library = seeded_m5_host_primitive_library();
            assert_valid(&library)?;
            let primitive = library
                .primitive(kind)
                .ok_or_else(|| format!("no primitive for kind: {kind_token}"))?;
            println!(
                "{}",
                serde_json::to_string_pretty(primitive).expect("primitive serializes")
            );
        }
        Some("release-packet") => {
            let library = seeded_m5_host_primitive_library();
            assert_valid(&library)?;
            println!("{}", library.release_packet().export_safe_json());
        }
        Some("audit") => {
            let library = seeded_m5_host_primitive_library();
            assert_valid(&library)?;
            let findings = audit_primitive_manifest_alignment(
                &library,
                &seeded_m5_component_manifest_package(),
            );
            if findings.is_empty() {
                println!("ok");
            } else {
                return Err(format!(
                    "host primitives are not aligned with their manifests: {}",
                    findings
                        .iter()
                        .map(|f| format!("{}:{}", f.primitive_id, f.code))
                        .collect::<Vec<_>>()
                        .join(",")
                )
                .into());
            }
        }
        Some("validate") => {
            assert_valid(&seeded_m5_host_primitive_library())?;
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

fn assert_valid(library: &M5HostPrimitiveLibrary) -> Result<(), Box<dyn std::error::Error>> {
    let violations = library.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!(
            "host primitive library failed validation: {}",
            tokens.join(",")
        )
        .into())
    }
}
