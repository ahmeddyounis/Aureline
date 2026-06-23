# Decision-history — evidence companion

Human-readable companion to
[`/fixtures/admin/m5-decision-history/canonical_history.json`](../../fixtures/admin/m5-decision-history/canonical_history.json)
and its boundary schema
[`/schemas/admin/m5-decision-history.schema.json`](../../schemas/admin/m5-decision-history.schema.json).
It gives reviewers the rendered per-profile timelines without reading the JSON.
The contract narrative lives in
[`/docs/admin/m5-decision-history.md`](../../docs/admin/m5-decision-history.md),
and the frozen object model it binds back to lives in
[`/artifacts/admin/m5-admin-plane.md`](./m5-admin-plane.md).

- Bundle id: `m5-decision-history:bundle:0001`
- Record kind: `m5_decision_history_bundle`
- Binds matrix: `m5-admin-plane:matrix:0001`
- Profiles: 4 · Events: 13 · Invariants: 13

## Profiles and coverage

| Profile | Deployment | Coverage state | Completeness | Locally inspectable | Console-independent |
| --- | --- | --- | --- | --- | --- |
| `managed_cloud` | managed_cloud | active_enforced | complete | yes | yes |
| `self_hosted` | self_hosted | active_enforced | complete | yes | yes |
| `sovereign_air_gapped` | sovereign_air_gapped | imported_snapshot_no_live | partial_imported | yes | yes |
| `mirrored_offline` | managed_cloud | mirror_offline_last_known | partial_offline | yes | yes |

Every profile keeps a locally inspectable history with no vendor console. The
sovereign profile's imported coverage and the mirrored profile's offline coverage
are labeled with a non-complete completeness class rather than presented as a full
history.

## Decision events (decision code · actor class · outcome state)

| Profile | Event | Family | Decision code | Actor class | Affected target | Policy epoch | State | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `managed_cloud` | `…managed_cloud.0001` | policy_change | force_disable | admin_action | `ai.provider.allowed` | `policy_epoch.managed.rev42` | active_enforced | fresh |
| `managed_cloud` | `…managed_cloud.0002` | provider_routing | narrow | policy_evaluation | `ai.route.default` | `policy_epoch.managed.rev42` | active_enforced | fresh |
| `managed_cloud` | `…managed_cloud.0003` | remote_mutation | mutation_recorded | user_action | `remote.session.fs.write` | `policy_epoch.managed.rev42` | active_enforced | fresh |
| `managed_cloud` | `…managed_cloud.0004` | auth_session | allow | policy_evaluation | `auth.session.start` | `policy_epoch.managed.rev42` | active_enforced | fresh |
| `self_hosted` | `…self_hosted.0001` | policy_change | force_disable | admin_action | `network.egress` | `policy_epoch.self_hosted.rev7` | active_enforced | fresh |
| `self_hosted` | `…self_hosted.0002` | trust_change | allow | admin_action | `trust.root.customer` | `policy_epoch.self_hosted.rev7` | active_enforced | recent |
| `self_hosted` | `…self_hosted.0003` | provider_routing | deny | provider_limitation | `ai.provider.external` | `policy_epoch.self_hosted.rev7` | unconfirmed_stale | stale |
| `sovereign_air_gapped` | `…sovereign.0001` | policy_change | force_disable | admin_action | `ai.provider.allowed` | `policy_epoch.offline.seal_a1` | active_enforced | recent |
| `sovereign_air_gapped` | `…sovereign.0002` | managed_identity_scope | local_only_continue | policy_evaluation | `identity.managed.scope` | `policy_epoch.offline.seal_a1` | imported_snapshot_no_live | stale |
| `sovereign_air_gapped` | `…sovereign.0003` | collaboration_control | force_disable | client_limitation | `collab.share.external` | `policy_epoch.offline.seal_a1` | active_enforced | fresh |
| `mirrored_offline` | `…mirrored.0001` | policy_change | force_disable | admin_action | `ai.provider.allowed` | `policy_epoch.mirror.rev42` | mirror_offline_last_known | stale |
| `mirrored_offline` | `…mirrored.0002` | publish_state | request_recorded | user_action | `marketplace.publish` | `policy_epoch.mirror.rev42` | mirror_offline_last_known | recent |
| `mirrored_offline` | `…mirrored.0003` | auth_session | defer_pending_refresh | client_limitation | `auth.session.refresh` | `policy_epoch.mirror.rev42` | unconfirmed_stale | stale |

The self-hosted `ai.provider.external` row is a **provider limitation** (the
external provider was unreachable), not a policy denial; the sovereign
`collab.share.external` row is a **client limitation** (the air-gapped client
cannot reach a sharing service). Both are surfaced as themselves rather than as
generic blocked/error events. The three stale rows are shown under non-confirmed
states (`unconfirmed_stale`, `imported_snapshot_no_live`,
`mirror_offline_last_known`), never as confirmed-green.

## Explorer filters (all eight families offered per profile)

`trust_change`, `policy_change`, `auth_session`, `remote_mutation`,
`provider_routing`, `collaboration_control`, `publish_state`, and
`managed_identity_scope`. Every event resolves to exactly one filter and is listed
under it; families with no events in a profile still appear as empty filters.

## Export parity

Each timeline offers both a `machine_readable_json` summary export and a
`plain_language_handoff` packet, and every row carries both a machine summary and
a plain-language sentence.

## Invariants (all hold)

| Invariant | Statement |
| --- | --- |
| `decision_history.surface_states_within_matrix` | Every rendered state is one the frozen matrix admits for the decision-history surface. |
| `decision_history.decision_truth` | Every event names a stable id, decision code, policy epoch, affected target and scope, and time; ids are unique. |
| `decision_history.actor_classes_distinguished` | Every event names a specific actor class and each timeline uses at least two distinct classes. |
| `decision_history.actor_classes_all_present` | Every actor class appears across the bundle. |
| `decision_history.explorer_filters_complete` | Every timeline offers all eight family filters and every event resolves to exactly one. |
| `decision_history.export_parity` | Every row carries both export representations and every timeline offers both export forms. |
| `decision_history.no_silent_green` | Stale evidence never sits under a confirmed active/enforced state. |
| `decision_history.locally_inspectable_offline` | Every profile keeps a locally inspectable, vendor-console-independent history. |
| `decision_history.coverage_labeled` | A partial history is labeled, never implied complete. |
| `decision_history.ownership_visible` | Every event names an owner and every force-disable links to an explanation. |
| `decision_history.consumer_parity` | One typed packet serves every consumer the matrix declares for this surface identically. |
| `decision_history.profiles_covered` | The managed-cloud, self-hosted, sovereign/air-gapped, and mirrored/offline profiles are all rendered. |
| `decision_history.export_safe` | Every stable id is an opaque token with no URL scheme or absolute path. |

## How to regenerate / verify

```sh
# Regenerate the fixture from the in-code builder
cargo run -p aureline-policy --example dump_m5_decision_history > \
  fixtures/admin/m5-decision-history/canonical_history.json

# Freeze gate: in-code bundle must equal the checked-in fixture
cargo test -p aureline-policy --test m5_decision_history

# Human-readable projection
cargo run -p aureline-policy --example dump_m5_decision_history -- --lines
```
