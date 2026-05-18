# Companion Scope / Desktop-Handoff Fixture Corpus

Reviewable fixtures for the scoped companion and desktop-handoff beta
projection in
[`crates/aureline-shell/src/companion_handoff/mod.rs`](../../../../crates/aureline-shell/src/companion_handoff/mod.rs).

The JSON files are literal projections of the seeded
`CompanionScopeBetaPage` produced by the
`aureline_shell_companion_scope` headless inspector.

## Index

| Fixture | Coverage |
| --- | --- |
| [`page.json`](./page.json) | Full page: six claimed workflows, support rows, summary, and zero defects. |
| [`rows.json`](./rows.json) | Source rows for review, docs, light remote edit, session join, CI status, and incident awareness. |
| [`support_rows.json`](./support_rows.json) | Per-row support-safe reconstruction projections. |
| [`support_export.json`](./support_export.json) | Support-export wrapper with page, support rows, case ids, and defect summary. |
| [`drill_stale_label_missing.json`](./drill_stale_label_missing.json) | Negative drill: stale CI row hides freshness labels. |
| [`drill_companion_owns_protected_approval.json`](./drill_companion_owns_protected_approval.json) | Negative drill: companion owns protected approval. |

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- page > fixtures/ux/m3/companion_scope/page.json
cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- rows > fixtures/ux/m3/companion_scope/rows.json
cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- support-rows > fixtures/ux/m3/companion_scope/support_rows.json
cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- support-export > fixtures/ux/m3/companion_scope/support_export.json
cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- drill-stale-label-missing > fixtures/ux/m3/companion_scope/drill_stale_label_missing.json
cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- drill-companion-owns-protected-approval > fixtures/ux/m3/companion_scope/drill_companion_owns_protected_approval.json
```

## Verify

```sh
cargo test -p aureline-shell --test companion_scope_beta_fixtures
cargo run -q -p aureline-shell --bin aureline_shell_companion_scope -- validate
```
