# M5 Archived-Snapshot Viewers & Analysis-Only Banners: One Vocabulary Across Surfaces

This lane makes every preserved-evidence view say **"this is historical / non-live"** before a user can
mistake it for the current editable object. It is the B149 archive-consumer lane over the five
non-live-evidence object classes frozen in the
[historical-reference matrix](./m5-historical-evidence-ops.md) and made machine-readable by the
historical-snapshot-descriptor implement lane.

- **Module:** `crates/aureline-ui/src/m5_archived_snapshot_viewer_and_analysis_only_banner_consumers/`
- **Schema:** [`schemas/program/m5-archived-snapshot-viewer-consumers.schema.json`](../../schemas/program/m5-archived-snapshot-viewer-consumers.schema.json)
- **Support export:** `artifacts/support/m5-archived-snapshot-viewer-consumers/support_export.json` (+ `matrix.csv`, `summary.md`)
- **Fixtures:** `fixtures/recovery/m5-archived-snapshot-viewer-consumers/`
- **Emitter:** `cargo run -p aureline-ui --example dump_m5_archived_snapshot_viewer_consumers -- <subcommand>`

## What it proves

Every archive-bearing surface — a support bundle viewer, a retirement snapshot page, a review / incident
evidence reopen flow, and the shell, help / docs, runbook-archive, release-center, companion / export,
program-governance, and CLI / export consumers among them — frames a preserved snapshot with **one canonical
archive / state banner and fact grid**: snapshot label, capture time, provenance, analysis-only posture, and
the exact action set allowed on archived evidence.

Three honesty axes mirror the batch acceptance criteria.

1. **One vocabulary / no drift.** For a given preserved-evidence profile every consumer surface presents an
   identical banner grammar — the same banner-role word (a token from the frozen historical-reference role
   vocabulary), snapshot-label word, capture-time word, provenance word, analysis-only-posture word, and
   allowed-action-set word. A surface may narrow *which actions remain* but never reword the grammar.
2. **Analysis-only, never write-capable-as-live.** An archived view exposes **inspect**, **compare**, and
   **export-evidence** actions, and an **open-current-live-object** action *only* where the live target still
   exists (the `live_target_openable` posture). Mutation affordances are disabled by construction — the action
   set is a closed enum with no write / edit variant. No binding may present a write-capable control as if the
   current object were open live, reopen a live target without validating identity / trust / route /
   authority, dead-link an expired or removed artifact instead of showing metadata, leave non-live evidence
   unjoined to its capture context, or let archived / imported evidence look live, writable, or current by
   omission.
3. **Screen-reader and keyboard discoverable.** Every binding names the accessibility routes through which the
   archived / non-live state, its provenance, and the open-live-target action can be discovered without
   pointer-only chrome. Keyboard focus and screen-reader announcement are mandatory.

## Action postures

| Posture | Open-current-live-object? | Disclosure |
| --- | --- | --- |
| `live_target_openable` | yes (validated) | full view, no narrowing |
| `metadata_only_exit` | no (target removed) | metadata-only inspection exit note |
| `imported_offline_only` | no (never live) | imported / offline warning note |
| `exported_redacted` | no | export-safe redaction note |

Narrowing is always disclosed through an explicit note naming the reason, the preserved grammar, and the next
action, so a surface can narrow the action set without quietly implying the object is still live.

## Regenerating the checked-in artifacts

```text
cargo run -p aureline-ui --example dump_m5_archived_snapshot_viewer_consumers -- support-export
cargo run -p aureline-ui --example dump_m5_archived_snapshot_viewer_consumers -- csv
cargo run -p aureline-ui --example dump_m5_archived_snapshot_viewer_consumers -- report
cargo run -p aureline-ui --example dump_m5_archived_snapshot_viewer_consumers -- fixture-metadata-only-narrowed
cargo run -p aureline-ui --example dump_m5_archived_snapshot_viewer_consumers -- fixture-imported-offline-narrowed
cargo run -p aureline-ui --example dump_m5_archived_snapshot_viewer_consumers -- validate
```

The `seed.rs` builders are the only mint-from-truth path; the checked-in JSON must byte-match their output.
