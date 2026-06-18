//! Dump governed extension appearance-inheritance descriptor records.
//!
//! Mints the seeded audit, support export, and rendered artifacts from one
//! source of truth so the checked fixtures, support-export, and docs lanes stay
//! in lockstep:
//!
//! ```text
//! cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- audit
//! cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- inputs
//! cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- descriptors
//! cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- support-export
//! cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- compact
//! cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- markdown
//! cargo run -q -p aureline-extensions --example dump_extension_appearance_descriptor_records -- validate
//! ```

use aureline_extensions::appearance_descriptors::{
    seeded_extension_appearance_audit, seeded_extension_appearance_inputs,
    seeded_extension_appearance_support_export, validate_extension_appearance_audit,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let audit = seeded_extension_appearance_audit();

    match args.first().map(String::as_str) {
        Some("audit") | None => print_json(&audit)?,
        Some("inputs") => print_json(&seeded_extension_appearance_inputs())?,
        Some("descriptors") => print_json(&audit.descriptors)?,
        Some("support-export") => print_json(&seeded_extension_appearance_support_export())?,
        Some("compact") => {
            for line in audit.compact_lines() {
                println!("{line}");
            }
        }
        Some("markdown") => print!("{}", audit.render_markdown()),
        Some("validate") => match validate_extension_appearance_audit(&audit) {
            Ok(()) => println!("ok"),
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} descriptor={} field={} message={}",
                        defect.defect_kind.as_str(),
                        defect.descriptor_ref,
                        defect.field,
                        defect.message
                    );
                }
                std::process::exit(3);
            }
        },
        Some(other) => return Err(format!("unknown subcommand: {other}").into()),
    }
    Ok(())
}

fn print_json<T: serde::Serialize>(value: &T) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
