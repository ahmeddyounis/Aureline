fn main() {
    let packet = aureline_infra::seeded_provider_overlay_handoff_packet();
    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("fixture serializes")
    );
}
