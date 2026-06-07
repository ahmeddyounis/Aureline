# Database Statement-Safety And Result-Grid Qualification

This packet qualifies the database/data-tooling rows that are visible in the
promoted build. It prevents connection pickers, SQL run bars, result grids,
query history, explain plans, notebook/chart/AI handoffs, and support exports
from inheriting stable language from generic table, notebook, request, or
runtime surfaces.

Canonical files:

- Packet:
  [`artifacts/release/m4/database-statement-safety-and-result-grid-qualification.json`](./database-statement-safety-and-result-grid-qualification.json)
- Schema:
  [`schemas/release/database-statement-safety-and-result-grid-qualification.schema.json`](../../../schemas/release/database-statement-safety-and-result-grid-qualification.schema.json)
- Rust owner:
  `aureline_data::database_qualification`
- Fixtures:
  [`fixtures/data/m4/database-statement-safety-and-result-grid/cases.json`](../../../fixtures/data/m4/database-statement-safety-and-result-grid/cases.json)

## Qualification Boundary

Stable coverage is limited to governed record contracts and visible guard
rows. It covers:

- connection class, execution origin, auth-source mode, target identity,
  engine, database/schema ref, and read-only/write-capable/policy-blocked
  posture;
- statement-safety class, statement count, transaction posture, object-impact
  hint posture, and protected-target review or block behavior;
- result-grid virtualization, large-cell expansion, typed headers,
  truncation truth, binary/rich safe-preview posture, and explicit
  copy/export redaction;
- query-history metadata-first retention, local-first bounds, redaction,
  statement fingerprinting, and exclusion of raw secrets, raw statements, and
  row payloads by default;
- explain-plan mode, engine/version binding, capture time, estimated versus
  actual truth, and stale imported labels;
- notebook, chart, AI, clipboard, and support/export handoff refs preserving
  source query, row/column scope, type fidelity, freshness, destination class,
  and share/local restrictions.

It does not qualify full multi-engine live SQL execution, direct row mutation,
or mature database adapter depth. Those rows are deliberately narrowed until
current connector, write-guard, transaction, cancel, row-edit, and rollback
evidence exists for each claimed surface.

## Connection Profile And Run-Bar Truth

The connection corpus covers embedded local, local network/container dev,
remote controlled, cloud/warehouse, and imported snapshot origins. Every row
projects the same truth through picker rows, run bars, query history, export
review, and explain-plan panes:

- engine and current database/schema;
- connection class and execution origin;
- auth source mode without raw credentials;
- opaque target identity;
- write posture as `read_only`, `write_capable`, or `policy_blocked`.

No row may infer target truth from shell history, notebook state, or generic
table viewer state.

## Statement-Safety And Write-Guard Labs

The statement labs prove that read-only, DML, DDL, session-affecting,
multi-statement, and ambiguous classes remain distinct before execution.
Destructive or ambiguous statements require review or block on protected
targets, and mutation-class rows keep transaction posture and object-impact
availability visible.

The packet treats a generic `Run` control without these fields as
non-conforming.

## Result-Grid Scale, Export, And Handoff Labs

Result-grid labs require virtualization by default, explicit large-cell
expansion, typed column headers, truncation disclosure, safe-preview handling
for binary/rich payloads, and copy/export actions that preserve row scope,
format, and redaction mode.

Handoff labs require the same source refs and redaction posture when data moves
to notebook, chart, AI review, clipboard, or support/export lanes. Handoffs are
not clipboard shortcuts that lose provenance.

## Query-History And Explain-Plan Truth

Query history remains local-first, bounded, redactable, metadata-first, and
fingerprint-oriented. Support/export projections carry redacted templates,
refs, hashes, buckets, and omission markers rather than raw credentials, full
SQL bodies, bind values, or row payloads by default.

Explain plans distinguish estimated, actual, and imported-stale modes. An
estimated plan does not imply execution. Imported stale plans keep capture time
and engine/version refs visible and do not claim cross-engine comparability.

## Support Export Projection

Support-safe projection includes:

- surface id, displayed label, proof ref, and downgrade reason;
- connection class, engine family, execution origin, auth-source class, target
  ref, write posture, and policy-blocked state;
- statement-safety class, transaction posture, object-impact availability, and
  block/review state;
- result scope, truncation state, export format/redaction posture, and handoff
  destination class;
- query-history retention class and explain-plan freshness class.

It excludes raw credentials, raw hostnames, raw ports, raw SQL bodies, raw bind
values, row payloads, and raw plan bodies unless a separate reviewed local
opt-in path governs that payload.

## Downgrade Rule

Any database surface without a current packet proving connection safety,
statement-safety classification, result-grid virtualization/export truth,
query-history redaction, explain-plan freshness, and handoff lineage must render
below stable. The honest fallback is preview, inspect-only, import-only, or
labs, depending on what remains safe.
