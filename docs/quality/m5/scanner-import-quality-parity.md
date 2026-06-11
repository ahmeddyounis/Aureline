# Scanner-import quality parity

This document describes the canonical **scanner-import quality-parity** packet
used by Aureline's Problems, review, pipeline, support-export, and release
surfaces. It is the user-facing companion to the governed artifact at
`artifacts/quality/m5/scanner-import-quality-parity.json` and the typed model in
the `aureline-runtime` crate (`scanner_import_quality_parity`).

## Why this packet exists

The stable scanner-import lane (`crate::scanner_import`) already normalizes SARIF
2.1.0 and structured scanner payloads into read-only imported findings, delta
packets, CI/local parity views, review packets, pipeline-viewer projections,
support exports, and release evidence. This packet does **not** re-derive any of
that work. It publishes one inspectable truth packet that pins down, for the
whole imported code-quality lane:

- **which product surfaces ingest imported scanner truth, and what each
  guarantees**,
- **which parity states are first-class** rather than hidden caveats, and
- **what must hold before any imported-quality row promotes.**

It carries no raw scanner bodies, raw source, raw paths, provider payloads, or
secrets.

## Surfaces

Each surface row is bound to the stable record-kind it consumes from the
scanner-import lane and asserts that imported results stay **source-labeled**,
**freshness-labeled**, **baseline-compatible**, and **read-only**, with an
explicit non-empty list of downgrade behaviors:

| Surface | Record kind it consumes |
| --- | --- |
| `problems` | `imported_scanner_problems_projection_alpha` |
| `review_workspace` | `diagnostic_review_packet_alpha` |
| `pipeline_viewer` | `imported_scanner_pipeline_viewer_projection` |
| `support_bundle` | `imported_scanner_support_export_alpha` |
| `release_packet` | `scanner_import_release_packet` |
| `ci_local_parity` | `scanner_ci_local_parity_view` |
| `cli` | `scanner_import_cli_projection` |

The `pipeline_viewer` surface is backed by
`ScannerImportSessionAlpha::pipeline_viewer_projection`, which threads the same
diff-scoped findings, suppressions, and baseline shifts the review and support
surfaces use, while keeping every row read-only imported evidence labeled by
source, freshness, and per-finding parity state.

## Parity states

Parity gaps are **named states**, not silent caveats. Every state carries
canonical truth flags (`is_gap`, `blocks_exact_delta`, `blocks_promotion`) that
the packet validation enforces against the row, so a gap can never be quietly
recorded as comparable:

- `comparable` — imported and local evidence agree on a compatible baseline.
- `comparable_requires_local_confirmation` — compatible, but an exact claim waits
  on a compatible local run.
- `parity_gap_stale_imported` — recoverable by a rerun or refresh.
- `parity_gap_unmapped_rule` — recoverable by adding a rule-family mapping.
- `parity_gap_rule_pack_mismatch` — blocks promotion; not confirmable away.
- `parity_gap_profile_mismatch` — blocks promotion; not confirmable away.
- `unsupported_scanner_family` — surfaced rather than dropped; blocks promotion.
- `distinct_source_not_comparable` — never promotable as live parity.

## Promotion gate

The promotion gate guards M5 imported-quality rows. It **cannot waive** delta
compatibility, export-safe artifacts, or defined downgrade behavior, and it
requires a signed import/export on mirror-only and air-gapped profiles. The
`promotable` flag is **recomputed** from the surface and parity-state guarantees:
weakening any surface (for example, letting an imported row stop being read-only,
or dropping its downgrade behavior) flips the gate closed and fails validation.
This encodes the acceptance criterion that no scanner or imported-quality row can
promote while missing delta compatibility, export-safe artifacts, or downgrade
behavior.

## Consuming the packet

Load the embedded packet with
`scanner_import_quality_parity::current_scanner_import_quality_parity()` and call
`validate()` to confirm the invariants, or `export_projection()` for a
redaction-safe projection for support and release ingest. The packet is canonical
for this lane: later product surfaces, docs/help, support exports, and
release/public-truth packets should ingest it directly instead of restating
parity status by hand.
