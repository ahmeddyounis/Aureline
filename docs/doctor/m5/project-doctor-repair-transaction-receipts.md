# Project Doctor guided repair transaction receipts

This document describes the canonical packet that turns each guided repair into
an auditable **repair transaction receipt** for the M5 failure families. It is
the user-facing companion to the governed artifact at
`artifacts/doctor/m5/project-doctor-repair-transaction-receipts.json`, the
boundary schema at
`schemas/doctor/project-doctor-repair-transaction-receipts.schema.json`, and the
typed model in the `aureline-doctor` crate
(`guided_repair_transaction_receipts`).

It builds on the explainability packet
(`artifacts/doctor/m5/project-doctor-explainability-parity.json`): that packet
pins *whether and why a candidate repair is available*; this packet pins *what
happens when a repair runs* — the transaction it declares before mutating, the
stages it executes, and the reversibility receipt it produces.

## What a repair transaction declares before mutation

Every receipt records the full transaction declaration that exists **before any
mutation begins**, so a user can review exactly what will change and how it can
be undone:

- **Which repair?** A `repair_id` beginning with `repair.`, identified by a
  `receipt_id` beginning with `receipt:`.
- **Why?** One or more `initiating_findings` (each `doctor.finding.`-prefixed)
  and a `failure_family` (one of the eight M5 lanes: `notebook_kernel`,
  `request_api`, `database_connection`, `profiler_replay`, `preview_route`,
  `sync_offboarding`, `companion_handoff`, `incident_packet`).
- **What may change?** The `impacted_state_classes` and the `preconditions`
  checked first.
- **Where?** The disclosed `host_boundary` (`local_workspace`, `remote_host`,
  `container`, `devcontainer`, `tunnel`, or `managed_service`) and an opaque
  `boundary_scope_ref` naming the mount/port/tunnel/target scope.
- **What checkpoint exists?** A `checkpoint` disclosure with `present`, a
  `checkpoint_kind` (`transactional_snapshot`, `filesystem_snapshot`,
  `state_export`, or `none`), and an opaque `checkpoint_ref` (empty when absent).
- **How can it be undone?** A `reversal_class` of `reversible_transactional`,
  `reversible_with_snapshot`, `compensating_only`, or `irreversible_guarded`.
- **How will it be checked?** A `verification_plan`.

## Stages: review, dry run, checkpoint, apply, verify, rollback/compensate

The `stages` array records the executed flow in canonical order:

`review` → `dry_run` → `checkpoint` → `apply` → `verify` → (`rollback` |
`compensate`)

Each `StageRecord` carries the `stage`, its `status` (`passed`, `partial`,
`failed`, `skipped`, `inconclusive`), and a reviewer-safe `note`. Validation
enforces that stages appear in canonical order with no duplicates, that every
transaction begins with `review` and reaches `apply` and `verify`, that the
`checkpoint` stage is present **iff** a checkpoint was captured, and that
`rollback` and `compensate` (the two mutually exclusive terminal recovery
stages) never both appear.

## Completion receipts are never a generic success/failure

The terminal `completion_state` distinguishes six named outcomes instead of a
generic success/failure toast:

| completion state            | meaning                                                        |
| --------------------------- | -------------------------------------------------------------- |
| `fixed`                     | applied and verified cleanly                                   |
| `partially_repaired`        | applied to part of its scope (`apply` is `partial`)            |
| `reduced_but_not_resolved`  | applied but the finding persists in reduced form              |
| `verification_inconclusive` | verification ran but could not confirm the repair             |
| `rolled_back_exact`         | reversed exactly from a checkpoint                             |
| `rolled_back_compensating`  | reversed by compensating actions (no exact checkpoint)        |

Each receipt links back to its `initiating_findings`, its `checkpoint`, the
`affected_objects` it touched, the `verification_results` it collected, and the
`support_paths` it offers — so a receipt is a self-contained, inspectable proof
of what the repair did and how to recover.

## Guardrails

Two guardrails are enforced by construction and re-checked by
`ProjectDoctorRepairTransactionReceipts::validate()`:

- **No hidden reset or cache wipe of durable user state.** A transaction whose
  `mutates_durable_user_state` is true must either carry a checkpoint or be an
  explicitly `irreversible_guarded` repair that offers support/export paths. It
  can never silently wipe durable state.
- **No false promise of reversibility.** When no checkpoint exists, the receipt
  says so (`checkpoint.present` is false), its `reversal_class` may **not** be
  `reversible_transactional` or `reversible_with_snapshot`, it must offer
  `support_paths`, and it can never claim `rolled_back_exact` (an exact rollback
  requires a checkpoint). A reversal class that promises exact reversal must be
  backed by the matching `checkpoint_kind`.

There is no generic one-click "fix everything" path: every mutation is a bounded
repair transaction with its own declared receipt.

## One machine meaning across every surface

Each receipt records the `parity_surfaces` it renders on. Every receipt must
render on the four **core** surfaces — `desktop_receipt`, `cli_row`,
`headless_json`, and `support_export` — and may additionally render on
`incident_packet` and `public_truth`. The locale-invariant `machine_meaning_keys`
(`repair_id`, `failure_family`, `completion_state`, `reversal_class`) may never
change with localized prose; the `explanation` is additive only.

## Read-only and metadata-only

A receipt is metadata about a repair: every field is a typed state or an opaque
ref. Each receipt sets `redaction_class: metadata_safe_default` and
`raw_private_material_excluded: true`, and carries no credential bodies, raw
provider payloads, or mount/port/tunnel secrets.

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with the receipt id,
repair id, failure family, initiating findings, host boundary and scope,
checkpoint presence/kind/ref, reversal class, durable-state flag, completion
state, partial-success flag, affected objects, support paths, a
cross-surface-stable flag, and the explanation, plus `with_checkpoint_count`,
`without_checkpoint_count`, `rolled_back_count`, and
`cross_surface_stable_count`. Help/About, docs/help, support exports, incident
packets, and release/public-truth packets should ingest this projection directly
rather than restating receipt text by hand.

## Validation

`ProjectDoctorRepairTransactionReceipts::validate()` reports every violation,
including unsupported schema version or record kind, non-canonical closed
vocabularies, empty required fields or lists, a duplicate receipt id, a receipt
id or repair id without its prefix, a missing or wrongly prefixed initiating
finding, a receipt offering no support/export path, an inconsistent checkpoint
disclosure, a reversal-class/checkpoint-kind disagreement, a missing required
stage, a duplicate or out-of-order stage, both rollback and compensate stages, a
checkpoint-stage/disclosure mismatch, a completion state that disagrees with its
stages and results, an unguarded durable mutation, a no-checkpoint receipt that
claims clean/snapshot reversibility or an exact rollback or offers no support
path, a receipt that is not stable across the core surfaces, a missing
locale-invariant machine-meaning key, a non-metadata-safe redaction class, a
generic/empty verification check id, and a summary block that disagrees with the
receipts.
