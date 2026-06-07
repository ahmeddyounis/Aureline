//! Headless emitter for stable shell-slot zoning and responsive fallback truth.

use std::path::PathBuf;

use aureline_shell::stabilize_shell_zoning_and_responsive_fallback::{
    canonical_shell_zoning_packet, support_export_lines,
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
        None | Some("all") => {
            println!("{}", render_json());
            Ok(())
        }
        Some("index") => {
            print_index();
            Ok(())
        }
        Some("plaintext") => {
            for line in support_export_lines() {
                println!("{line}");
            }
            Ok(())
        }
        Some("emit-fixtures") => {
            let dir = args
                .get(1)
                .ok_or("emit-fixtures <dir> requires a target directory")?;
            emit_fixtures(PathBuf::from(dir))
        }
        Some(other) => Err(format!("unknown subcommand: {other}").into()),
    }
}

fn render_json() -> String {
    serde_json::to_string_pretty(&canonical_shell_zoning_packet()).expect("packet serializes")
}

fn print_index() {
    let packet = canonical_shell_zoning_packet();
    println!(
        "slots={} ladders={} surfaces={} placeholders={} audit_pass={}",
        packet.declared_slots.len(),
        packet.responsive_ladders.len(),
        packet.stable_surface_claims.len(),
        packet.placeholder_hydration_cases.len(),
        packet.audit.passes()
    );
    for slot in &packet.declared_slots {
        println!(
            "{} zone={} placeholder={}",
            slot.slot_id.as_str(),
            slot.shell_zone.as_str(),
            slot.placeholder_class.as_str()
        );
    }
}

fn emit_fixtures(dir: PathBuf) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(&dir)?;
    let path = dir.join("shell_zoning_responsive_fallback_packet.json");
    std::fs::write(&path, format!("{}\n", render_json()))?;
    println!("wrote {}", path.display());
    Ok(())
}
