//! Headless emitter for the M5 attention-qualification bundle.
//!
//! Prints the canonical certification that binds every claimed attention family —
//! notification envelopes, durable activity objects, action/retention semantics,
//! quiet-hours and suppression, badge aggregates, and fanout receipts on the
//! attention-routing matrix spine — to the shell, companion, and operator profiles
//! that advertise it, with each profile's claim derived from its dependencies'
//! evidence. Release evidence, About/Help, the activity center, support export, the
//! compatibility report, and public-truth surfaces render this bundle instead of
//! restating attention quality claims. With `--lines`, prints the human-readable
//! projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-activity --example dump_m5_attention_qualification            # JSON
//! cargo run -p aureline-activity --example dump_m5_attention_qualification -- --lines
//! ```

use aureline_activity::m5_attention_qualification::{
    attention_qualification_bundle, attention_qualification_lines,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = attention_qualification_bundle();
    bundle
        .validate()
        .expect("canonical attention-qualification bundle validates");

    if want_lines {
        for line in attention_qualification_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle)
                .expect("serialize attention-qualification bundle")
        );
    }
}
