use aureline_vfs::{
    seeded_filesystem_mutation_lineage_matrix_fixtures,
    seeded_filesystem_mutation_lineage_matrix_packet,
};

fn main() {
    let packet = seeded_filesystem_mutation_lineage_matrix_packet();
    let fixtures = seeded_filesystem_mutation_lineage_matrix_fixtures();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "packet": packet,
            "fixtures": fixtures,
        }))
        .expect("packet and fixtures serialize")
    );
}
