//! Conformance dump for the usage-and-forecast view set.
//!
//! Prints either the canonical view set or a set narrowed by one active managed
//! state, so the checked-in artifact and fixtures can be regenerated
//! deterministically from the builder.
//!
//! ```text
//! cargo run -p aureline-service --example dump_m5_usage_forecast_views -- canonical
//! cargo run -p aureline-service --example dump_m5_usage_forecast_views -- managed_blocked
//! cargo run -p aureline-service --example dump_m5_usage_forecast_views -- grace_period
//! cargo run -p aureline-service --example dump_m5_usage_forecast_views -- meter_stale
//! ```

use aureline_service::m5_commercial_control_plane::ManagedStateClass;
use aureline_service::m5_usage_forecast_views::*;

fn main() {
    let which = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "canonical".to_owned());
    let mut set = canonical_stable_usage_forecast_view_set();
    match which.as_str() {
        "canonical" => {}
        "signed_in" => set.apply_managed_state(ManagedStateClass::SignedIn),
        "local_only" => set.apply_managed_state(ManagedStateClass::LocalOnly),
        "reauth_required" => set.apply_managed_state(ManagedStateClass::ReauthRequired),
        "managed_blocked" => set.apply_managed_state(ManagedStateClass::ManagedBlocked),
        "grace_period" => set.apply_managed_state(ManagedStateClass::GracePeriod),
        "seat_removed" => set.apply_managed_state(ManagedStateClass::SeatRemoved),
        "plan_downgrade" => set.apply_managed_state(ManagedStateClass::PlanDowngrade),
        "org_switched" => set.apply_managed_state(ManagedStateClass::OrgSwitched),
        "forecast_threshold" => set.apply_managed_state(ManagedStateClass::ForecastThreshold),
        "meter_stale" => set.apply_managed_state(ManagedStateClass::MeterStale),
        other => {
            eprintln!("unknown dump selector: {other}");
            std::process::exit(2);
        }
    }
    let violations = set.validate();
    assert!(
        violations.is_empty(),
        "dump set failed validation: {violations:?}"
    );
    println!("{}", set.export_safe_json());
}
