# Proof packet: M1 device-registry and settings-sync seed (conflict-review vocabulary)

Purpose: anchor proof captures for the unattended M1 lane that
validates the canonical device-registry and settings-sync seed. The
lane proves the seed is consumable by the settings UI, CLI
`settings sync`, the profile importer / exporter, the support-bundle
exporter, the mutation-journal renderer, the migration center, the
conflict review surface, and any later managed-sync service — without
re-encoding the device-participation, sync-session, conflict-review,
offered-resolution-path, resolution-state, scope-broadening verdict,
non-widening posture, local-ownership marker, or per-data-class
portability vocabularies on each surface.

Reviewer entry point:
[`/docs/settings/m1_sync_and_device_seed.md`](../../../docs/settings/m1_sync_and_device_seed.md).
Upstream optional-sync contract:
[`/docs/settings/sync_and_device_registry_seed.md`](../../../docs/settings/sync_and_device_registry_seed.md).

## Canonical sources

- `artifacts/settings/m1_device_registry_and_sync_seed.yaml` — seed
  rows the runner consumes. Carries:
  - the M1 envelope (`schema_version`, `matrix_id`, `owner_dri`,
    `overview_page`, `upstream_sync_contract_ref`,
    `upstream_settings_registry_ref`, `row_schema_ref`,
    `build_identity_ref`, `validation_lane_ref`),
  - closed envelope vocabularies for device participation, device
    class, os family, identity mode, sync session state,
    conflict-review class, conflict resolution-state class, offered
    resolution-path class, non-widening posture, scope-broadening
    verdict, local-ownership marker, data-portability class, and
    failure-drill id,
  - required coverage lists (participation states, session states,
    conflict-review classes, portability classes),
  - the named runtime consumers the seed asserts are live, and
  - one sync-state profile row per typed scenario with the uniform
    `(sync_state_profile_id, device_participation_state_class,
    device_class, os_family_class, identity_mode_class,
    sync_session_state_class, conflict_review_class,
    conflict_resolution_state_class,
    offered_resolution_path_classes, non_widening_posture_class,
    scope_broadening_verdict_class, local_ownership_marker_class,
    data_class_portabilities, last_successful_sync_state_class,
    rollback_checkpoint_required, approval_ticket_required,
    owner_dri, failure_drill)` shape.

- `schemas/settings/device_registry.schema.json` — envelope schema;
  freezes vocabularies, required coverage lists, named consumer
  shape, matrix identity, and pins the canonical landing-page path.

- `schemas/settings/settings_sync_state.schema.json` — row schema;
  freezes the closed device-participation, device-class,
  os-family-class, identity-mode, sync-session-state,
  conflict-review, conflict-resolution-state, offered-resolution-path,
  non-widening-posture, scope-broadening-verdict, local-ownership-
  marker, and data-portability vocabularies, plus the conditional
  invariants (revoked → refused / device_revoked; paused → degraded-
  or-paused / device_paused; scope_broadening_refusal → would-widen-
  refused; no_conflict / value_equal_no_op → not_applicable).

- `tests/governance/m1_device_registry_and_sync_seed_lane/run_m1_device_registry_and_sync_seed_lane.py`
  — unattended runner that replays the seed and emits the durable
  JSON capture.

## Upstream sources the seed projects against

- `docs/settings/sync_and_device_registry_seed.md` — upstream
  optional-sync and device-registry contract (wire-level packet
  shapes: device record, scope bundle, session envelope, conflict
  packet). The seed inherits the contract's vocabularies and projects
  them into the higher-level profile rows.
- `docs/settings/schema_registry_seed.md` — companion settings
  schema-registry seed (setting-definition / effective-setting /
  write-intent publishing contract) the seed cross-references.

## Named runtime consumers

- `docs/settings/m1_sync_and_device_seed.md` — reviewer-facing
  landing page that quotes the seeded rows verbatim so the settings
  UI, CLI `settings sync`, profile importer, support exporter, and
  migration center all read the same conflict-review vocabulary and
  portability model.
- `docs/settings/sync_and_device_registry_seed.md` — upstream
  optional-sync contract; the runner asserts the doc resolves on
  disk so the M1 seed cannot quietly outlive its upstream.
- `tests/governance/m1_device_registry_and_sync_seed_lane/run_m1_device_registry_and_sync_seed_lane.py`
  — live CI/review consumer (this lane) that replays the seed,
  asserts closed-vocabulary agreement with the row schema,
  structural invariants, required coverage, named-consumer
  resolution, and reproduces every named failure drill loudly.

