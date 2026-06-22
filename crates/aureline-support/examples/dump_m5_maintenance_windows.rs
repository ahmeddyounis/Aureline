//! Headless emitter for the M5 maintenance / failover / reconciliation windows.
//!
//! Prints the canonical window set — the scheduled / read-only / drain / migration
//! / failover / reconciling / resolved windows with exact times and time zones,
//! named blocked write classes, local-safe / publish-later continuity, changed-
//! boundary disclosure, and computed review-before-replay — as one support-export-
//! safe record bound to the operator-surface matrix. Shell UI, CLI/headless
//! inspect, service-health, companion, incident/support/managed consumers, and
//! support export render this set instead of restating maintenance, failover, or
//! reconciliation truth as a generic outage banner. With `--lines`, prints the
//! human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-support --example dump_m5_maintenance_windows            # JSON
//! cargo run -p aureline-support --example dump_m5_maintenance_windows -- --lines
//! ```

use aureline_support::m5_maintenance_windows::{maintenance_window_lines, maintenance_window_set};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = maintenance_window_set();
    set.validate()
        .expect("canonical maintenance-window set validates");

    if want_lines {
        for line in maintenance_window_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize maintenance-window set")
        );
    }
}
