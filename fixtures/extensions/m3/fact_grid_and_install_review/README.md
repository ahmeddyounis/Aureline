# Marketplace fact-grid and install-review fixtures

This corpus pins the shared fact-grid projection used by marketplace result
rows, detail views, native install review, compatibility reports, diagnostics,
and support exports.

## Index

| Fixture | Coverage |
| --- | --- |
| [`fact_grids.json`](./fact_grids.json) | Public, mirrored, private/retest, offline bundle, manual import, and revoked catalog rows with client scope, registry source, compatibility, script/native-build risk, manifest changes, permission deltas, lockfile impact, revocation, rollback, and activation budget. |
| [`support_exports.json`](./support_exports.json) | Metadata-safe support exports quoting the same fact-grid fields without raw artifact or credential data. |

## Regenerate

```sh
cargo run -q -p aureline-shell --bin aureline_shell_marketplace_truth -- fact-grids > fixtures/extensions/m3/fact_grid_and_install_review/fact_grids.json
cargo run -q -p aureline-shell --bin aureline_shell_marketplace_truth -- fact-grid-support-rows > fixtures/extensions/m3/fact_grid_and_install_review/support_exports.json
```

## Verification

```sh
cargo test -p aureline-extensions fact_grid
cargo test -p aureline-shell --test marketplace_truth_beta_fixtures
```
