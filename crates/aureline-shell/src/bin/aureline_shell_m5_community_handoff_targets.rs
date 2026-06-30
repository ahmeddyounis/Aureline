//! Headless emitter for the M5 community-handoff target review sheet set.
//!
//! The bin is the only mint-from-truth path for the support export checked in at
//! `artifacts/help/m5-community-handoff-proof/target_set.json`, the governance
//! Markdown summary `artifacts/help/m5-community-handoff-governance.md`, the
//! matrix CSV `artifacts/help/m5-community-handoff-targets.csv`, and the narrowed
//! fixtures under `fixtures/help/community-handoff/`. Help/About, support,
//! ecosystem, and reporting surfaces read this set so an outbound issue,
//! security, docs, discussion, community, or official-support route is typed,
//! labeled, and reviewable before a browser opens.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_community_handoff_targets -- support-export
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_community_handoff_targets -- governance
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_community_handoff_targets -- csv
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_community_handoff_targets -- fixture-security-local-safe-fallback
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_community_handoff_targets -- fixture-community-not-a-commitment
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_community_handoff_targets -- validate
//! ```

use aureline_shell::m5_community_handoff_targets::{
    seeded_community_support_sheet_no_commitment, seeded_m5_community_handoff_target_sheet_set,
    seeded_security_disclosure_sheet_unsupported_profile,
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
        Some("support-export") | None => {
            let set = seeded_m5_community_handoff_target_sheet_set();
            set.validate()?;
            println!("{}", set.export_safe_json());
        }
        Some("governance") => {
            let set = seeded_m5_community_handoff_target_sheet_set();
            set.validate()?;
            print!("{}", set.render_markdown_summary());
        }
        Some("csv") => {
            let set = seeded_m5_community_handoff_target_sheet_set();
            set.validate()?;
            print!("{}", set.render_matrix_csv());
        }
        Some("fixture-security-local-safe-fallback") => {
            let sheet = seeded_security_disclosure_sheet_unsupported_profile();
            sheet.validate()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&sheet).expect("sheet serializes")
            );
        }
        Some("fixture-community-not-a-commitment") => {
            let sheet = seeded_community_support_sheet_no_commitment();
            sheet.validate()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&sheet).expect("sheet serializes")
            );
        }
        Some("validate") => {
            seeded_m5_community_handoff_target_sheet_set().validate()?;
            seeded_security_disclosure_sheet_unsupported_profile().validate()?;
            seeded_community_support_sheet_no_commitment().validate()?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}
