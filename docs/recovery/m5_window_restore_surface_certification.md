# M5 window-restore surface certification (M05-1187)

This contract is the **closing B141 surface-certification capstone** over the frozen M5 window-restore matrix
(`m5_window_restore_matrix`). Where the freeze matrix defines the five governed workspace-restore families —
**shared-workspace-authority, window-local-topology, skeleton-first-restore, no-rerun-session-hydration, and
display-topology-recovery** — the 1181–1184 implementation lanes resolve their per-surface workspace-authority,
window-topology, layout-skeleton, session-hydration, restore-fidelity, and display-recovery truth, the 1185
shared-consumer lane aligns their grammar across surfaces, and the 1186 accessibility lane proves keyboard /
screen-reader / high-zoom / high-contrast / localization / CLI-export parity, this capstone **certifies** that
the shared window-restore truth holds on every claimed M5 **desktop workspace profile** and auto-narrows any
profile that cannot sustain it.

- **Module:** `crates/aureline-ui/src/m5_window_restore_surface_certification/`
- **Schema:** `schemas/shell/m5-window-restore-surface-certification.schema.json`
- **Release proof:** `artifacts/release/m5-window-restore-surface-certification/`
  (`support_export.json`, `matrix.csv`) and `…-surface-certification.md`
- **Fixtures:** `fixtures/ui/m5-window-restore-surface-certification/`

## What the packet certifies

The packet is keyed on the claimed **profile** a user, reviewer, admin, or support engineer reads a
multi-window ownership, window-local-topology, layout-skeleton, session-hydration, restore-fidelity, or
display-remap surface through, not on the reusable workspace-restore family it renders:

1. **Live trusted restore surface** — a live, first-party, fully-current window restore. The **only** profile
   that may certify a `trusted_restore_surface` claim.
2. **Reviewable restore structure** — a self-sufficient, inspectable workspace-authority / window-topology /
   registry reference; certifies at most `reviewable_restore_surface`.
3. **Disclosed layout-skeleton profile** — a skeleton-first surface whose layout-skeleton proof can only be
   partially disclosed; auto-narrows to `layout_skeleton_disclosed_projection`.
4. **Unverified session-replay profile** — a no-rerun surface whose session-replay fence cannot be confirmed;
   auto-narrows to `session_replay_unverified_projection`.
5. **Unverified display-recovery profile** — a surface whose display-remap recovery evidence has aged out or is
   policy-blocked; auto-narrows to `display_recovery_unverified_projection`.

Each row certifies its profile across **nine truth axes** — visual, keyboard, screen-reader, high-zoom-reflow,
high-contrast, localization, CLI/export, degraded-state, and window-restore-component-truth behavior — and
resolves to a derived verdict:

- **green** — every axis certified, every invariant held, the claimed restore tier delivered;
- **yellow** — a truth axis is not current, so the restore claim auto-narrows to the weakest supported ceiling
  with a bound reason and a frozen downgrade trigger;
- **red** (blocked) — a degraded axis is hidden behind a fresh trusted claim, a hard invariant breaks, CLI/export
  parity drops, a non-live profile claims a trusted restore surface, or the narrowing is inconsistent.

## Invariants

1. **A degraded axis must produce a visible claim narrowing.** A profile that keeps a `trusted_restore_surface`
   / `reviewable_restore_surface` claim while one of its truth axes is not current over-claims and blocks.
2. **Only a live first-party profile may certify a trusted restore surface.** Every other profile is at most a
   reviewable restore structure or a narrowed projection.
3. **CLI/export parity is always-on.** Support and automation must always be able to reconstruct the canonical
   workspace authority, window topology, pane roles, layout skeleton, session-hydration posture, restore-fidelity
   class, display affinity, and registry reference as text / JSON / Markdown.
4. **Every B141 hard invariant holds per row.** No profile may rerun commands or reattach privileged sessions
   implicitly during restore, let a missing extension or remote target delete layout structure silently, leave
   windows or dialogs unreachable after a display-topology remap, merge workspace-authority state and
   window-topology state into one opaque blob, or overclaim restore fidelity when the system only reopened context
   or evidence.
5. **One canonical proof bundle.** Every row cites exactly one canonical window-restore proof bundle
   (`artifacts/release/m5-window-restore-proof/support_export.json`) — the frozen window-restore matrix proof —
   so release, docs, and support consume a single window-restore certification source rather than hand-authored
   prose.

## Boundary

The packet is metadata-only. Raw credentials, plaintext secrets, bearer tokens, endpoint URLs, and private-key
material never cross this boundary; the export carries typed tokens, opaque evidence refs, and repo-relative
paths only.

## Regenerating the artifacts

The checked-in release artifacts and fixtures are byte-locked to the seed builder. To regenerate them after an
intentional change, run:

```
GEN_WINDOW_RESTORE_CERT_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_window_restore_surface_certification::tests::regenerate_checked_artifacts_when_requested
```

then rebuild so the `include_str!` byte-lock tests pick up the new bytes.
