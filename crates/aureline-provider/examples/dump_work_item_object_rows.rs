fn main() {
    let packet = aureline_provider::seeded_work_item_object_rows_packet();
    println!(
        "{}",
        serde_json::to_string_pretty(&packet).expect("work-item object rows serialize")
    );
}
