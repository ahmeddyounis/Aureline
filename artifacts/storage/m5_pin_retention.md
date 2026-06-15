# M5 pin / retention manager and cleanup history (human-readable)

Companion to the boundary schema at
[`/schemas/storage/m5_pin_retention.schema.json`](../../schemas/storage/m5_pin_retention.schema.json),
the contract at
[`/docs/storage/m5_pin_retention_contract.md`](../../docs/storage/m5_pin_retention_contract.md),
and the scenario corpus under
[`/fixtures/storage/m5_pin_retention_cases/`](../../fixtures/storage/m5_pin_retention_cases/).

A pin / retention manager explains *why an artifact is still on disk*, and the
cleanup-history lane explains *what a past eviction did*. Every storage-class,
artifact-family, and pin-source value re-exports verbatim from
[`/artifacts/runtime/storage_classes.yaml`](../runtime/storage_classes.yaml) and
the frozen matrix at
[`/artifacts/storage/m5_artifact_family_storage_matrix.yaml`](m5_artifact_family_storage_matrix.yaml).
The pin-actor, retention-state, unpin-path, export-path, referenced-object, and
cleanup columns are the only sets this lane introduces.

## What a pin states

| Column | Meaning |
| --- | --- |
| pin source | The frozen `pin_source_class` that holds the artifact. |
| who pinned it | The `pin_actor` derived from the source (user, admin policy, release / case / review / offline / certified-template / support-export process, retention policy). |
| expiry / policy window | The `retention_state`, plus an `expires_at` for a finite window. |
| referenced object | The object whose reference keeps it on disk. |
| unpin path | How the pin is released. |
| export path | The export-before-delete path offered before any delete. |

## Unpin paths, by pin source

| Pin source | Unpin path |
| --- | --- |
| explicit_user_pin | user unpins directly |
| explicit_admin_policy_pin / policy_bundle_last_known_good_ref | admin / policy change required |
| release / case / review / offline / certified / support reference | release the referencing object |
| retention_window_ref | auto-unpins at retention expiry |

## What a cleanup event records

- **actor** — user, admin policy, system pressure governor, retention scheduler,
  or offboarding flow;
- **trigger** — low-disk pressure, managed-quota pressure, explicit clear-data,
  offboarding / reset, retention-window expiry, or case close;
- **class and family** — the storage class and artifact family it touched;
- **disposition** — trimmed disposable / rebuildable / unpinned, expired unpinned
  evidence past retention, exported-then-deleted, or a pin-blocked no-op;
- **reclaimed bytes** and **blocked pins** — what came back and what stopped it;
- **resulting state** — fully reclaimed, rebuild-pending, reindex-needed, partial
  (pins retained), or authoritative state untouched.

## Scenarios covered

- `evidence_and_checkpoint_pins` — crash evidence, a review packet, a
  support-export assembly, a replay bundle under a retention window, and a
  checkpoint, with a disposable trim and a pin-blocked evidence expiry.
- `offline_packs_and_certified_templates` — docs / model / template packs and
  pinned notebook outputs, with an unpinned-artifact trim that retains the pins
  and a rebuildable trim that needs a reindex.
- `cleanup_history_blocked_by_pins` — pressure blocked twice by an admin-policy
  hold and a retention-windowed checkpoint, plus an explicit exported-then-deleted
  checkpoint.
- `managed_quota_preserves_user_owned_state` — managed quota trims an unpinned
  model pack but refuses to touch user-owned recovery state.

## Guardrails enforced by the manager

- The pin actor, unpin path, and export path are all derived from the pin source
  and the frozen matrix; no surface invents a private mapping.
- Storage pressure never reclaims user-owned recovery bytes; the only cleanup
  that may delete recovery state is an explicit, exported-then-deleted user
  action.
- Pinned and in-window evidence is retained; only unpinned evidence past
  retention may expire.
- Blocked pins are always recorded, never hidden in logs-only diagnostics.
- No cleanup event ever touches authoritative state or captures a raw payload.
