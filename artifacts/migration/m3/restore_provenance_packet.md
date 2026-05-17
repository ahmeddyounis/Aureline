# Restore Provenance Packet

This packet binds first-useful-work restore/import rows to provenance
objects that can be quoted by docs, Help, migration center, and support
export without reinterpreting restore fidelity.

- Packet id: `migration_packet:restore_provenance.beta_switching`
- As of: `2026-05-15`
- Scorecard: `artifacts/milestones/m3/first_useful_work_scorecard.json`
- Packet manifest: `fixtures/onboarding/m3/first_useful_work/manifest.yaml`
- Restore card fixture: `fixtures/workspace/m3/portable_state_and_restore/restore_provenance_card_layout_only.json`
- Shared state schema: `schemas/state/restore_provenance.schema.json`
- Workspace restore schema: `schemas/workspace/restore_provenance.schema.json`

## Provenance Rules

Every restore or import row must show:

| Field | Required behavior |
|---|---|
| Source class | Auto checkpoint, manual export, imported competitor profile, layout snapshot, handoff packet, or support recovery bundle. |
| Producer version | Producer name, version, channel, and platform class when known. |
| Fidelity result | `exact`, `compatible`, `layout_only`, or `manual_review`, plus display restore level when applicable. |
| Machine-local exclusions | Secret material, live handles, local trust anchors, display hints, terminal authority, or delegated approvals excluded by design. |
| Missing dependencies | Missing extension, remote session, toolchain, kernel, route, display topology, or policy grant with fallback action. |
| Support/export refs | Diagnostics, compare, support export, and recovery refs retained after restore. |

## Imported Profile Restore Provenance
<a id="imported-profile-restore-provenance"></a>

Used by import scorecard rows:

- `scorecard_row:first_useful_work.import.vs_code_code_oss`
- `scorecard_row:first_useful_work.import.jetbrains_family`
- `scorecard_row:first_useful_work.import.vim_neovim`
- `scorecard_row:first_useful_work.import.emacs`

| Source class | Producer version | Fidelity result | Machine-local exclusions | Missing dependencies | Support/export refs |
|---|---|---|---|---|---|
| `imported_competitor_profile` | source profile version when detected; fallback `producer:source-profile:unknown` | `compatible` for mapped rows, `manual_review` for partial rows | source extension storage, source indexes, raw command history, live credentials, raw absolute paths | unsupported source runtime, missing native equivalent, policy-blocked authority | `fixtures/migration/m3/migration_wizard/support_export.json`, `fixtures/migration/m3/migration_wizard/unsupported_gaps.json` |

Outcome posture:

- Exact settings/keymap rows may apply after preview.
- Partial/manual-review rows stay inspectable before apply and after
  rollback.
- Unsupported runtime rows remain retained diagnostics and must not be
  restored as live authority.

Downgrade triggers:

- `restore_provenance_missing`
- `unsupported_gap_hidden`
- `machine_local_exclusion_hidden`
- `first_useful_work_packet_stale`

## Session And Layout Restore Provenance
<a id="session-and-layout-restore-provenance"></a>

Used by scorecard row:
`scorecard_row:first_useful_work.entry.restore`

| Source class | Producer version | Fidelity result | Machine-local exclusions | Missing dependencies | Support/export refs |
|---|---|---|---|---|---|
| `aureline_session_restore_manifest` or `aureline_layout_snapshot` | `producer:aureline:0.0.0-dev` in current fixtures | `layout_only` when remote authority or extension surface is missing | live terminal sessions, delegated approvals, remote authority tickets, secrets | missing extension, missing remote, changed display topology | `fixtures/workspace/m3/portable_state_and_restore/restore_provenance_card_layout_only.json` |

Required behavior:

- Preserve pane order and role as placeholders when a dependency is
  unavailable.
- Never replay terminal commands, debug sessions, notebook kernels, or
  remote actions as part of restore.
- Keep compare/export refs visible in diagnostics and support export.

## Missing Root And Recent Work Provenance
<a id="missing-root-and-recent-work-provenance"></a>

Used by scorecard row:
`scorecard_row:first_useful_work.entry.recent_work_missing_root`

| Source class | Producer version | Fidelity result | Machine-local exclusions | Missing dependencies | Support/export refs |
|---|---|---|---|---|---|
| `aureline_workspace_manifest_bundle` plus recent-work registry row | producer version is the current registry schema version | `manual_review` until target is located or opened without restore | raw absolute path in exported support bundles, local filesystem tokens | absent local path, offline remote route, stale root identity | `fixtures/workspace/missing_root_cases/missing_local_root.json`, `fixtures/ux/first_useful_work_cases/missing_target_local_repo_moved.yaml` |

Required behavior:

- Preserve `Locate`, `Open without restore`, and `Remove from recents`
  as same-weight actions.
- Keep the missing target diagnostic typed as `missing_target`; do not
  collapse it into a generic setup failure.

## Workspace Switch Provenance
<a id="workspace-switch-provenance"></a>

Used by scorecard rows:

- `scorecard_row:first_useful_work.entry.workspace_switch`
- `scorecard_row:first_useful_work.archetype.go_service_or_monorepo_slice`

| Source class | Producer version | Fidelity result | Machine-local exclusions | Missing dependencies | Support/export refs |
|---|---|---|---|---|---|
| `workspace_manifest_bundle` with selected workset artifact | workset artifact schema version `1` | `exact` for local UI, remote UI, and headless reopen parity in the current fixture | machine-local root handles excluded from portable export | rebinding required on another machine, provider lock if managed source appears | `fixtures/workspace/m3/workset_switcher/portable_named_workset.json` |

Required behavior:

- Preserve active workset identity across switch and reopen.
- Surface `portable_with_rebinding` rather than implying raw local
  paths are portable.
- Downgrade if the switcher loses exact reopen parity on a claimed row.

## Support Export Boundary

Restore provenance packets may export source class, producer version,
schema version, fidelity result, missing dependency class, diagnostic
refs, compare refs, and support refs. They must not export raw secrets,
live handles, delegated approvals, raw terminal bodies, raw source
profile files, or private absolute paths unless a redaction policy has
explicitly transformed them to safe labels.
