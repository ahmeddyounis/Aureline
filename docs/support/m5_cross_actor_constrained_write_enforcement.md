# M5 Cross-Actor Constrained-Write Enforcement: One Gate Across Actors

This lane ships the B150 shared constrained-write gate: every mutation-capable actor — a direct edit / save, an
AI apply, an automation recipe, an importer, a repair, and a code action — is routed through **one** gate and
**one** safe-next-step resolver, so no actor gets a hidden bypass around the constrained-state rules. It is the
actor-parity mutation gate over the frozen constrained-file-state matrix
(`schemas/program/m5-constrained-file-state.schema.json` and siblings) and reuses the safe-next-step and
recovery-class vocabularies from the write-review-sheet lane.

- **Schema:** `schemas/program/m5-cross-actor-constrained-write-enforcement.schema.json`
- **Support export:** `artifacts/support/m5-cross-actor-constrained-write-enforcement/support_export.json`
- **Matrix CSV:** `artifacts/support/m5-cross-actor-constrained-write-enforcement/matrix.csv`
- **Summary:** `artifacts/support/m5-cross-actor-constrained-write-enforcement/summary.md`
- **Fixtures:** `fixtures/editor/m5-cross-actor-constrained-write-enforcement/`
- **Record kind:** `m5_cross_actor_constrained_write_enforcement_registry`

## The blocked-write reason is keyed to the state class, never the actor

The gate resolves a blocked-write reason and a safe next step as a **pure function of the constrained-object
class**. An AI apply, a repair, an importer, and a direct save that all land on the same object hit the same
structured reason and the same safe next step — the reason vocabulary is actor-independent.

| Object class | Blocked-write reason | Safe next step | Write disposition |
| --- | --- | --- | --- |
| `read_only` | `read_only_path_not_directly_writable` | `duplicate_to_editable_copy` | `read_only_blocked` |
| `generated` | `generated_artifact_regenerate_only` | `regenerate_with_preview` | `regenerate_only` |
| `policy_locked` | `policy_lock_requires_approval` | `request_approval` | `approval_gated` |
| `managed` | `managed_source_requires_detach` | `detach_from_managed_source` | `detach_required` |
| `projection` | `projection_requires_overlay_or_detach` | `create_overlay_patch` | `detach_required` |
| `captured_snapshot` | `captured_snapshot_restore_only` | `duplicate_to_editable_copy` | `read_only_blocked` |

## What every gate binding shows

Each binding carries one `resolution` (identical for a profile across every actor routed against it): the
state-class blocked reason, the write-constrained disposition, the safe next step, the recovery / undo class, the
exact write target, the canonical source, an export-safe explanation, and the labels for any co-applicable state
classes. It also carries an `ActorGateTrace` preserving the routed actor, the blocked reason, and the chosen
fallback path.

## No bypass write

The `direct_edit_save` actor is the only one that goes through direct typing; `ai_apply`, `automation_recipe`,
`importer`, `repair`, and `code_action` all bypass direct typing and are routed through the same gate. There is
no direct-write action to represent — the only write-adjacent action, `open_safe_next_step_review`, opens the
reviewed transition — so a mutation-capable actor can never silently write a generated, managed, projection, or
captured-snapshot object just because it did not go through the editor.

## Postures and narrowing

- `enforced_gate` — the gate resolves the blocked reason and offers `open_safe_next_step_review`.
- `fail_closed_on_actor_drift` — the gate **fails closed** because the actor context drifted or the exact write
  target could not be explained truthfully; it names a `fail_closed_reason`, offers no write path, and carries an
  explicit narrow note pointing to `resolve_actor_context_then_retry`.
- `export_redacted` — an export-safe rendering in the support packet; carries a narrow note plus an
  export-detail note and points at the canonical contracts.

Narrowing changes only which actions remain; it never rewords the state-class reason.

## Invariants

- **AC1** — At least one object (`managed/captured-snapshot-mirror`) is routed by an AI, a repair, an importer,
  and a direct-save actor, all resolving to the same blocked reason and safe next step.
- **AC2** — No mutation-capable actor can silently write a generated, managed, projection, or captured-snapshot
  object by bypassing direct typing; the action set is closed and safe (no direct-write variant) and every actor
  is routed through the shared gate.
- **AC3 (fail closed)** — The gate fails closed when the actor context drifts or the write target cannot be
  explained truthfully.
- **AC3 (trace)** — Every support / export trace preserves the actor, the blocked reason, and the chosen fallback
  path.

Guardrails (each must be `false` on every binding): an actor silently writing a constrained object by bypassing
direct typing, a hidden bypass for AI / automation / import / repair, an actor-specific free-form reason instead
of the state-class vocabulary, an unstated exact write target or canonical source, and one state class hiding
another.

Raw secret values, credentials, and private endpoints stay outside the support boundary; the packet references
upstream contracts by id.
