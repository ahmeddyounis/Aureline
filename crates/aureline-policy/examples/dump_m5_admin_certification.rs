//! Headless emitter for the M5 admin-certification bundle.
//!
//! Prints the admin-plane certification — effective-policy, decision-history,
//! endpoint-posture, retention/deletion, offboarding, and procurement/admin-packet
//! truth qualified per profile against the upstream proof lanes that already
//! produce it, bound back to the frozen admin-plane matrix and read across the
//! managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline
//! profiles. Shell admin center, CLI/headless inspect, Help/About, support export,
//! commercial/procurement, and release evidence read this bundle instead of
//! restating admin-plane quality claims by hand. With `--lines`, prints the
//! human-readable projection instead of JSON.
//!
//! ```sh
//! cargo run -p aureline-policy --example dump_m5_admin_certification            # JSON
//! cargo run -p aureline-policy --example dump_m5_admin_certification -- --lines
//! ```

use aureline_policy::m5_admin_certification::{
    admin_certification_bundle, admin_certification_lines,
};

fn main() {
    let want_lines = std::env::args().skip(1).any(|a| a == "--lines");

    let bundle = admin_certification_bundle();
    bundle
        .validate()
        .expect("canonical admin-certification bundle validates");

    if want_lines {
        for line in admin_certification_lines(&bundle) {
            println!("{line}");
        }
    } else {
        println!(
            "{}",
            serde_json::to_string_pretty(&bundle).expect("serialize admin-certification bundle")
        );
    }
}
