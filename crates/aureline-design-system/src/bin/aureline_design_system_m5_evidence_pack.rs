//! Headless emitter for the M5 component-gallery evidence pack.
//!
//! The bin is the only mint-from-truth path for the checked-in evidence fixtures under
//! `fixtures/ui/m5-component-gallery/` (the pack file and one file per component) and the
//! release-packet proof at
//! `artifacts/release/m5-design-system-proof/evidence-pack-release.json`. Shell-quality gates,
//! docs/help, QA, and release review consume the evidence this bin mints, so visual / a11y proof,
//! freshness, and the derived claim gate read from one governed source.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_evidence_pack -- pack
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_evidence_pack -- component <component_kind>
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_evidence_pack -- release-packet
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_evidence_pack -- reevaluate <YYYY-MM-DD>
//! cargo run -q -p aureline-design-system --bin aureline_design_system_m5_evidence_pack -- validate
//! ```

use aureline_design_system::m5_component_manifest::M5ComponentKind;
use aureline_design_system::m5_evidence_pack::{seeded_m5_evidence_pack, M5EvidencePack};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("pack") | None => {
            let pack = seeded_m5_evidence_pack();
            assert_valid(&pack)?;
            println!("{}", pack.export_safe_json());
        }
        Some("component") => {
            let kind_token = args
                .get(1)
                .ok_or("component requires a component kind argument")?;
            let kind = parse_kind(kind_token)?;
            let pack = seeded_m5_evidence_pack();
            assert_valid(&pack)?;
            let component = pack
                .component(kind)
                .ok_or_else(|| format!("no component evidence for kind: {kind_token}"))?;
            println!(
                "{}",
                serde_json::to_string_pretty(component).expect("component serializes")
            );
        }
        Some("release-packet") => {
            let pack = seeded_m5_evidence_pack();
            assert_valid(&pack)?;
            println!("{}", pack.release_packet().export_safe_json());
        }
        Some("reevaluate") => {
            let evaluated_at = args
                .get(1)
                .ok_or("reevaluate requires a YYYY-MM-DD evaluation date argument")?;
            let pack = seeded_m5_evidence_pack().reevaluate(evaluated_at);
            assert_valid(&pack)?;
            println!("{}", pack.export_safe_json());
        }
        Some("validate") => {
            assert_valid(&seeded_m5_evidence_pack())?;
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

fn assert_valid(pack: &M5EvidencePack) -> Result<(), Box<dyn std::error::Error>> {
    let violations = pack.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("evidence pack failed validation: {}", tokens.join(",")).into())
    }
}
