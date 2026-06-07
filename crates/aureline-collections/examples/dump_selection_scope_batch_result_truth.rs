use aureline_collections::seeded_selection_scope_packet;

fn main() {
    let packet = seeded_selection_scope_packet();
    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("packet must serialize")
    );
}
