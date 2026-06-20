//! Emits the seeded extension/companion contributed-locale support report.
//!
//! ```sh
//! cargo run -q -p aureline-i18n --example dump_contributed_locale_support
//! cargo run -q -p aureline-i18n --example dump_contributed_locale_support -- validate
//! ```
//!
//! Regenerate the canonical fixture with:
//!
//! ```sh
//! cargo run -q -p aureline-i18n --example dump_contributed_locale_support \
//!   > fixtures/i18n/extension-companion-pack-compat/support_report.json
//! ```

use aureline_i18n::seeded_contributed_locale_support_report;

fn main() {
    if let Err(err) = run() {
        eprintln!("{err}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let report = seeded_contributed_locale_support_report();
    report.validate().map_err(|findings| {
        format!("seeded contributed-locale report failed validation: {findings:?}")
    })?;

    match args.first().map(String::as_str) {
        Some("report") | None => println!("{}", serde_json::to_string_pretty(&report)?),
        Some("validate") => match report.validate() {
            Ok(()) => println!("ok"),
            Err(findings) => {
                for finding in findings {
                    eprintln!("{}: {}", finding.row_ref, finding.message);
                }
                std::process::exit(3);
            }
        },
        Some(other) => return Err(format!("unknown selector: {other}").into()),
    }
    Ok(())
}
