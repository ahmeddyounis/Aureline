# M5 offboarding continuity plan (human-readable)

Companion to the boundary schema at
[`/schemas/storage/m5_offboarding_continuity.schema.json`](../../schemas/storage/m5_offboarding_continuity.schema.json),
the contract at
[`/docs/storage/m5_offboarding_continuity_contract.md`](../../docs/storage/m5_offboarding_continuity_contract.md),
and the scenario corpus under
[`/fixtures/storage/m5_offboarding_continuity_cases/`](../../fixtures/storage/m5_offboarding_continuity_cases/).

An offboarding continuity plan is the operator-facing object the shell shows
before an account offboarding, device reset, workspace wipe, or sign-out cleanup
removes anything. It distinguishes exportable durable state from non-portable
derived data, names the offline / mirror / certified-workspace continuity each
removal would break, and protects evidence, user-owned recovery state, and
offline / certified / policy-pinned packs unless they are explicitly reviewed
away. Every storage-class, family, authority, and pin-source value re-exports
verbatim from
[`/artifacts/storage/m5_artifact_family_storage_matrix.yaml`](m5_artifact_family_storage_matrix.yaml)
and [`/artifacts/runtime/storage_classes.yaml`](../runtime/storage_classes.yaml);
the offboarding-flow, portability, continuity-warning, disposition, and
portability-honesty columns are the only sets this plan introduces.

## What a plan states

- **Flow + initiator** — account offboarding, device reset, workspace wipe, or
  sign-out cleanup, and who initiated it.
- **Per-family portability** — `exportable_durable_state`,
  `captured_evidence_export_to_retain`,
  `rebuildable_from_pinned_or_offline_source`, or `non_portable_derived_cache`.
- **Continuity warnings** — what removal breaks: offline readiness, mirror
  continuity, certified-workspace readiness, policy-bundle continuity, evidence
  continuity, or recovery-state continuity.
- **Disposition** — `dispose_rebuildable`, `export_then_dispose`,
  `retained_protected_continuity`, `retained_for_offline_continuity`, or
  `retained_not_selected`.
- **Export-before-delete** — required on protected classes, offered on
  continuity-pinned packs, not applicable on pure caches.
- **Portability headline** — the plan-level honesty statement that never implies
  the user exported everything when only caches were cleared.
- **Open inspector / open clear-data review** — the actions that move the user
  from the plan into the storage inspector and the class-selective review (never
  a generic delete-all).

## Disposition per class and review state

| Storage class | Reviewed away? | Disposition | Export |
| --- | --- | --- | --- |
| interactive_hot_cache / knowledge_cache | n/a | dispose_rebuildable | none (disposable) |
| artifact_cache / prebuild (no continuity pin) | n/a | dispose_rebuildable | none (disposable) |
| artifact_cache / prebuild (offline/certified/release/policy pin) | no | retained_for_offline_continuity | **offered** |
| artifact_cache / prebuild (offline/certified/release/policy pin) | yes | export_then_dispose | **offered** |
| evidence_support_cache | no | retained_protected_continuity | **required** |
| evidence_support_cache | yes | export_then_dispose | **required** |
| user_owned_recovery_state | no | retained_protected_continuity | **required** |
| user_owned_recovery_state | yes | export_then_dispose | **required** |

A protected or continuity-pinned family requested for removal *without* an
explicit review stays retained, and the plan records a guardrail notice saying so.

## Portability honesty

- `nothing_disposed_all_retained` — no family was removed; everything is retained.
- `caches_only_removed_durable_retained` — only rebuildable / non-portable derived
  data was removed; durable state and captured evidence are unchanged (even if an
  offline/certified continuity warning fired, nothing portable was lost).
- `durable_state_exported_before_removal` — some exportable durable state or
  captured evidence was explicitly reviewed away, each exported before removal.

## Scenarios covered

`account_offboarding_durable_retained`, `device_reset_caches_only`,
`offline_certified_policy_pins_retained`,
`offline_bundle_reviewed_away_continuity_warned`, and
`workspace_wipe_reviewed_away_export_first`.

## Guardrails enforced by the plan

- Evidence, user-owned recovery state, and offline / certified / policy / mirror
  pins are never silently disposed; removal needs an explicit, exported review.
- Protected classes always require export-before-delete in either bucket.
- The portability headline never implies full data portability when only caches
  were removed.
- Offline, mirror, and certified-workspace continuity stays visible before any
  deletion; accepting a loss surfaces a named guardrail notice rather than hiding
  it in logs-only diagnostics.
- Storage pressure has no path here: disposal is operator-driven and explicit, so
  managed quota or low disk can never satisfy itself by wiping user-owned state
  through an offboarding side effect.
