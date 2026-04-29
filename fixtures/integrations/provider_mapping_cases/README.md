# Provider mapping fixtures

Worked scenario bundles for the contract frozen in
[`/docs/integrations/provider_account_mapping_and_offline_capture_contract.md`](../../../docs/integrations/provider_account_mapping_and_offline_capture_contract.md).

Each fixture is a self-contained YAML document. The `records` array
contains records valid against
[`/schemas/integrations/provider_mapping_state.schema.json`](../../../schemas/integrations/provider_mapping_state.schema.json).

Coverage:

| Fixture                                  | Main condition                                                                          | Required behavior                                                                                                                            |
|------------------------------------------|------------------------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------------------------------------------|
| `repo_inherited_mapping.yaml`            | Repo-attached provider link inherited as the active mapping.                             | Disclose `scope_provenance_class = repository_metadata` verbatim; resolve default target as `repo_metadata_inherited`; admit sync modes within the inherited scope. |
| `admin_forced_scope.yaml`                | Org admin policy forces the surface onto a specific board and narrows permitted sync modes. | Resolve `default_target_resolution_class = admin_forced`; narrow `permitted_sync_modes`; emit an `admin_policy_changed_target` audit hook citing the affected mappings. |
| `offline_capture_only_mode.yaml`         | Provider unreachable; user files a bug report.                                            | Pin `metadata_safe_export_only = true`; mint an `offline_capture_control_record` riding the publish-later queue under `publish_when_connectivity_returns`; preserve handoff state refs. |
| `cached_read_only_state.yaml`            | Seat transitioned to a bounded grace window; mutation denied; cached read state remains. | Render `account_state_class = limited_scope`, `account_kind_class = grace_mode`, `seat_or_plan_state_class = seat_grace`; narrow `sync_mode_class` to `read_only_sync`; emit a `seat_state_changed` audit hook. |
| `policy_blocked_provider_action.yaml`    | Admin policy denies all external provider actions for the surface.                        | Render `account_state_class = policy_blocked` paired with `admin_forced_target`; narrow the mapping to `blocked_by_admin_policy` / `offline_capture_only`; capture a blocked-work note under `publish_after_admin_policy_review`. |

Conventions:

- Records cite opaque ids only; raw URLs, raw OAuth tokens, raw
  delegated tokens, raw cookies, raw provider-private profile bodies,
  raw billing payloads, and raw export bodies never appear.
- `preserved_handoff_state_refs` lists publish-later queue item refs,
  browser-handoff packet refs, or imported-snapshot refs that the
  fixture's account state and mapping carried across the transition.
  Loss of provider connectivity, seat expiry, account switch, or
  policy change MUST NOT erase prepared handoff state.
- `metadata_safe_export_only` is pinned `true` whenever cached
  read-only state, offline mode, or policy-blocked state is in
  effect, so silent widening of export or telemetry scope is
  structurally impossible.
