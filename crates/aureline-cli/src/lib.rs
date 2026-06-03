//! CLI/headless schema stabilization, machine-readable output contracts, and
//! support/export compatibility promises.
//!
//! This crate owns the typed register that binds every CLI command surface,
//! headless output schema, machine-readable output format, and support/export
//! compatibility promise to the stable claim manifest entry whose lifecycle
//! label it backs. Downstream surfaces — docs, Help/About, support exports,
//! and release-center dashboards — ingest the register directly instead of
//! cloning status text.

#![doc(html_root_url = "https://docs.rs/aureline-cli/0.0.0")]

pub mod stabilize_stable_cli_headless_schemas_machine_readable_output;

pub use stabilize_stable_cli_headless_schemas_machine_readable_output::{
    current_stabilize_stable_cli_headless_schemas_machine_readable_output, CliHeadlessAction,
    CliHeadlessExportProjection, CliHeadlessExportRow, CliHeadlessGapReason, CliHeadlessKind,
    CliHeadlessPublicationRecord, CliHeadlessRow, CliHeadlessRule, CliHeadlessState,
    CliHeadlessSummary, CliHeadlessViolation, CliSchemaDetail, MachineReadableDetail,
    StabilizeStableCliHeadlessSchemasMachineReadableOutput, SupportExportCompatDetail,
    STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_JSON,
    STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_PATH,
    STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_RECORD_KIND,
    STABILIZE_STABLE_CLI_HEADLESS_SCHEMAS_MACHINE_READABLE_OUTPUT_SCHEMA_VERSION,
};
