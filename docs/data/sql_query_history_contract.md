# SQL query-history, replay-mode, and literal-redaction contract

This document is the normative contract for database query history. It
narrows the broader database-tooling contract around one risk surface:
previously-run SQL must not become a hidden credential cache, a raw
literal archive, or a production mutation shortcut.

Every database surface that records, reopens, replays, exports, or
links a SQL statement reads these companion artifacts:

- [`/schemas/data/query_history_entry.schema.json`](../../schemas/data/query_history_entry.schema.json)
  — boundary schema for `query_history_entry_record` and
  `query_history_entry_audit_event_record`.
- [`/schemas/data/query_replay_mode.schema.json`](../../schemas/data/query_replay_mode.schema.json)
  — boundary schema for `query_replay_mode_record`.
- [`/fixtures/data/query_history_cases/`](../../fixtures/data/query_history_cases/)
  — worked query-history and replay-mode cases.
- [`/docs/data/database_tooling_contract.md`](./database_tooling_contract.md)
  — broader connection-profile, statement-safety, result-grid, and
  query-history composition contract.

This contract reuses, without redefining, the secret-broker redaction
rules in
[`/docs/adr/0007-secret-broker-credential-handle-trust-store-redaction.md`](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md),
the storage and retention vocabulary in
[`/docs/governance/storage_and_retention_vocabulary.md`](../governance/storage_and_retention_vocabulary.md),
the privacy lifecycle terms in
[`/docs/governance/privacy_history_and_lifecycle_contract.md`](../governance/privacy_history_and_lifecycle_contract.md),
and the support bundle contract in
[`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md).

## Query-History Entry

A `query_history_entry_record` is a metadata and linkage packet, not a
SQL text cache. It MUST carry all of these fields:

| Field group | Required truth |
|---|---|
| Statement posture | `statement_template_posture_class`, `parameter_placeholder_posture_class`, `body_label_opaque_ref`, and optional `bind_value_set_label_opaque_ref`. Raw SQL bodies and bind values stay in local stores. |
| Connection capture | Captured connection profile ref, connection class, environment class, boundary label, write-capability posture, engine class/version, auth-context fingerprint ref, and policy epoch ref where available. |
| Execution origin | The surface that produced the entry, such as desktop SQL editor, CLI runner, AI-tool review surface, automation run review, notebook handoff, support/export reader, or admin audit reader. |
| Safety capture | Captured `statement_safety_class`, statement-safety result ref, and a redaction-safe disclosure sentence. |
| Result summary | Result-size class, row-count truth class, returned-row bucket, byte-size bucket, truncation state, and optional result-grid ref. |
| Replay baseline | Captured engine/version, captured connection profile ref, captured safety label, and typed replay-drift risk. |
| Privacy state | Retention class, storage mode, retention limit, sharing posture, literal handling, clear-history scopes, support-export behavior, max retained entries, and max age. |
| Downstream links | Refs to replay-mode rows, explain-plan views, result exports, notebook handoffs, incidents, and audit packets, with a copy policy that forbids raw literals by default. |

The default posture is `local_only_default_no_remote_retention`,
`local_first_workspace_store_default`,
`bounded_by_count_and_age_default`, and
`literal_redacted_at_boundary_default`. A surface MAY offer stricter
storage, shorter retention, or more redaction. It MUST NOT widen to
workspace share, org share, support export, managed retention, or raw
literal disclosure without an explicit admission path recorded on the
entry or a linked audit event.

## Replay Modes

Every replay affordance MUST resolve to exactly one
`query_replay_mode_record` before execution or review. The record's
label is mandatory; a surface that only says "Run again" is
non-conforming.

| Replay mode | Required label | Execution truth |
|---|---|---|
| `exact_rerun_same_connection` | `Exact rerun on same connection` | Executable only after the captured connection profile/session, auth epoch, engine version, safety class, and policy epoch are verified as the same context. |
| `rerun_with_current_auth_context` | `Rerun with current auth/context` | Executable only after review; auth, engine version, connection policy, and statement safety are re-resolved under the current context before execution. |
| `open_for_review_only` | `Open for review only` | Never executable from that action. Opens the redacted template and metadata for inspection, copy review, explain-plan comparison, or manual editing. |
| `blocked_by_drift_or_policy` | `Blocked by drift/policy` | Not executable. The record must cite the typed denial reason, such as auth drift, engine drift, connection-class drift, policy expiry, production policy, retention policy, or statement-safety block. |

No replay mode may claim certainty it cannot prove. If the captured
auth epoch, engine version, connection class, safety classification, or
policy epoch cannot be verified, the mode is either
`rerun_with_current_auth_context`, `open_for_review_only`, or
`blocked_by_drift_or_policy`; it is not an exact rerun.

## Retention, Clear, And Export

Query history is bounded and local-first by default:

- Local history MUST declare both `max_retained_entries` and
  `max_age_days`.
- Clear-history actions MUST name their scope:
  `clear_selected_entry_only`, `clear_current_connection_history`,
  `clear_current_workspace_history`, `clear_all_local_query_history`,
  `clear_support_export_projection`, or
  `clear_policy_locked_not_user_clearable`.
- Entries under policy hold, audit-only retention, or managed admin
  publication are not silently user-clearable; the UI and CLI must show
  the locked scope.
- Support/export projections default to metadata, redacted templates,
  refs, hashes, buckets, and omission markers. They do not copy raw
  credentials, raw statement bodies, full literals, bind values, row
  payloads, or raw explain-plan bodies.
- Literal disclosure beyond the redacted boundary requires explicit
  user opt-in and is admissible only for user-authored local entries.

Support/export packets may include the connection class, engine family
and version label, execution origin, statement-safety class,
result-size summary, truncation state, replay-mode label, denial
reason, and linked audit refs. They include raw SQL, raw literals, or
row data only through a separate reviewed local opt-in path governed by
the support-bundle and privacy-history contracts.

## Linkage Rules

Query-history entries link outward by opaque ref:

- Explain-plan views cite the query-history entry and engine/version
  basis. They do not copy raw explain-plan bodies by default.
- Result exports cite the history entry, result-grid record, row-count
  truth, truncation state, export format, and redaction posture.
- Notebook handoffs cite the history entry and result-grid record, and
  preserve whether the handoff is typed or textual fallback.
- Incident workspaces and audit packets cite the entry, replay mode,
  denial reason, and audit event refs without copying credentials or
  full literals.

The default `copy_policy_class` is
`refs_only_no_raw_literals_default`. Any downstream surface that needs
more than refs or redacted summaries must present a review step and
record the explicit admission.

## Fixtures

The worked cases in
[`/fixtures/data/query_history_cases/`](../../fixtures/data/query_history_cases/)
cover:

- an embedded local database with exact same-connection rerun;
- a remote read-only session that reruns with current auth/context;
- a production-labeled target opened for review only;
- a blocked replay after auth-context drift;
- a support/export redaction packet that keeps only redacted templates,
  refs, hashes, and audit linkages.

## Out Of Scope

This contract does not implement the SQL editor UX, engine adapters,
query planner, explain-plan visualizer, result-grid renderer, or
support-bundle uploader. It freezes the record shapes and vocabulary
those implementations must emit.

## Versioning

Every boundary record carries an integer schema version. Adding enum
values or optional fields is additive-minor and bumps the relevant
schema version. Removing or repurposing a value, weakening a redaction
rule, changing a required label, or making a previously explicit
replay/retention state implicit is breaking and requires a new decision
row co-signed by `security_trust_review` and
`product_scope_review`.
