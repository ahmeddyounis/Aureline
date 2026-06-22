//! Headless emitter for the M5 operator-board set.
//!
//! Prints the canonical board set — the incident-response, support-queue,
//! admin-approvals, and release-readiness overview boards — as summaries over
//! many canonical objects, with the shared filter/saved-view vocabulary, the
//! computed no-silent-green tile state, first-class owner and blocker/waiver
//! state, canonical open-detail routing, and the frozen default-view exports.
//! Shell UI, CLI/headless inspect, incident/support/admin/release consumers, and
//! support export render this set instead of restating the board contract by
//! hand. With `--lines`, prints the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-support --example dump_m5_operator_boards            # JSON
//! cargo run -p aureline-support --example dump_m5_operator_boards -- --lines
//! ```

use aureline_support::m5_operator_boards::{operator_board_lines, operator_board_set};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = operator_board_set();
    set.validate()
        .expect("canonical operator-board set validates");

    if want_lines {
        for line in operator_board_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize operator-board set")
        );
    }
}
