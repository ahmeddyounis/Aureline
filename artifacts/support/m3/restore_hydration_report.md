# Restore Hydration Report

This packet reviews the restore hydrator: the runtime that recreates window
shells and pane topology first, then hydrates heavy dependencies lazily,
replacing missing surfaces with truthful placeholders and never replaying
mutating or privileged sessions.

## Evidence

| Evidence | Path |
| --- | --- |
| Request schema | `schemas/workspace/window_topology_snapshot.schema.json` |
| Provenance schema | `schemas/workspace/pane_tree.schema.json` |
| Rust runtime | `crates/aureline-workspace/src/restore_hydrator/mod.rs` |
| Canonical fixtures | `fixtures/workspace/m3/restore_hydration/` |
| Beta contract | `docs/workspace/m3/restore_hydrator_beta.md` |
| Tests | `crates/aureline-workspace/tests/restore_hydration.rs` |

## Review Findings

| Area | Result |
| --- | --- |
| Skeleton first | Window shells, pane topology, and focus anchors are recreated before any heavy hydration; the `skeleton` phase completes for every window. |
| Topology preserved | Every leaf pane id from the snapshot is preserved in the restored window; validation rejects any dropped pane. |
| Missing-surface honesty | Missing extensions, remotes, remote authority, revoked permissions, and dead live processes reopen as placeholders with the in-product reason class and safe actions. |
| No impersonation | Placeholder postures never use `live_attach_visible`; exact-restore windows are rejected if they carry any placeholder or evidence-only outcome. |
| Monitor-safe remap | Vanished displays, mismatched scale buckets, off-screen bounds, and fullscreen windows are remapped and recorded as `display_adjustment` rows with affected pane ids; final bounds always sit inside a connected display. |
| No silent rerun | Every live-surface outcome records `explicit_user_action_required`; command-bearing surfaces add `no_command_rerun`; authority is never reacquired silently. |
| Authority safety | Workspace authority is resolved by ref in the `rebind` phase; live authority, tokens, and delegated approvals are never serialized or replayed. |
| Restore class | The window restore class folds the worst pane outcome with display adjustments into `exact_restore`, `compatible_restore`, `layout_only`, or `evidence_only`. |

## Support Export Posture

`RestoreHydrationSummary` compiles a metadata-safe envelope with:

- aggregate restore class and per-class pane counts (live, placeholder, evidence-only);
- missing-dependency classes using the in-product `placeholder_reason_class` vocabulary;
- remaining manual actions using the in-product `placeholder_action_class` vocabulary;
- display-adjustment classes applied during restore;
- stable diagnostics, support-export, and crash-recovery refs that keep the summary visible.

`render_plaintext` renders a support-safe view of the same truth. The envelope
carries opaque ids only — raw paths, hosts, credentials, command lines, source
content, and live authority handles never cross the boundary.

## Follow-Ups

- Wire `RestoreHydrationSummary` into the shell restore diagnostics surface and
  the support export bundle so the in-product cards and the exported report read
  the same record.
- Extend the corpus with a reattachable-session drill once a persistent-session
  transport lands.
