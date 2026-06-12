//! Dumps the seeded provider scope-review page and support export as JSON.

use std::env;

use aureline_provider::seeded_provider_scope_review_page;

fn main() {
    let mode = env::args().nth(1).unwrap_or_else(|| "page".to_owned());
    let page = seeded_provider_scope_review_page();

    let json = match mode.as_str() {
        "page" => serde_json::to_string_pretty(&page).expect("serialize page"),
        "support_export" => serde_json::to_string_pretty(&page.support_export_projection())
            .expect("serialize support export"),
        other => panic!("unsupported mode: {other}"),
    };

    println!("{json}");
}
