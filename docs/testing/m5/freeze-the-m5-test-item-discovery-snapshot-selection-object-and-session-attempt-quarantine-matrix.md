# M5 test-item, discovery-snapshot, selection-object, session-attempt, verdict, and quarantine matrix

This document is the contract for the test-intelligence freeze. It binds **every
claimed M5 test-intelligence surface** to a single bounded qualification matrix,
so Milestone 5 can ship this depth area with canonical implementation, proof,
downgrade behavior, and operator-facing truth instead of ad hoc prototypes, side
spreadsheets, or feature copy that outruns evidence.

M5 increasingly depends on trustworthy test discovery, session, retry, coverage,
flaky, snapshot, and imported CI evidence across framework packs, notebooks, AI
test generation, and review / pipeline flows. Those lanes only stay trustworthy
if test items, discovery snapshots, selection objects, session plans, attempt
records, verdicts, quarantines, and triage packets are canonical product objects
rather than display-name lists and provider dashboards.

The matrix is canonical: no product, docs/help, diagnostics, AI/review, or
release-control surface may present a greener claim than this matrix, and any row
that cannot identify a durable test item, a discovery snapshot, a selection
object, or a session-attempt class — or whose verdict still requires review —
auto-downgrades before it publishes.

Display labels never stand in for durable test identity. Parameterized templates
stay distinct from their concrete invocations. Partial discovery stays visible.
Imported or provider-backed results never masquerade as live local truth.
Quarantine and mute states stay visible, filterable, and exportable. Snapshot,
golden, and test-generation proposals use the same preview / diff / apply rules
used elsewhere.

## Source of truth

- Packet type: `TestQualificationMatrixPacket`
  (`crates/aureline-runtime/src/freeze_the_m5_test_item_discovery_snapshot_selection_object_and_session_attempt_quarantine_matrix/`).
- Boundary schema:
  `schemas/testing/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix.schema.json`.
- Checked support export:
  `artifacts/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix/support_export.json`.
- Markdown summary:
  `artifacts/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix.md`.
- Protected fixtures:
  `fixtures/testing/m5/freeze-the-m5-test-item-discovery-snapshot-selection-object-and-session-attempt-quarantine-matrix/`.
- Conformance dump: `cargo run -p aureline-runtime --example dump_m5_test_qualification_matrix [support|summary]`.

The matrix reuses the frozen test-intelligence vocabulary rather than minting
synonyms: the test-item identity class comes from
`testing_identity::TestItemIdentityClass`, the verdict projection class comes from
`tests::ImportedCiProjectionClass`, and the quarantine / mute state comes from
`tests::FlakyVerdictState`.

## Claimed surfaces

Each claimed test-intelligence surface carries one matrix row:

`framework_test_explorer`, `notebook_test_cells`, `ai_test_generation`,
`review_test_panel`, `ci_import_overlay`, `coverage_surface`,
`flaky_quarantine_board`, `snapshot_golden_review`, and
`support_export_projection`.

Every required surface MUST be represented; a missing surface blocks the matrix.

## Per-row dimensions

Each row identifies the four objects a claimed surface must own:

- **test-item identity class** — `stable`, `remap_review_required`,
  `imported_read_only`, `display_text_only_denied`, or `unknown_requires_review`.
  Only `stable` and `imported_read_only` back a public claim; the others force an
  auto-downgrade.
- **discovery-snapshot class** — `complete_discovery`,
  `partial_visible_discovery`, `streaming_discovery`,
  `provider_imported_discovery`, or `stale_cached_discovery`.
- **selection-object class** — `durable_identity_selection`,
  `visible_range_selection`, `query_matched_selection`, or
  `provider_scoped_selection`.
- **session-attempt class** — `local_live_session`, `rerun_attempt_lineage`,
  `imported_ci_session`, or `mixed_local_imported_session`.

A row also names its **verdict projection class**
(`not_imported_ci`, `authoritative_imported_read_only`,
`stale_imported_read_only`, `fresh_local_reconfirmation`, or
`imported_ci_projection_unknown_requires_review`), its **selection-object
contract**, its **triage-packet contract** (carrying the flaky / quarantine
state), and its **proposal descriptors**.

### Proposal descriptors

Each proposal a surface offers declares its kind (`generate_test`,
`accept_snapshot`, `update_golden`, `accept_baseline`, or `apply_codemod`) and its
preview / apply requirements. Every such proposal touches source or a checked-in
artifact, so it **must** render a reviewable diff and gate behind an explicit
apply step before commit — regardless of which surface offers it.

### Triage packets

The triage-packet contract carries the quarantine / mute state and keeps it
visible, filterable, exportable, and evidence-backed. A `muted` or
`reproduced_flaky` state must carry renewal / expiry semantics rather than hiding
indefinitely.

## Auto-downgrade

When every object dimension (test item, discovery snapshot, selection object,
session attempt) is identified and the verdict does not still require review, the
row's `effective_qualification` equals its `claimed_qualification`. Otherwise the
row auto-downgrades: its effective qualification ranks strictly below its claim
(`stable` > `beta` > `preview` > `experimental` > `held` > `unavailable`), it
records a downgrade trigger (`unidentified_test_item`,
`display_only_test_identity`, `unidentified_discovery_snapshot`,
`unidentified_selection_object`, `unidentified_session_attempt`,
`imported_verdict_requires_review`, `partial_discovery_limited`,
`quarantine_debt_unresolved`, `provider_narrowed`, or
`upstream_dependency_narrowed`), and it carries a precise degraded label. A
generic provider error never stands in for a precise downgrade truth.

## Guardrails

The matrix refuses to publish unless:

- display labels never stand in for durable test identity;
- parameterized templates stay distinct from their concrete invocations;
- partial discovery stays visible, never hidden behind a complete-looking set;
- imported / provider-backed results never masquerade as live local truth;
- quarantine / mute states stay visible, filterable, and exportable; and
- snapshot / golden / test-generation proposals use preview / diff / apply; and
- any row lacking an identified test / discovery / selection / session object
  auto-downgrades below its claim.

## Consumer projection

Product, docs/help, diagnostics, AI/review, and release-control surfaces ingest
this matrix directly instead of cloning test-state terminology by surface.
Downgraded rows are visibly labeled below current in every consumer surface.

## Boundary safety

Raw test source, raw provider payloads, raw log bytes, provider cursors,
credentials, and raw artifact bodies never cross this boundary. The packet carries
only typed class tokens, booleans, opaque ids, and redaction-aware reviewable
labels.
