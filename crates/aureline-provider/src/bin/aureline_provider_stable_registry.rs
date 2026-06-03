//! Emit the seeded stable provider-account/install-grant registry packet as JSON.

use std::io::{self, Write};

use aureline_provider::seeded_stable_provider_account_install_grant_registry_packet;

fn main() {
    let packet = seeded_stable_provider_account_install_grant_registry_packet();
    let json = serde_json::to_string_pretty(&packet).expect("serialize packet");
    io::stdout()
        .write_all(json.as_bytes())
        .expect("write stdout");
    io::stdout().write_all(b"\n").expect("write newline");
}
