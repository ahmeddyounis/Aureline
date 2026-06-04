use aureline_collections::seeded_dense_collection_contract_packet;

fn main() {
    let packet = seeded_dense_collection_contract_packet();
    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("packet must serialize")
    );
}
