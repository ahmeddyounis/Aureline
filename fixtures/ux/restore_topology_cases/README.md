# Restore topology drift + placeholder cases

Seed corpus for:

- [`/docs/ux/restore_placeholder_and_recenter_matrix.md`](../../../docs/ux/restore_placeholder_and_recenter_matrix.md)

These fixtures describe reviewable outcomes when restore must reconcile:

- display / monitor topology drift (dock/undock, detach, wake/reconnect,
  mixed-DPI changes, fullscreen/snap rewrites); and
- dependency loss (missing extensions/feature packs, remote targets,
  expired managed sessions, revoked permissions).

Each YAML file is a single scenario that pins:

- the typed `topology_change_class` inputs that force safe-bounds remap,
  recenter, or re-dock;
- the expected restore-history and provenance event classes that make the
  adjustment reviewable after reopen; and
- the placeholder posture (missing-dependency class + recovery actions)
  when capability cannot hydrate.

Fixtures are narrative-first and do not carry raw paths, raw URLs, raw
credentials, or raw logs. Identities are opaque refs; timestamps are
illustrative only.

## Cases

| Fixture | Exercises |
| --- | --- |
| [`display_detach_safe_bounds_remap.yaml`](./display_detach_safe_bounds_remap.yaml) | display removed → safe bounds remap + owner-dialog recenter + terminal evidence-only no-rerun |
| [`mixed_dpi_scale_normalized.yaml`](./mixed_dpi_scale_normalized.yaml) | scale bucket drift → scale normalize + safe bounds clamp while preserving focus reachability |
| [`wake_reconnect_managed_session_expired.yaml`](./wake_reconnect_managed_session_expired.yaml) | wake reconnect + expired managed session → authority boundary disclosure + placeholder instead of hidden reauth |
| [`dependency_missing_preserves_layout.yaml`](./dependency_missing_preserves_layout.yaml) | dependency missing on reopen → placeholder inserted in same slot, no silent layout collapse |

