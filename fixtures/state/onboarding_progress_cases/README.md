# Portable onboarding progress, dismissals, and imported-profile history state fixtures

Seed corpus for the contract frozen in
[`/docs/state/onboarding_state_contract.md`](../../../docs/state/onboarding_state_contract.md)
and the schema at
[`/schemas/state/onboarding_progress.schema.json`](../../../schemas/state/onboarding_progress.schema.json).

Each file is a single JSON document validating against one of the
seven record kinds in the schema:

- `onboarding_progress_entry_record`
- `onboarding_dismissal_record`
- `first_useful_work_milestone_record`
- `imported_profile_history_entry_record`
- `rollback_reminder_record`
- `remembered_compatibility_notice_record`
- `onboarding_state_bundle_record`

Every fixture:

- Resolves every axis to vocabulary either re-exported from the
  portable-profile contract, the onboarding-portability state
  contract, the migration-and-restore playbook, or introduced by
  this contract in §3.
- Pins the `storage_lane`, `state_authority_class`,
  `state_portability_class`, `reset_class`, `export_class`,
  `profile_scope_class`, `delete_posture`, `hold_posture`, and
  `support_export_posture` values this contract owns.
- Carries no raw absolute paths, raw URLs, raw credential material,
  raw secrets, raw operator identifiers, or raw cryptographic
  material. Every id is an opaque ref; every timestamp is a
  monotonic placeholder.
- Names the contract sections it exercises under
  `__fixture__.contract_sections`.

## Cases

| Fixture | Record kind | Scenario axis | Contract anchor |
| --- | --- | --- | --- |
| [`account_free_tour_progress_portable.json`](./account_free_tour_progress_portable.json) | `onboarding_progress_entry_record` | Account-free local use: tour step 3 completed, rides portable profile body. | §4.1, §5.1, §7 (`state_item.tour_progress`) |
| [`welcome_banner_dismissal_portable.json`](./welcome_banner_dismissal_portable.json) | `onboarding_dismissal_record` | Account-free local use: welcome banner acknowledged once, rides portable profile body. | §4.2, §7 (`state_item.dismissal.welcome_banner`) |
| [`session_only_restore_card_dismissal.json`](./session_only_restore_card_dismissal.json) | `onboarding_dismissal_record` | Session-only dismissal: restore card dismissed for this session, ephemeral lane. | §4.2, §5.3, §6.2 (`ephemeral_session_never_exports_bodies`, `dismissal_scope_matches_lane`) |
| [`first_useful_edit_milestone_device_scoped.json`](./first_useful_edit_milestone_device_scoped.json) | `first_useful_work_milestone_record` | Device switch / portable restoration: first useful edit observed, machine-local state only. | §4.3, §5.2, §6.2 (`recent_work_and_milestones_are_device_scoped_by_default`), §7 (`state_item.first_useful_work_milestone`) |
| [`migrated_profile_history_entry_compatible.json`](./migrated_profile_history_entry_compatible.json) | `imported_profile_history_entry_record` | Migrated profile: competitor-config import under compatible fidelity, rollback window open. Rides portable profile body (redacted). | §4.4, §5.1, §7 (`state_item.imported_profile_history`), §9.1 |
| [`rollback_reminder_armed_device_scoped.json`](./rollback_reminder_armed_device_scoped.json) | `rollback_reminder_record` | Rollback reminder paired with the competitor-config import. Lives on machine-local state; window close horizon declared. | §4.5, §6.2 (`every_rollback_reminder_has_close_or_resolution`), §7 (`state_item.rollback_reminder`) |
| [`remembered_compatibility_notice_portable.json`](./remembered_compatibility_notice_portable.json) | `remembered_compatibility_notice_record` | Schema translation hint acknowledged; rides portable profile body so acknowledgement suppresses re-display on other devices. | §4.6, §6.2 (`remembered_compatibility_notice_never_binds_live_authority`), §7 (`state_item.remembered_compatibility_notice`) |
| [`managed_policy_consent_acknowledgement.json`](./managed_policy_consent_acknowledgement.json) | `onboarding_progress_entry_record` | Managed policy overlay: forced consent-acknowledgement task completed under policy bundle. Clear denied locally. | §3.2 (`clear_denied_admin_owned`), §4.1, §6.2 (`admin_owned_rows_refuse_local_clear`), §7 (`state_item.consent_acknowledgement_onboarding`) |
| [`onboarding_state_bundle.json`](./onboarding_state_bundle.json) | `onboarding_state_bundle_record` | Top-level aggregator binding every fixture above and declaring all twelve const-true invariants. | §6, §6.2 |

## Schema references

- Onboarding-progress state schema:
  [`/schemas/state/onboarding_progress.schema.json`](../../../schemas/state/onboarding_progress.schema.json).
- Onboarding-portability vocabulary (upstream, re-exported):
  [`/schemas/ux/onboarding_portability_state.schema.json`](../../../schemas/ux/onboarding_portability_state.schema.json).
- Portable-profile / state-map vocabulary (upstream, re-exported):
  [`/schemas/profile/portable_profile.schema.json`](../../../schemas/profile/portable_profile.schema.json).
- Shared restore / migration provenance (re-exported fidelity labels):
  [`/schemas/state/restore_provenance.schema.json`](../../../schemas/state/restore_provenance.schema.json).

## Companion corpora

- Onboarding-portability entry-surface rows and state items:
  [`/fixtures/ux/entry_surface_rows/`](../../ux/entry_surface_rows/).
- Migration-and-restore cases:
  [`/fixtures/state/migration_cases/`](../migration_cases/).
- Migration-and-corruption cases:
  [`/fixtures/state/migration_and_corruption_cases/`](../migration_and_corruption_cases/).
