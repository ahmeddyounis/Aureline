fn main() {
    let packet = aureline_infra::seeded_relation_graph_incident_support_parity_packet();
    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("fixture serializes")
    );
}
