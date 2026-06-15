//! Conformance dump for the metering-degradation rule set.
//!
//! Prints the canonical rule set so the checked-in artifact and fixtures can be
//! regenerated deterministically from the builder.
//!
//! ```text
//! cargo run -p aureline-service --example dump_m5_metering_degradation_rules -- canonical
//! ```

use aureline_service::m5_metering_degradation_rules::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "canonical".to_owned());
    let set = match which.as_str() {
        "canonical" => canonical_stable_metering_degradation_rule_set(),
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    };
    let violations = set.validate();
    assert!(
        violations.is_empty(),
        "dump set failed validation: {violations:?}"
    );
    let control_plane = set.cross_check_against_control_plane();
    assert!(
        control_plane.is_empty(),
        "dump set drifted from the control plane: {control_plane:?}"
    );
    println!("{}", set.export_safe_json());
}
