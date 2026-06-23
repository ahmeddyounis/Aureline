//! Headless emitter for the M5 activity-objects bundle.
//!
//! Prints the canonical bundle that implements Aureline's durable activity object
//! model: the job-family registry every claimed M5 long-running, retryable, or
//! reviewable family maps to, the activity-object corpus, and the activity-center
//! rows rendered from them across the shell, support export, companion, and
//! operator surfaces. The shell activity center, support export, companion
//! summaries, operator dashboard, Help/About, and CLI/headless consumers render
//! this bundle instead of reimplementing the durable job model per surface. With
//! `--lines`, prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-activity --example dump_m5_activity_objects            # JSON
//! cargo run -p aureline-activity --example dump_m5_activity_objects -- --lines
//! ```

use aureline_activity::m5_activity_objects::{activity_objects_bundle, activity_objects_lines};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = activity_objects_bundle();
    bundle
        .validate()
        .expect("canonical activity-objects bundle validates");

    if want_lines {
        for line in activity_objects_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).expect("serialize activity-objects bundle")
        );
    }
}
