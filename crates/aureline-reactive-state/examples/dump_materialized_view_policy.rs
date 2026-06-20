use aureline_reactive_state::{
    seeded_materialized_view_policy, seeded_materialized_view_policy_fixtures,
};

fn main() {
    let packet = seeded_materialized_view_policy();
    let fixtures = seeded_materialized_view_policy_fixtures();
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "packet": packet,
            "fixtures": fixtures,
        }))
        .expect("packet and fixtures serialize")
    );
}
