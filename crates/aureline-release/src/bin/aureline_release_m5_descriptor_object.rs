//! Headless emitter for the M5 public-truth descriptor objects.
//!
//! The bin is the only mint-from-truth path for the descriptor-object registry checked in at
//! `artifacts/public-truth/descriptors/m5-descriptor-object-registry.json`, the release-grade
//! parity proof under `artifacts/release/m5-descriptor-parity-proof/descriptor-objects.json`,
//! and the descriptor-object instance fixtures under
//! `fixtures/public-truth/m5-badge-consumers/`. Each object freezes the typed provenance,
//! freshness, qualification, and client-scope state a claimed M5 artifact carries — over the
//! frozen controlled enums — and derives its effective qualification from named narrowings, so
//! weaker-but-present evidence auto-narrows to beta and absent provenance or evidence blocks
//! stable, with every weaker value surviving as explicit state rather than omission.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_object -- registry
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_object -- markdown
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_object -- object stable
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_object -- object narrowed
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_object -- object not-provided
//! cargo run -q -p aureline-release --bin aureline_release_m5_descriptor_object -- validate
//! ```

use aureline_release::m5_descriptor_object::{
    seeded_m5_descriptor_object_registry, seeded_narrowed_descriptor_object,
    seeded_not_provided_descriptor_object, seeded_stable_descriptor_object, DescriptorObject,
    M5DescriptorObjectRegistry,
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
        Some("registry") | None => {
            let registry = seeded_m5_descriptor_object_registry();
            assert_registry_valid(&registry)?;
            println!("{}", registry.export_safe_json());
        }
        Some("markdown") => {
            let registry = seeded_m5_descriptor_object_registry();
            assert_registry_valid(&registry)?;
            print!("{}", registry.render_markdown_summary());
        }
        Some("object") => {
            let object = parse_object(args.get(1).map(String::as_str).unwrap_or(""))?;
            assert_object_valid(&object)?;
            println!("{}", object.export_safe_json());
        }
        Some("validate") => {
            let registry = seeded_m5_descriptor_object_registry();
            assert_registry_valid(&registry)?;
            println!(
                "ok: descriptor-object registry valid ({} objects)",
                registry.objects.len()
            );
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_object(token: &str) -> Result<DescriptorObject, Box<dyn std::error::Error>> {
    match token {
        "stable" => Ok(seeded_stable_descriptor_object()),
        "narrowed" => Ok(seeded_narrowed_descriptor_object()),
        "not-provided" | "not_provided" | "blocked" => Ok(seeded_not_provided_descriptor_object()),
        other => Err(format!("unknown object selector: {other}").into()),
    }
}

fn assert_registry_valid(
    registry: &M5DescriptorObjectRegistry,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = registry.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("registry failed validation: {}", tokens.join(",")).into())
}

fn assert_object_valid(object: &DescriptorObject) -> Result<(), Box<dyn std::error::Error>> {
    let violations = object.validate();
    if violations.is_empty() {
        return Ok(());
    }
    let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
    Err(format!("object failed validation: {}", tokens.join(",")).into())
}
