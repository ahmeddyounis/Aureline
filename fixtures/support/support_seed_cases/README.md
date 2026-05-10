# Support-bundle seed fixtures

These fixtures pin the snapshot shape minted by the support-bundle seed
under [`/crates/aureline-support`](../../../crates/aureline-support).
They are the reviewer-facing artifacts for the protected walk and the
failure drill described in
[`/docs/support/support_bundle_seed.md`](../../../docs/support/support_bundle_seed.md).

## Cases

| File | Drill |
|---|---|
| `protected_walk_default_local_preview.json` | Default local preview minted on a deterministic dev build. Two metadata-only rows (build identity + policy/trust). No prohibited rows; honesty marker is silent. |
| `failure_drill_secret_bearing_prohibited.json` | Failure drill where a queued secret-bearing row is rewritten to `prohibited` before any preview render. The manifest carries a typed excluded-class entry, the redaction report names the prohibited row, and the honesty marker lights. |

## Refresh

The fixtures are re-emitted by the integration test
[`/crates/aureline-support/tests/support_bundle_seed_protected_walk.rs`](../../../crates/aureline-support/tests/support_bundle_seed_protected_walk.rs).
Run `cargo test -p aureline-support --test support_bundle_seed_protected_walk -- --ignored emit_fixtures`
to regenerate them after a contract change.
