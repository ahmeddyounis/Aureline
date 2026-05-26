# rust_daily_driver_quality_truth_packet fixture corpus

Fixture corpus for the M4 stable Rust daily-driver quality truth packet
(`schemas/language/rust_daily_driver_quality_truth.schema.json`).

Each fixture is a `RustDailyDriverQualityTruthPacketInput` with an
`expect` block that pins the materialized packet's promotion state,
finding count, lane and row-class token sets, support-class,
daily-loop step, known-limit, downgrade-automation, and
evidence-class tokens, and the support-export safety verdict. Tests
in
`crates/aureline-language/tests/rust_daily_driver_quality_truth_packet.rs`
load each case and assert that
`RustDailyDriverQualityTruthPacket::materialize` agrees.

Cases:

- `baseline_stable.json` — The Rust daily-driver lane carries a
  `daily_driver_quality` row at `replacement_grade` plus every
  certified `daily_loop_step` row (open_or_import, navigate, edit,
  complete, refactor, run_test_debug, review, migrate, recover),
  framework-pack, migration-evidence, and archetype-repo rows, plus
  cargo-workspace (single-package `Cargo.toml` / `Cargo.lock` and
  multi-package `[workspace]` / `members` / `exclude` /
  `default-members` / `resolver`; `rust-toolchain.toml` channel
  pinning; `CARGO_HOME` / `CARGO_TARGET_DIR` / `CARGO_REGISTRIES_*` /
  `CARGO_NET_OFFLINE` resolution; `[patch]` / `[replace]` /
  `[profile.*]` directives), lint-format (`cargo clippy` /
  `cargo clippy --fix` / `cargo fmt` / `cargo fmt --check`,
  `clippy.toml` / `rustfmt.toml`), test-runner (`cargo test` /
  `cargo test --doc` / `cargo nextest` / `cargo bench`), debugger
  (`rust-lldb` / `rust-gdb` / CodeLLDB / `lldb-dap`), and
  workspace-index (`rust-analyzer` LSP / proc-macro expansion /
  build-script execution / save-analysis / metadata fetch budget)
  rows. Every row binds support, known limit, downgrade automation,
  and evidence classes; narrowed rows carry their disclosure refs,
  and all eight required consumer projections preserve the packet
  verbatim.
- `replacement_grade_with_unbound_evidence_blocks_stable.json` — The
  Rust daily-driver quality row claims `replacement_grade` while its
  evidence class is `evidence_unbound`; the packet blocks the stable
  claim because no archetype, fixture-repo, migration,
  cargo-workspace, lint-format, test-runner, debugger,
  workspace-index, or design-partner evidence backs the row.
- `missing_daily_loop_step_for_replacement_grade_blocks_stable.json` —
  The lane claims `replacement_grade` but the `recover` daily-loop
  step is missing; the packet blocks the stable claim.
- `narrowed_row_missing_disclosure_ref_blocks_stable.json` — A
  known-limit row narrows below replacement grade with
  `cargo_workspace_subset_only` but drops its disclosure ref; the
  packet blocks the stable claim.
- `projection_collapses_evidence_class_vocabulary_blocks_stable.json`
  — The `help_about` consumer projection drops the evidence-class
  vocabulary; the packet blocks the stable claim.
- `raw_source_material_blocks_stable.json` — A daily-driver row
  admits raw source bodies past the boundary; the packet blocks the
  stable claim because raw Rust crate source, `Cargo.lock` checksum
  values, `.env` secrets, and ambient `cargo login` credentials must
  never leak through the daily-driver boundary.
