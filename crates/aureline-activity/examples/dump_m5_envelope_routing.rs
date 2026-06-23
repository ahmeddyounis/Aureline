//! Headless emitter for the M5 envelope-routing bundle.
//!
//! Prints the canonical bundle that implements Aureline's typed notification
//! envelope path: the producer registry every M5 subsystem emits through, the
//! envelope corpus, the representative routing contexts, and every routing
//! decision. The shell activity center, OS notifications, dock/taskbar badge,
//! browser and mobile companions, operator dashboard, support export, Help/About,
//! and CLI/headless consumers render this bundle instead of reimplementing
//! notification routing per surface. With `--lines`, prints the human-readable
//! projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-activity --example dump_m5_envelope_routing            # JSON
//! cargo run -p aureline-activity --example dump_m5_envelope_routing -- --lines
//! ```

use aureline_activity::m5_envelope_routing::{envelope_routing_bundle, envelope_routing_lines};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = envelope_routing_bundle();
    bundle
        .validate()
        .expect("canonical envelope-routing bundle validates");

    if want_lines {
        for line in envelope_routing_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).expect("serialize envelope-routing bundle")
        );
    }
}
