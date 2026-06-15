//! Conformance dump for the entitlement-summary set.
//!
//! Prints the canonical entitlement-summary set, so the checked-in artifact and
//! fixtures can be regenerated deterministically from the builder.
//!
//! ```text
//! cargo run -p aureline-service --example dump_m5_entitlement_summary
//! ```

use aureline_service::m5_entitlement_summary::*;

fn main() {
    let set = canonical_stable_entitlement_summary_set();
    let violations = set.validate();
    assert!(
        violations.is_empty(),
        "dump set failed validation: {violations:?}"
    );
    println!("{}", set.export_safe_json());
}
