fn main() {
    let packet = aureline_infra::seeded_source_intelligence_object_packet();
    println!("{}", serde_json::to_string_pretty(&packet).expect("fixture serializes"));
}
