//! Headless emitter for the M5 embedded service-dashboard / auth-handoff set.
//!
//! Prints the canonical embedded-surface set — the origin bars, device-permission
//! rows, and browser / device-code auth handoff cards with owner/origin and
//! capability truth, device processing/retention/revoke disclosure, and explicit
//! handoff reason/target/code/expiry/return paths — as one support-export-safe
//! record bound to the operator-surface matrix. Shell UI, CLI/headless inspect,
//! companion, incident/support/admin/managed consumers, and support export render
//! this set instead of letting an embedded webview impersonate native chrome or
//! hiding a browser/device-code boundary behind a generic Continue. With `--lines`,
//! prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-support --example dump_m5_embedded_dashboards            # JSON
//! cargo run -p aureline-support --example dump_m5_embedded_dashboards -- --lines
//! ```

use aureline_support::m5_embedded_dashboards::{embedded_surface_lines, embedded_surface_set};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = embedded_surface_set();
    set.validate()
        .expect("canonical embedded-surface set validates");

    if want_lines {
        for line in embedded_surface_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize embedded-surface set")
        );
    }
}
