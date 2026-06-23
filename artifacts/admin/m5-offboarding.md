# Offboarding — evidence companion

Human-readable companion to
[`/fixtures/admin/m5-offboarding/canonical_offboarding.json`](../../fixtures/admin/m5-offboarding/canonical_offboarding.json)
and its boundary schema
[`/schemas/admin/m5-offboarding.schema.json`](../../schemas/admin/m5-offboarding.schema.json).
It gives reviewers the rendered per-profile offboarding wizards without reading
the JSON. The contract narrative lives in
[`/docs/admin/m5-offboarding.md`](../../docs/admin/m5-offboarding.md), and the
frozen object model it binds back to lives in
[`/artifacts/admin/m5-admin-plane.md`](./m5-admin-plane.md).

- Bundle id: `m5-offboarding:bundle:0001`
- Record kind: `m5_offboarding_bundle`
- Binds matrix: `m5-admin-plane:matrix:0001`
- Profiles: 4 · Checkpoints: 24 · Triggers: 9 · Invariants: 19

## Profiles and coverage

| Profile | Deployment | Coverage state | Completeness | Locally inspectable | Console-independent | Seat-free |
| --- | --- | --- | --- | --- | --- | --- |
| `managed_cloud` | managed_cloud | active_enforced | complete | yes | yes | yes |
| `self_hosted` | self_hosted | active_enforced | complete | yes | yes | yes |
| `sovereign_air_gapped` | sovereign_air_gapped | boundary_changed_recheck_required | partial_imported | yes | yes | yes |
| `mirrored_offline` | managed_cloud | mirror_offline_last_known | partial_offline | yes | yes | yes |

Every profile keeps a locally inspectable wizard with no vendor console and is
completable without a still-active paid seat. The sovereign profile's
boundary-pending view and the mirrored profile's offline view are labeled with a
non-complete completeness class and a non-`active_enforced` coverage state rather
than presented as a confirmed-complete flow.

## Triggers (impact in plain language)

| Profile | Triggers |
| --- | --- |
| `managed_cloud` | seat_loss · subscription_cancellation · plan_downgrade |
| `self_hosted` | deprovision · org_switch |
| `sovereign_air_gapped` | deprovision · seat_loss |
| `mirrored_offline` | subscription_cancellation · org_switch |

All five trigger classes (seat loss, cancellation, deprovision, org switch, plan
downgrade) appear across the bundle, and each names the impacted managed features,
export rights, local-safe continuation, and managed copies remaining. None gates
user-owned recovery on an active paid seat.

## Ordered checkpoints (kind · scope · outcome · state · confirm · managed copies · recovery)

| Profile | # | Kind | Scope | Outcome | State | Confirm | Managed copies | Recovery |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `managed_cloud` | 1 | review_artifacts | personal | completed | active_enforced | — | none_remaining | — |
| `managed_cloud` | 2 | export | personal | available_now | export_available_now | — | none_remaining | — |
| `managed_cloud` | 3 | transfer | workspace | completed | active_enforced | — | transferred_to_owner | — |
| `managed_cloud` | 4 | confirm | org | available_now | active_enforced | yes | pending_scheduled_delete | — |
| `managed_cloud` | 5 | delete | personal | completed | delete_receipted | yes | deleted_with_receipt | — |
| `managed_cloud` | 6 | local_continuation | personal | available_now | export_available_now | — | none_remaining | — |
| `self_hosted` | 1 | review_artifacts | team | completed | active_enforced | — | none_remaining | — |
| `self_hosted` | 2 | export | team | available_now | export_available_now | — | pending_scheduled_delete | — |
| `self_hosted` | 3 | transfer | org | completed | active_enforced | — | transferred_to_owner | — |
| `self_hosted` | 4 | confirm | org | available_now | active_enforced | yes | pending_scheduled_delete | — |
| `self_hosted` | 5 | delete | org | blocked | delete_blocked_by_hold | yes | retained_under_hold | yes |
| `self_hosted` | 6 | local_continuation | team | available_now | export_available_now | — | none_remaining | — |
| `sovereign_air_gapped` | 1 | review_artifacts | personal | completed | active_enforced | — | none_remaining | — |
| `sovereign_air_gapped` | 2 | export | personal | available_now | export_available_now | — | none_remaining | — |
| `sovereign_air_gapped` | 3 | transfer | workspace | failed_recoverable | boundary_changed_recheck_required | — | transferred_to_owner | yes |
| `sovereign_air_gapped` | 4 | confirm | org | available_now | active_enforced | yes | retained_under_hold | — |
| `sovereign_air_gapped` | 5 | delete | org | blocked | delete_blocked_by_hold | yes | retained_under_hold | yes |
| `sovereign_air_gapped` | 6 | local_continuation | personal | available_now | export_available_now | — | none_remaining | — |
| `mirrored_offline` | 1 | review_artifacts | personal | completed | active_enforced | — | none_remaining | — |
| `mirrored_offline` | 2 | export | personal | available_now | export_available_now | — | none_remaining | — |
| `mirrored_offline` | 3 | transfer | team | failed_recoverable | mirror_offline_last_known | — | retained_upstream_mirror | yes |
| `mirrored_offline` | 4 | confirm | org | available_now | active_enforced | yes | retained_upstream_mirror | — |
| `mirrored_offline` | 5 | delete | workspace | deferred | delete_pending | yes | retained_upstream_mirror | — |
| `mirrored_offline` | 6 | local_continuation | personal | available_now | export_available_now | — | none_remaining | — |

