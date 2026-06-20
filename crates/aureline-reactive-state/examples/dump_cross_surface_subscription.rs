//! Regenerates the checked-in cross-surface subscription artifact and
//! fixtures from the seeded contract.
//!
//! Run with:
//!
//! ```bash
//! cargo run -p aureline-reactive-state --example dump_cross_surface_subscription
//! ```
//!
//! Keys are canonicalized through `serde_json::Value` (a sorted map) so
//! the on-disk JSON is byte-stable across hosts. The replay gate in
//! `tests/cross_surface_subscription.rs` asserts the on-disk artifact and
//! fixtures match the seeded projection and satisfy the frozen contract.

use std::fs;
use std::path::{Path, PathBuf};

use aureline_reactive_state::{
    seeded_cross_surface_subscription_fixtures, seeded_cross_surface_subscription_packet,
    CROSS_SURFACE_SUBSCRIPTION_FIXTURE_DIR, CROSS_SURFACE_SUBSCRIPTION_PACKET_REF,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("..").join("..")
}

fn canonical_json<T: serde::Serialize>(value: &T) -> String {
    let value: serde_json::Value = serde_json::to_value(value).expect("value serializes");
    let mut out = serde_json::to_string_pretty(&value).expect("pretty prints");
    out.push('\n');
    out
}

fn main() {
    let root = repo_root();

    let packet = seeded_cross_surface_subscription_packet();
    let packet_path = root.join(CROSS_SURFACE_SUBSCRIPTION_PACKET_REF);
    fs::create_dir_all(packet_path.parent().expect("packet has a parent"))
        .expect("artifact dir exists");
    fs::write(&packet_path, canonical_json(&packet)).expect("packet writes");
    println!("wrote {}", packet_path.display());

    let fixtures_dir = root.join(CROSS_SURFACE_SUBSCRIPTION_FIXTURE_DIR);
    fs::create_dir_all(&fixtures_dir).expect("fixture dir exists");
    for fixture in seeded_cross_surface_subscription_fixtures() {
        let short = fixture
            .fixture_id
            .rsplit(':')
            .next()
            .expect("fixture id has a tail");
        let path = fixtures_dir.join(format!("{short}.json"));
        fs::write(&path, canonical_json(&fixture)).expect("fixture writes");
        println!("wrote {}", path.display());
    }
}
