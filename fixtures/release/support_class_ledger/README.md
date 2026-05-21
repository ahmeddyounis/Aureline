# Support-class ledger fixture cases

Negative fixtures for `ci/check_support_class_ledger.py`. Each `*.json` file is a
complete support-class ledger that is structurally valid **except for one
targeted flaw**, paired in [`cases.json`](./cases.json) with the check id its
flaw must trip.

The gate runs every case during `--check` and fails when a case marked
`rejected` validates clean or trips a different check than expected, so the
downgrade-automation rejections stay live even as the canonical ledger
(`artifacts/release/support_class_ledger.json`) evolves. The Rust contract test
(`crates/aureline-release/tests/support_class_ledger.rs`) also parses each case
and asserts the typed model rejects it.

| Case | Flaw | Expected check id |
|---|---|---|
| `certified_without_manifest.json` | Certified entry references an archetype absent from the manifest | `entry.certified_archetype_not_in_manifest` |
| `narrowing_entry_not_narrowed.json` | A narrowing-state entry still publishes its claimed class | `entry.effective_not_narrowed` |

To regenerate a case after a deliberate ledger shape change, copy the canonical
ledger, reintroduce the single flaw, and confirm the gate still trips the
expected check id.