Every profile renders all six checkpoint kinds in order. All four scopes
(personal, workspace, team, org), all five outcomes (completed, available-now,
deferred, blocked, failed-recoverable), and all six managed-copies dispositions
(none-remaining, deleted-with-receipt, pending-scheduled-delete,
retained-under-hold, retained-upstream-mirror, transferred-to-owner) are
exercised. Every delete is confirmation-gated; the two stale failed transfers and
the deferred delete sit under non-confirmed states, never confirmed-green.

## Failed / blocked flows are recoverable

| Checkpoint | Outcome | Diagnostic | Restore checkpoint | Next-step owner |
| --- | --- | --- | --- | --- |
| `self_hosted` delete | blocked | delete_blocked_by_hold | `restore.self_hosted.delete_predelete_01` | security_owner |
| `sovereign` transfer | failed_recoverable | boundary_recheck_required | `restore.sovereign.transfer_pretransfer_01` | org_admin |
| `sovereign` delete | blocked | delete_blocked_by_hold | `restore.sovereign.delete_predelete_01` | compliance_owner |
| `mirrored` transfer | failed_recoverable | transfer_recipient_unavailable | `restore.mirrored.transfer_pretransfer_01` | org_admin |

Each blocked or failed checkpoint retains a restore checkpoint, a *typed*
diagnostic (never a generic sign-in or billing error), and next-step guidance with
the restore / retained-diagnostics / next-step / resume affordances, so the flow
is repaired from the saved checkpoint rather than restarted from zero.

## Deletion schedules

| Checkpoint | Outcome | When | What remains |
| --- | --- | --- | --- |
| `managed_cloud` delete | immediate | now | — |
| `self_hosted` delete | blocked | when the security owner releases the regulatory hold | the operational audit history |
| `sovereign` delete | blocked | when the offline hold seal is lifted by the compliance owner | the sealed evidence packet |
| `mirrored` delete | deferred | when the mirror reconnects to the control plane | the upstream managed copy |

All three delete outcomes (immediate, deferred, blocked) appear; every
non-immediate delete names what remains and when it completes.

## Local-only continuation rights

Every profile guarantees all four continuation rights — `export_user_owned_artifacts`,
`continue_local_only`, `edit_local_artifacts`, `publish_later` — each available
offline and free of a paid seat, and renders a `local_continuation` checkpoint.

## Export parity and cross-surface

Each wizard offers both a `machine_readable_json` summary export and a
`plain_language_handoff` packet, and every checkpoint carries both a machine
summary and a plain-language sentence. There is exactly one typed packet per
profile, consumed identically by the shell admin center, CLI/headless inspect,
Help/About, support export, and procurement surfaces.

## Invariants (all hold)

| Invariant | Statement |
| --- | --- |
| `offboarding.surface_states_within_matrix` | Every rendered state is one the frozen matrix admits for the offboarding surface. |
| `offboarding.checkpoints_ordered_and_complete` | Every profile renders one checkpoint per kind in ascending order; ids are unique. |
| `offboarding.no_paid_seat_required` | No checkpoint, trigger, or coverage view requires a still-active paid seat to recover user-owned data. |
| `offboarding.triggers_explain_impact` | Every trigger explains impacted features, export rights, local continuation, and managed copies; every class appears. |
| `offboarding.scopes_distinguished` | Personal, workspace, team, and org scopes all appear. |
| `offboarding.confirmation_gates_deletes` | Every profile has an explicit confirm checkpoint and every delete is confirmation-gated. |
| `offboarding.managed_copies_honest` | Every checkpoint states its managed-copies disposition; a remaining copy names what/where/when/who. |
| `offboarding.failed_flows_recoverable` | Every blocked or failed checkpoint retains a restore checkpoint, typed diagnostics, and next-step guidance. |
| `offboarding.deletion_schedule_present` | Every delete carries a schedule; non-immediate names a remainder; all three outcomes appear. |
| `offboarding.transfer_named` | Every transfer names the owner ownership moves to. |
| `offboarding.local_continuation_guaranteed` | Every profile guarantees all four offline, seat-free continuation rights and a local-continuation checkpoint. |
| `offboarding.export_parity` | Every checkpoint carries both export representations and every wizard offers both export forms. |
| `offboarding.no_silent_green` | Stale evidence never sits under a confirmed active/export-available/receipted state. |
| `offboarding.locally_inspectable_offline` | Every profile keeps a locally inspectable, console-independent, seat-free wizard. |
| `offboarding.coverage_labeled` | A partial flow view is labeled, never implied complete. |
| `offboarding.consumer_parity` | One typed packet serves every consumer the matrix declares for this surface identically. |
| `offboarding.profiles_covered` | The managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline profiles are all rendered. |
| `offboarding.outcomes_all_present` | Every checkpoint outcome and every managed-copies disposition appears. |
| `offboarding.export_safe` | Every stable id is an opaque token and every governing schema is a repo-relative ref. |

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_offboarding > \
  fixtures/admin/m5-offboarding/canonical_offboarding.json

# Freeze gate: in-code bundle must equal the checked-in fixture
cargo test -p aureline-policy --test m5_offboarding

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_offboarding -- --lines
```
