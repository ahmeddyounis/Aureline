//! Conformance dump for the commercial-control-plane matrix.
//!
//! Prints either the canonical matrix or a matrix narrowed by one active managed
//! state, so the checked-in artifact and fixtures can be regenerated
//! deterministically from the builder.
//!
//! ```text
//! cargo run -p aureline-service --example dump_m5_commercial_control_plane -- canonical
//! cargo run -p aureline-service --example dump_m5_commercial_control_plane -- managed_blocked
//! cargo run -p aureline-service --example dump_m5_commercial_control_plane -- grace_period
//! cargo run -p aureline-service --example dump_m5_commercial_control_plane -- seat_removed
//! cargo run -p aureline-service --example dump_m5_commercial_control_plane -- org_switched
//! cargo run -p aureline-service --example dump_m5_commercial_control_plane -- meter_stale
//! ```

use aureline_service::m5_commercial_control_plane::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "canonical".to_owned());
    let mut matrix = canonical_stable_commercial_control_plane_matrix();
    match which.as_str() {
        "canonical" => {}
        "signed_in" => matrix.apply_managed_state(ManagedStateClass::SignedIn),
        "local_only" => matrix.apply_managed_state(ManagedStateClass::LocalOnly),
        "reauth_required" => matrix.apply_managed_state(ManagedStateClass::ReauthRequired),
        "managed_blocked" => matrix.apply_managed_state(ManagedStateClass::ManagedBlocked),
        "grace_period" => matrix.apply_managed_state(ManagedStateClass::GracePeriod),
        "seat_removed" => matrix.apply_managed_state(ManagedStateClass::SeatRemoved),
        "plan_downgrade" => matrix.apply_managed_state(ManagedStateClass::PlanDowngrade),
        "org_switched" => matrix.apply_managed_state(ManagedStateClass::OrgSwitched),
        "forecast_threshold" => matrix.apply_managed_state(ManagedStateClass::ForecastThreshold),
        "meter_stale" => matrix.apply_managed_state(ManagedStateClass::MeterStale),
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
    let violations = matrix.validate();
    assert!(
        violations.is_empty(),
        "dump matrix failed validation: {violations:?}"
    );
    println!("{}", matrix.export_safe_json());
}
