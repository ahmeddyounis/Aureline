//! Dumps the canonical M5 exception/expiry packet and its projections as YAML.
//!
//! Run with
//! `cargo run -p aureline-policy --example dump_m5_exception_expiry_fixtures`
//! and write the output to
//! `fixtures/governance/m5_exception_expiry/canonical_packet.yaml`
//! whenever the seeded packet changes so the checked-in fixture stays in sync.
//!
//! Pass a subcommand to inspect a derived projection instead of the packet:
//!
//! ```sh
//! cargo run -p aureline-policy --example dump_m5_exception_expiry_fixtures -- request-sheets
//! cargo run -p aureline-policy --example dump_m5_exception_expiry_fixtures -- approval-history
//! cargo run -p aureline-policy --example dump_m5_exception_expiry_fixtures -- expiry-banners
//! cargo run -p aureline-policy --example dump_m5_exception_expiry_fixtures -- revalidation
//! ```

use aureline_policy::m5_exception_expiry::seeded_m5_exception_expiry_packet;

fn main() {
    let packet = seeded_m5_exception_expiry_packet();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let yaml = match args.first().map(String::as_str) {
        Some("request-sheets") => serde_yaml::to_string(&packet.request_sheets()),
        Some("approval-history") => serde_yaml::to_string(&packet.approval_history()),
        Some("expiry-banners") => serde_yaml::to_string(&packet.expiry_banners()),
        Some("revalidation") => serde_yaml::to_string(&packet.self_revalidation()),
        _ => serde_yaml::to_string(&packet),
    }
    .expect("serializes to YAML");
    print!("{yaml}");
}
