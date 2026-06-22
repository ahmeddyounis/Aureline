//! Headless emitter for the M5 action-plan set.
//!
//! Prints the canonical action-plan / checklist workspaces — incident response,
//! support remediation, and admin access review — as ordered, ownership-bearing
//! next-step plans over canonical incident/support/admin objects. Each item keeps
//! its local check-off distinct from any external mutation, links canonical
//! evidence, carries approval/policy state and due/expiry, and never lets a local
//! checkoff resolve a provider-owned object; each plan declares explicit scope and
//! boundary truth and freezes a snapshot handoff. Shell UI, CLI/headless inspect,
//! incident/support/admin/managed consumers, and support export render this set
//! instead of restating the action-plan contract by hand. With `--lines`, prints
//! the human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-support --example dump_m5_action_plans            # JSON
//! cargo run -p aureline-support --example dump_m5_action_plans -- --lines
//! ```

use aureline_support::m5_action_plans::{action_plan_lines, action_plan_set};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let set = action_plan_set();
    set.validate().expect("canonical action-plan set validates");

    if want_lines {
        for line in action_plan_lines(&set) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&set).expect("serialize action-plan set")
        );
    }
}
