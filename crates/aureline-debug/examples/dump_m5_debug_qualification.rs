//! Headless emitter for the M5 debug qualification set.
//!
//! Prints the canonical, frozen set of debugger qualification rows, claim publications, and
//! downgrade rules. Each qualification row binds the debugger object families it claims to the
//! proof packets that keep it current, computes one qualification status from evidence
//! freshness and completeness, and derives the maturity the product is allowed to publish —
//! stable only when certified with a supported, exact-mapping backend. Claim publications for
//! the claim board, About/help/service-health, support exports, and release packets republish
//! the narrowest maturity across the rows they cover, and downgrade rules name why each claim
//! narrowed. With `--lines`, prints the human-readable projection instead of JSON.
//!
//! Regenerate the checked-in fixture with:
//!
//! ```sh
//! cargo run -p aureline-debug --example dump_m5_debug_qualification \
//!   > fixtures/debug/m5_debug_qualification/canonical_set.json
//! cargo run -p aureline-debug --example dump_m5_debug_qualification -- --lines
//! ```

use aureline_debug::m5_debug_qualification::{
    m5_debug_qualification_lines, m5_debug_qualification_set,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = m5_debug_qualification_set();
    set.validate()
        .expect("canonical m5 debug qualification set validates");

    if want_lines {
        for line in m5_debug_qualification_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize m5 debug qualification set")
        );
    }
}
