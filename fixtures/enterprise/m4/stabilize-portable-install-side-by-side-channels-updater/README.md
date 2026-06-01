# Fixture: Stabilize Portable Install, Side-by-Side Channels, Updater Ownership

This fixture set is the canonical truth source for the stable install-profile,
portable mode, side-by-side channel, updater/handler ownership, and rollback
scope claims materialized by the `stabilize_portable_install_side_by_side_channels_updater`
module in `crates/aureline-install`.

Later dashboards, About surfaces, diagnostics panels, and support exports must
ingest these fixtures rather than maintaining parallel status text.

## Files

| File | Content |
|---|---|
| `page.json` | Seeded `StabilizePortableInstallPage` record covering per-user, preview, portable, managed, and air-gapped install profiles |
| `summary.json` | Aggregate summary extracted from `page.json` |
| `defects.json` | Defects array — empty for the stable seeded page |
| `support_export.json` | Metadata-safe support-export projection referencing `page.json` |

## Coverage

The seeded page covers five install-profile rows:

| Row | Mode | Channel | Updater | Rollback scope | Portable guard |
|---|---|---|---|---|---|
| Windows Stable per-user | `per_user_installed` | `stable` | `user` | `full_artifact_graph` | n/a |
| Windows Preview per-user | `side_by_side_preview` | `preview` | `user` | `full_artifact_graph` | n/a |
| Windows Portable Stable | `portable` | `portable_stable` | `user` | `full_artifact_graph` | `fully_suppressed` |
| Windows Managed per-machine | `managed_deployed` | `stable` | `managed_fleet` | `managed_fleet_owned` | n/a |
| Air-gapped Bundle Stable | `offline_bundle` | `stable` | `admin` | `full_artifact_graph` | n/a |

Two side-by-side import-review rows prove that compare-or-skip with checkpoint
is available before any durable state root is shared:

- `stable-to-preview`: Stable → Preview channel handoff
- `portable-to-installed`: Portable → per-user installed handoff

Two fleet rollout diagnostics rows confirm that install-profile identity and
channel separation are preserved through log export and rollout revert.

## Acceptance criteria served

- About, update, diagnostics, and enterprise rollout surfaces all report the
  same install mode, channel, updater owner, and durable-state roots for each
  profile row.
- Stable and preview side-by-side tests prove state-root isolation and the
  `requires_import_review` path is gated on compare-or-skip.
- Portable-mode row proves `fully_suppressed` write guard and discloses absent
  integrations (shell hooks, PATH mutation, credential store, service
  registration).
- Rollback drills reference `rollback_scope: full_artifact_graph` for all
  user-owned rows; the managed row names `managed_fleet_owned` scope.
- Fleet diagnostics rows confirm `identity_preserved_in_export: true` and
  `channel_separation_maintained_on_revert: true` for managed and air-gapped
  lanes.
