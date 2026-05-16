//! Headless inspector for the beta policy-pack page.
//!
//! The bin emits the same audited records consumed by the admin/settings
//! center, support-export wrapper, shell summary, diagnostics views, and
//! fixture replay.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_policy_pack_beta -- page
//! cargo run -q -p aureline-shell --bin aureline_shell_policy_pack_beta -- packs
//! cargo run -q -p aureline-shell --bin aureline_shell_policy_pack_beta -- diffs
//! cargo run -q -p aureline-shell --bin aureline_shell_policy_pack_beta -- denial-traces
//! cargo run -q -p aureline-shell --bin aureline_shell_policy_pack_beta -- import-receipts
//! cargo run -q -p aureline-shell --bin aureline_shell_policy_pack_beta -- defects
//! cargo run -q -p aureline-shell --bin aureline_shell_policy_pack_beta -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_policy_pack_beta -- summary
//! cargo run -q -p aureline-shell --bin aureline_shell_policy_pack_beta -- validate
//! ```

use aureline_shell::policy_pack_beta::{
    audit_policy_pack_beta_page, seeded_policy_pack_beta_page, validate_policy_pack_beta_page,
    PolicyPackBetaPage, PolicyPackBetaRenderSummary, PolicyPackBetaSummary,
    PolicyPackBetaSupportExport, PolicyPackSourceClass,
};

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let page = seeded_policy_pack_beta_page();

    match args.first().map(String::as_str) {
        Some("page") | None => print_json(&page)?,
        Some("packs") => print_json(&page.packs)?,
        Some("diffs") => print_json(&page.diffs)?,
        Some("denial-traces") => print_json(&page.denial_traces)?,
        Some("import-receipts") => print_json(&page.import_receipts)?,
        Some("defects") => print_json(&page.defects)?,
        Some("support-export") => {
            let export = PolicyPackBetaSupportExport::from_page(
                "support-export:policy-pack-beta:001",
                "2026-05-16T00:00:00Z",
                page,
            );
            print_json(&export)?;
        }
        Some("summary") => {
            let summary = PolicyPackBetaRenderSummary::from_page(&page);
            print_json(&summary)?;
        }
        Some("drill-signature-dropped") => {
            let mut page = page;
            let receipt = page
                .import_receipts
                .iter_mut()
                .find(|receipt| receipt.source_class == PolicyPackSourceClass::SignedMirrorOrigin)
                .ok_or("seeded page must include a mirror import receipt")?;
            receipt.preserves_signature_blob = false;
            receipt.provenance.signature_blob_ref = "dropped".to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-denial-unresolvable") => {
            let mut page = page;
            page.denial_traces[0].rule_id = "rule:does-not-exist".to_owned();
            print_json(&rebuild_with_defects(page))?;
        }
        Some("drill-public-fallback") => {
            let mut page = page;
            page.packs[1].no_public_endpoint_fallback = false;
            print_json(&rebuild_with_defects(page))?;
        }
        Some("validate") => match validate_policy_pack_beta_page(&page) {
            Ok(()) => println!("ok"),
            Err(defects) => {
                for defect in defects {
                    eprintln!(
                        "defect: kind={} subject_id={} field={} note={}",
                        defect.defect_kind_token, defect.subject_id, defect.field, defect.note,
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
    let json = serde_json::to_string_pretty(value)?;
    println!("{json}");
    Ok(())
}

fn rebuild_with_defects(mut page: PolicyPackBetaPage) -> PolicyPackBetaPage {
    page.defects = audit_policy_pack_beta_page(
        &page.packs,
        &page.diffs,
        &page.denial_traces,
        &page.import_receipts,
    );
    page.summary = PolicyPackBetaSummary::from_records(
        &page.packs,
        &page.diffs,
        &page.denial_traces,
        &page.import_receipts,
        &page.defects,
    );
    page
}
