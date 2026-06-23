//! Headless emitter for the M5 quiet-hours-suppression bundle.
//!
//! Prints the canonical bundle that applies one coherent suppression policy —
//! quiet-hours, do-not-disturb, presentation/follow, lock-screen privacy, admin
//! suppression, and managed-endpoint posture — across the in-app activity center, OS
//! notification, and companion surfaces, and explains for every surface whether the
//! event was shown, downgraded, or withheld. The shell activity center, OS
//! notifications, companion summaries, operator dashboard, support export, Help/About,
//! and CLI/headless consumers render this bundle instead of reimplementing quiet-hours
//! and suppression per surface. With `--lines`, prints the human-readable projection
//! instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-activity --example dump_m5_quiet_hours_suppression            # JSON
//! cargo run -p aureline-activity --example dump_m5_quiet_hours_suppression -- --lines
//! ```

use aureline_activity::m5_quiet_hours_suppression::{
    quiet_hours_suppression_bundle, quiet_hours_suppression_lines,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = quiet_hours_suppression_bundle();
    bundle
        .validate()
        .expect("canonical quiet-hours-suppression bundle validates");

    if want_lines {
        for line in quiet_hours_suppression_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle)
                .expect("serialize quiet-hours-suppression bundle")
        );
    }
}
