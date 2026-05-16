//! Dump extension webview boundary audit records.
//!
//! Used by the checked fixture, support-export, and docs validation lanes:
//!
//! ```text
//! cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- packet
//! cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- inputs
//! cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- rows
//! cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- support-rows
//! cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- defects
//! cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- support-export
//! cargo run -q -p aureline-extensions --example dump_webview_boundary_audit_records -- validate
//! ```

use aureline_extensions::{
    project_extension_webview_boundary_support_export,
    seeded_extension_webview_boundary_audit_packet, seeded_extension_webview_boundary_inputs,
    validate_extension_webview_boundary_packet,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let packet = seeded_extension_webview_boundary_audit_packet();

    match args.first().map(String::as_str) {
        Some("packet") | None => print_json(&packet)?,
        Some("inputs") => print_json(&seeded_extension_webview_boundary_inputs())?,
        Some("rows") => print_json(&packet.rows)?,
        Some("support-rows") => print_json(&packet.support_rows)?,
        Some("defects") => print_json(&packet.defects)?,
        Some("support-export") => {
            let export = project_extension_webview_boundary_support_export(
                &packet,
                "extension-webview-boundary:support-export:default",
                "2026-05-16T00:00:00Z",
            );
            print_json(&export)?;
        }
        Some("validate") => match validate_extension_webview_boundary_packet(&packet) {
            Ok(()) => println!("ok"),
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} row={} field={} message={}",
                        defect.defect_kind.as_str(),
                        defect.row_ref,
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