## Live runtime consumers (read-only)

- `artifacts/build/build_identity.json` — exact-build identity that
  the capture embeds for cross-artifact traceability.

## Validation captures

- `artifacts/milestones/m1/captures/device_registry_and_sync_seed_validation_capture.json`

## Sync-state profile coverage

The seed asserts the following sync-state profiles are present as
typed rows:

| `sync_state_profile_id` | Participation | Session | Conflict | Local ownership |
| --- | --- | --- | --- | --- |
| `active.local_authoritative.no_conflict` | `active` | `open` | `no_conflict` | `local_authoritative` |
| `active.scope_broadening_refusal` | `active` | `open` | `scope_broadening_refusal` | `local_authoritative` |
| `active.rollback_friendly_review_pending` | `active` | `open` | `rollback_friendly_review_pending` | `local_authoritative` |
| `paused.local_authoritative_degraded` | `paused` | `local_authoritative_degraded` | `device_paused` | `local_authoritative` |
| `revoked.refused` | `revoked` | `refused` | `device_revoked` | `machine_local_excluded` |
| `manual_continuity.keep_synced_pending_review` | `active` | `manual_continuity_in_progress` | `keep_synced_pending_review` | `local_authoritative` |
| `stale_payload.local_authoritative_degraded` | `active` | `local_authoritative_degraded` | `stale_payload` | `local_authoritative` |

The union of every row's `data_class_portabilities` covers all four
portability classes (`portable`, `machine_local`, `excluded`,
`policy_owned`) so machine-local exclusions and policy-owned
ownership stay distinct from local-authoritative state.

## Failure-drill coverage

Seven named drills, all reproducible under
`--force-drill <sync_state_profile_id>:<drill_id>`:

| Row | Drill | Expected check id |
| --- | --- | --- |
| `active.local_authoritative.no_conflict` | `device_sync_state_drill.active_local_authoritative_non_widening_affirmation_dropped` | `settings_sync_state.non_widening_affirmation_missing_blocked_on_active_row` |
| `active.scope_broadening_refusal` | `device_sync_state_drill.active_scope_broadening_verdict_relaxed_to_widen_trust` | `settings_sync_state.scope_broadening_refusal_requires_would_widen_trust_refused_verdict` |
| `active.rollback_friendly_review_pending` | `device_sync_state_drill.active_keep_local_resolution_path_dropped` | `settings_sync_state.keep_local_resolution_path_required_for_active_conflict` |
| `paused.local_authoritative_degraded` | `device_sync_state_drill.paused_session_state_widened_to_open` | `settings_sync_state.paused_device_session_state_must_be_degraded_or_paused` |
| `revoked.refused` | `device_sync_state_drill.revoked_session_state_widened_to_open` | `settings_sync_state.revoked_device_session_state_must_be_refused` |
| `manual_continuity.keep_synced_pending_review` | `device_sync_state_drill.manual_continuity_data_class_drifted_from_excluded_to_portable` | `settings_sync_state.excluded_data_class_widening_blocked` |
| `stale_payload.local_authoritative_degraded` | `device_sync_state_drill.stale_payload_conflict_review_class_drifted_to_no_conflict` | `settings_sync_state.stale_payload_must_not_drift_to_no_conflict` |

## Refresh

Re-run the validation lane after a change to:

- the seed YAML,
- either schema (envelope or row),
- the reviewer-facing landing page,
- the upstream optional-sync contract or settings schema-registry
  seed the seed cross-references, or
- the build-identity record the capture embeds.

## Closure rule

The lane stays open until the latest capture lands under the governed
proof root and every row reports PASS for closed-vocabulary membership
(device_participation_state_class, device_class, os_family_class,
identity_mode_class, sync_session_state_class, conflict_review_class,
conflict_resolution_state_class, offered_resolution_path_class,
non_widening_posture_class, scope_broadening_verdict_class,
local_ownership_marker_class, data_portability_class), the conditional
invariants (revoked → refused / device_revoked; paused → degraded-or-
paused / device_paused; scope_broadening_refusal → would-widen-refused
+ keep_local path; no_conflict / value_equal_no_op → not_applicable;
active conflict rows always offer keep_local; active open sessions
never publish non_widening_affirmation_missing; excluded data classes
never widen to portable; stale_payload rows never drift to no_conflict),
the required coverage rules (participation states, session states,
conflict-review classes, portability classes), named-runtime-consumer
existence, and its seven named failure drills.
