//! Headless emitter for the M5 auth-boundary lane: browser / device-code handoff
//! cards and webview origin bars.
//!
//! The bin is the only mint-from-truth path for the support exports checked in at
//! `artifacts/help/m5-auth-boundary-proof/browser_handoff_cards.json` and
//! `artifacts/help/m5-auth-boundary-proof/webview_origin_bars.json`, the
//! governance Markdown summary `artifacts/help/m5-auth-boundary-governance.md`,
//! the matrix CSVs `artifacts/help/m5-auth-boundary-browser-cards.csv` and
//! `artifacts/help/m5-auth-boundary-webview-bars.csv`, and the narrowed fixtures
//! under `fixtures/help/auth-boundary/`. Help/About, voice, and admin surfaces
//! read these sets so a browser/device-code handoff and any embedded webview are
//! origin-labeled and non-impersonating before a user grants trust or enters
//! credentials.
//!
//! Subcommands:
//!
//! ```sh
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_auth_boundaries -- browser-cards
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_auth_boundaries -- webview-bars
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_auth_boundaries -- governance
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_auth_boundaries -- csv-browser-cards
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_auth_boundaries -- csv-webview-bars
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_auth_boundaries -- fixture-device-code-card
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_auth_boundaries -- fixture-untrusted-webview
//! cargo run -q -p aureline-shell --bin aureline_shell_m5_auth_boundaries -- validate
//! ```

use aureline_shell::m5_auth_boundaries::{
    seeded_device_code_card_fixture, seeded_m5_browser_handoff_card_set,
    seeded_m5_webview_origin_bar_set, seeded_untrusted_webview_origin_bar_fixture,
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
        Some("browser-cards") | None => {
            let set = seeded_m5_browser_handoff_card_set();
            set.validate()?;
            println!("{}", set.export_safe_json());
        }
        Some("webview-bars") => {
            let set = seeded_m5_webview_origin_bar_set();
            set.validate()?;
            println!("{}", set.export_safe_json());
        }
        Some("governance") => {
            let cards = seeded_m5_browser_handoff_card_set();
            let bars = seeded_m5_webview_origin_bar_set();
            cards.validate()?;
            bars.validate()?;
            print!("{}", cards.render_markdown_summary());
            println!();
            print!("{}", bars.render_markdown_summary());
        }
        Some("csv-browser-cards") => {
            let set = seeded_m5_browser_handoff_card_set();
            set.validate()?;
            print!("{}", set.render_matrix_csv());
        }
        Some("csv-webview-bars") => {
            let set = seeded_m5_webview_origin_bar_set();
            set.validate()?;
            print!("{}", set.render_matrix_csv());
        }
        Some("fixture-device-code-card") => {
            let card = seeded_device_code_card_fixture();
            card.validate()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&card).expect("card serializes")
            );
        }
        Some("fixture-untrusted-webview") => {
            let bar = seeded_untrusted_webview_origin_bar_fixture();
            bar.validate()?;
            println!(
                "{}",
                serde_json::to_string_pretty(&bar).expect("bar serializes")
            );
        }
        Some("validate") => {
            seeded_m5_browser_handoff_card_set().validate()?;
            seeded_m5_webview_origin_bar_set().validate()?;
            seeded_device_code_card_fixture().validate()?;
            seeded_untrusted_webview_origin_bar_fixture().validate()?;
            println!("ok");
        }
        Some(other) => {
            return Err(format!("unknown subcommand: {other}").into());
        }
    }
    Ok(())
}
