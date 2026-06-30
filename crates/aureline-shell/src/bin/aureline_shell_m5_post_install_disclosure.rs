//! Headless emitter for the M5 post-install notice/provenance/SBOM disclosure
//! panels.
//!
//! The bin is the only mint-from-truth path for the panel-set support export
//! checked in under `artifacts/help/m5-post-install-proof/`, the per-family panel
//! exports, the governance Markdown summary, the panel CSV, and the narrowed
//! fixtures under `fixtures/help/post-install-disclosure/`. About/help,
//! installed-state inspectors, diagnostics exports, and — for packs — marketplace
//! detail read these panels so a user can inspect how each installed or generated
//! artifact arrived, what it is made of, and which provenance/notice/SBOM data is
//! missing, without leaving Aureline.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_post_install_disclosure -- panel-set
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_post_install_disclosure -- panel desktop_build_installer
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_post_install_disclosure -- panel extension_framework_pack
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_post_install_disclosure -- panel mirrored_offline_artifact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_post_install_disclosure -- panel generated_export_artifact
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_post_install_disclosure -- governance
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_post_install_disclosure -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_post_install_disclosure -- fixture-signature-revoked
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_post_install_disclosure -- fixture-generated-sbom-not-provided
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_post_install_disclosure -- validate
//! ```

use aureline_shell::m5_post_install_disclosure::{
    seeded_m5_post_install_disclosure_panel_set,
    seeded_post_install_generated_export_sbom_not_provided,
    seeded_post_install_product_build_signature_revoked, DisclosureArtifactFamily,
    M5PostInstallDisclosurePanelSet, PostInstallDisclosureRecord,
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
        Some("panel-set") | None => {
            let set = seeded_m5_post_install_disclosure_panel_set();
            assert_set_valid(&set)?;
            println!("{}", set.export_safe_json());
        }
        Some("panel") => {
            let family = args.get(1).map(String::as_str).ok_or(
                "panel subcommand needs a family token (desktop_build_installer, extension_framework_pack, mirrored_offline_artifact, generated_export_artifact)",
            )?;
            let set = seeded_m5_post_install_disclosure_panel_set();
            let wanted = parse_family(family)?;
            let panel = set
                .panels
                .iter()
                .find(|panel| panel.artifact_family() == wanted)
                .ok_or_else(|| format!("no panel for family {family}"))?;
            assert_record_valid(panel)?;
            println!("{}", serde_json::to_string_pretty(panel)?);
        }
        Some("governance") => {
            print!(
                "{}",
                seeded_m5_post_install_disclosure_panel_set().render_markdown_summary()
            );
        }
        Some("csv") => {
            print!(
                "{}",
                seeded_m5_post_install_disclosure_panel_set().render_panel_csv()
            );
        }
        Some("fixture-signature-revoked") => {
            let panel = seeded_post_install_product_build_signature_revoked();
            assert_record_valid(&panel)?;
            println!("{}", serde_json::to_string_pretty(&panel)?);
        }
        Some("fixture-generated-sbom-not-provided") => {
            let panel = seeded_post_install_generated_export_sbom_not_provided();
            assert_record_valid(&panel)?;
            println!("{}", serde_json::to_string_pretty(&panel)?);
        }
        Some("validate") => {
            assert_set_valid(&seeded_m5_post_install_disclosure_panel_set())?;
            assert_record_valid(&seeded_post_install_product_build_signature_revoked())?;
            assert_record_valid(&seeded_post_install_generated_export_sbom_not_provided())?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}

fn parse_family(token: &str) -> Result<DisclosureArtifactFamily, Box<dyn std::error::Error>> {
    DisclosureArtifactFamily::ALL
        .into_iter()
        .find(|family| family.as_str() == token)
        .ok_or_else(|| format!("unknown family: {token}").into())
}

fn assert_set_valid(
    set: &M5PostInstallDisclosurePanelSet,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = set.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("panel set failed validation: {}", tokens.join(",")).into())
    }
}

fn assert_record_valid(
    record: &PostInstallDisclosureRecord,
) -> Result<(), Box<dyn std::error::Error>> {
    let violations = record.validate();
    if violations.is_empty() {
        Ok(())
    } else {
        let tokens: Vec<&str> = violations.iter().map(|v| v.as_str()).collect();
        Err(format!("disclosure record failed validation: {}", tokens.join(",")).into())
    }
}
