use std::env;

use aureline_search::{
    seeded_scope_trust_narrowed_search_action_binding_packet, seeded_search_action_binding_packet,
};

fn main() {
    let packet = match env::args().nth(1).as_deref() {
        None | Some("canonical") => seeded_search_action_binding_packet(),
        Some("scope_trust_narrowed") => seeded_scope_trust_narrowed_search_action_binding_packet(),
        Some(other) => {
            panic!("unsupported mode {other}; expected canonical or scope_trust_narrowed")
        }
    };

    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("serialize search action-binding packet")
    );
}
