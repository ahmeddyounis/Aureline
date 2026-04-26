# System affordance cases

Worked fixtures for the contract frozen in
[`/docs/ux/desktop_affordance_contract.md`](../../../docs/ux/desktop_affordance_contract.md)
and the schema at
[`/schemas/platform/deep_link_intent.schema.json`](../../../schemas/platform/deep_link_intent.schema.json).

Each file is either a `deep_link_intent_record` or a
`system_affordance_case_record`. The cases exercise OS-facing entry,
deep-link review, notification click-through, badge / presence lineage,
native dialog routing, system share and copy affordances,
open-from-terminal, and desktop lifecycle recovery.

Every fixture:

- cites a canonical command id and object identity, plus event lineage
  where the affordance projects a notification or badge;
- declares route class, source surface, handler ownership, trust /
  policy result, replay posture, and fallback;
- keeps lock-screen and support-export payloads privacy-safe;
- preserves source intent when exact execution is denied or degraded;
- forbids silent rerun, hidden focus stealing, destructive cleanup, and
  authority widening from OS-level surfaces.

## Cases

| Fixture | Record kind | What it exercises |
|---|---|---|
| [`file_association_workspace_review.json`](./file_association_workspace_review.json) | `system_affordance_case_record` | workspace file association opens through a review sheet because handler ownership and trust posture must stay visible |
| [`deep_link_remote_review_replay_denied.json`](./deep_link_remote_review_replay_denied.json) | `deep_link_intent_record` | consumed remote review deep link denies replay and preserves recovery actions |
| [`notification_clickthrough_lock_screen_privacy.json`](./notification_clickthrough_lock_screen_privacy.json) | `system_affordance_case_record` | OS notification click-through resolves to the exact durable event while lock-screen copy stays generic |
| [`badge_presence_traceable_count.json`](./badge_presence_traceable_count.json) | `system_affordance_case_record` | badge and collaboration presence count derive from durable event lineage rather than mixed raw events |
| [`removable_volume_return_cached_context.json`](./removable_volume_return_cached_context.json) | `system_affordance_case_record` | removable root returns as stale until identity is reconciled, with cached context preserved |
| [`native_save_dialog_read_only_boundary.json`](./native_save_dialog_read_only_boundary.json) | `system_affordance_case_record` | native save routing preserves read-only, alias, remote/generated, and recovery truth |
| [`system_share_copy_permalink_review.json`](./system_share_copy_permalink_review.json) | `system_affordance_case_record` | system share and copy-permalink affordances require source identity, redaction, expiry, and policy review |
| [`open_from_terminal_wake_revalidation.json`](./open_from_terminal_wake_revalidation.json) | `system_affordance_case_record` | terminal-originated open preserves literal intent and wake-from-sleep requires revalidation before remote authority resumes |
