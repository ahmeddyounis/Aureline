use aureline_reactive_state::{
    seeded_m5_mutation_lineage_fixtures, seeded_m5_mutation_lineage_packet,
};

fn main() {
    let packet = seeded_m5_mutation_lineage_packet();
    let fixtures = seeded_m5_mutation_lineage_fixtures();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "packet": packet,
            "fixtures": fixtures,
        }))
        .expect("packet and fixtures serialize")
    );
}
