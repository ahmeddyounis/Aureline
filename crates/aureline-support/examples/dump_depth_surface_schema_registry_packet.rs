use aureline_support::seeded_depth_surface_schema_registry_packet;

fn main() {
    let packet = seeded_depth_surface_schema_registry_packet();
    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("serialize packet")
    );
}
