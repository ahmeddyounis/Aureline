//! Conformance dump for the commercial honesty-certification packet.
//!
//! Prints the canonical certification packet so the checked-in artifact and
//! fixtures can be regenerated deterministically from the builder.
//!
//! ```text
//! cargo run -p aureline-service --example dump_m5_commercial_honesty_certification -- canonical
//! ```

use aureline_service::m5_commercial_honesty_certification::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "canonical".to_owned());
    let packet = match which.as_str() {
        "canonical" => canonical_stable_honesty_certification_packet(),
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    };
    let violations = packet.validate();
    assert!(
        violations.is_empty(),
        "dump packet failed validation: {violations:?}"
    );
    let backing = packet.cross_check_backing_consumers();
    assert!(
        backing.is_empty(),
        "dump packet drifted from its backing consumers: {backing:?}"
    );
    println!("{}", packet.export_safe_json());
}
