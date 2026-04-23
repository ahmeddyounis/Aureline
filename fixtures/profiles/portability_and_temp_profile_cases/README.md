# Portability and temporary-profile verification cases

These fixtures are short, reviewable scenarios that anchor the
`profile_portability_state` and `temporary_profile_lifecycle_state`
vocabularies frozen in
[`/docs/verification/migration_and_profile_packet.md`](../../../docs/verification/migration_and_profile_packet.md).

Each fixture is one `migration_profile_portability_case_record`
rendered as a worked profile-library / temporary-session row. The
set exists so reviewers can diff a closed portability token, a
closed temporary-profile lifecycle token, and a state-map posture
claim without reverse-engineering it from a profile artifact body.

## Scope rules

- Fixtures reuse the frozen portable-profile, state-map,
  restore-provenance, and migration-center vocabularies; they do
  not redefine exclusion-reason ids, redaction classes, or
  authority classes.
- A new fixture MUST exercise at least one
  `profile_portability_state` token, one
  `temporary_profile_lifecycle_state` token (when applicable), or
  one sync / support-recovery posture from the packet, and MUST
  cite the motivating section.
- Opaque ids and monotonic timestamps are chosen for review clarity
  rather than to mirror a real machine.
- `non_portable_live_authority` handles never restore as live
  authority in any fixture; secret / trust-approval / admin-policy
  rows remain excluded from every portable body.

## Index

| Fixture | Portability state | Lifecycle state | Key coverage |
|---|---|---|---|
| [`portable_profile_plain_cross_machine.json`](./portable_profile_plain_cross_machine.json) | `portable_plain` | n/a | Baseline portable profile; no secrets, no trust approvals, no admin bundle; imports across machines. |
| [`portable_profile_with_machine_addendum.json`](./portable_profile_with_machine_addendum.json) | `portable_with_machine_addendum` | n/a | Portable body plus machine-local addendum for `machine_specific_settings`; addendum is `local_only` on the companion machine. |
| [`managed_sync_profile_roaming.json`](./managed_sync_profile_roaming.json) | `managed_sync_opt_in` | n/a | Managed-sync lane carries the body; rows whose state-map `sync_posture = never_synced` stay excluded. |
| [`temporary_profile_ephemeral_session.json`](./temporary_profile_ephemeral_session.json) | `local_only_unpublishable` | `ephemeral_in_memory_only` | Temporary / ephemeral session; writes scratch-only; no durable promotion without explicit save. |
| [`support_recovery_profile_snapshot.json`](./support_recovery_profile_snapshot.json) | `support_recovery_only` | n/a | Support-recovery manifest with redacted excerpts and typed exclusion reasons; routes through migration-center review. |

## Coverage contract

The shared fixture set MUST keep:

- at least one case for every `profile_portability_state` token
  referenced from first-run import, profile-library, managed-sync,
  or support-export surfaces;
- at least one case that exercises
  `temporary_profile_lifecycle_state = ephemeral_in_memory_only`
  with no durable write;
- at least one case whose addendum is `local_only` and whose
  portable body stays divorced from the addendum's rows;
- at least one case that carries the managed-sync `sync_posture`
  resolution rule explicitly;
- at least one case that clearly quotes the support-recovery
  redaction floor.
