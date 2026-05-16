# Extension settings and permission inspectors (beta)

This page documents the beta extension inspector set. The inspector lets
users, administrators, CLI/headless consumers, and support exports read
the same bounded truth for one installed extension:

- granted, denied, and policy-locked capabilities;
- the current runtime host placement and lifecycle state;
- extension-local settings as source-attributed, diffable rows;
- support-export rows that reproduce the same settings and permission
  truth without raw secret values.

The canonical Rust source lives in
[`crates/aureline-shell/src/extensions/inspectors/`](../../../crates/aureline-shell/src/extensions/inspectors/).
The cross-tool schema is
[`schemas/extensions/extension_inspector.schema.json`](../../../schemas/extensions/extension_inspector.schema.json).
The checked fixtures live in
[`fixtures/ux/m3/extension_inspectors/`](../../../fixtures/ux/m3/extension_inspectors/),
and the generated packet is mirrored in
[`artifacts/extensions/m3/extension_inspectors/`](../../../artifacts/extensions/m3/extension_inspectors/).

## Record shape

The shell emits two top-level records:

1. `extension_inspector_page` combines `extension_permission_inspector`
   and `extension_settings_inspector`.
2. `extension_inspector_support_export` is projected from the same page
   rows and carries a parity fingerprint.

The permission inspector consumes the existing extension contracts:

- permission manifest and effective-permission summary refs;
- runtime admission contract and runtime support-export refs;
- closed capability class, permission scope, declared-vs-effective diff,
  host placement, host supervision, lifecycle, admission decision, and
  admission reason vocabularies.

The settings inspector is intentionally row-based. Each extension-local
setting carries:

- a redacted effective value preview;
- the winning source ref and source scope;
- a source chain with attribution refs;
- one or more field-level diff rows;
- a lock state and optional policy source ref;
- a redaction class that distinguishes metadata-safe values from
  secret-handle rows.

Secret-bearing settings use brokered handle summaries. Raw secret values
must not appear in either the inspector page or the support-export
packet.

## Seeded acceptance row

The seeded `dev.aureline.samples/wasm-notes` packet exercises the
acceptance states:

| Surface | Covered truth |
| --- | --- |
| Permission inspector | 2 granted rows, 2 policy-locked rows, 1 denied row |
| Runtime truth | `wasm_isolated_subprocess`, `separate_subprocess_supervised`, lifecycle `active` |
| Settings inspector | 3 diffable rows, including 1 policy-locked setting and 1 secret-handle setting |
| Support export | same permission row ids, same setting row ids, same summaries, `raw_secret_values_exported=false` |

## Headless consumer

The shell exposes a headless inspector for CLI and support workflows:

```text
cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- page
cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- permissions
cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- settings
cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- validate
```

The `validate` subcommand checks that required permission dispositions
are present, runtime placement and lifecycle are not missing, every
setting row has source attribution and diff rows, and the support export
has parity with the page while exporting no raw secret values.

## How to verify

```text
cargo test -p aureline-shell seeded_page_covers_permission_and_setting_acceptance
cargo test -p aureline-shell support_export_replays_same_truth_without_raw_secret_values
cargo test -p aureline-shell page_fixture_matches_seeded_builder
cargo test -p aureline-shell support_export_fixture_matches_seeded_builder
cargo run -q -p aureline-shell --bin aureline_shell_extension_inspectors -- validate
```
