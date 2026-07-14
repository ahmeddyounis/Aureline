# M5 Window-Restore Accessibility & Auto-Narrowing Parity (M05-1186)

This contract is the accessibility-localization-support-export parity and honest auto-narrowing capstone over
the frozen [M5 window-restore matrix](m5_window_restore_contract.md). Where the freeze matrix defines the five
governed workspace-restore families and the 1181–1184 implementation lanes resolve their per-surface
workspace-authority, window-topology, skeleton-first restore, no-rerun session-hydration, and
display-topology-recovery truth, this lane certifies — per family — that restore truth stays reachable and
exportable even when the active profile is degraded or only partially qualified.

- **Schema:** `schemas/shell/m5-window-restore-accessibility-parity.schema.json`
- **Support export / CSV / report:** `artifacts/release/m5-window-restore-accessibility-parity/`
- **Fixtures:** `fixtures/ui/m5-window-restore-accessibility-parity/`
- **Canonical packet id:** `m5-window-restore-accessibility-parity:stable:0001`

## What it certifies

Each row keys on one `M5WindowRestoreFamily` and reuses the frozen matrix vocabulary — family tokens, required
labels, downgrade triggers, and consumer surfaces — rather than minting parallel synonyms.

1. **Keyboard / screen-reader / high-zoom / high-contrast / localization / CLI reach.** Every family exposes a
   non-visual path into the same window-restore identity, semantic role, registry reference, workspace
   authority, restore-fidelity class, and display affinity the rendered surface shows. Structure-heavy families
   (skeleton-first restore-fidelity, no-rerun session-replay, display-topology remap-history) additionally bind
   their structured layout to a flat list / textual / CLI path.
2. **Export parity.** The support / release / CLI export reconstructs each family's meaning from typed tokens
   and opaque refs without a raw payload.
3. **Honest auto-narrowing.** When a skeleton-first family's layout-skeleton proof can only be partially
   disclosed, a no-rerun session-replay fence cannot be confirmed, or a display-remap recovery evidence has aged
   out or is policy-blocked, the family's claim auto-narrows from `trusted_restore_surface` /
   `reviewable_restore_surface` to a `layout_skeleton_disclosed_projection` /
   `session_replay_unverified_projection` / `display_recovery_unverified_projection`, discloses the narrowing
   with a precise frozen trigger and binding dimension, and preserves the canonical identity. A
   fidelity-overclaimed, evidence-aged, or policy-blocked state can never keep a trusted, stable restore claim.
4. **Cross-surface disclosure.** The same narrowed state surfaces in the restore coordinator, shell UI,
   workspace service, session service, diagnostics, docs / help, CLI export, support export, and product UI so
   product, help, and release publication stay aligned on downgrade behavior.

## Acceptance criteria mapping

- **Accessibility and CLI/export paths inspect the same restore truth shown in GUI shell and recovery
  surfaces** → the per-row reach axes plus the export-summary and copy-export parity.
- **Claim publication and support exports downgrade automatically when B141 evidence is stale or incomplete** →
  the auto-narrow blocks bound to the frozen `proof_stale` and
  `overclaimed_restore_fidelity_when_only_context_or_evidence_reopened` triggers.
- **No claimed desktop profile can stay green after shared-authority, no-rerun, or display-remap proof ages
  out** → `cannot_be_shown_trusted` flags the session-replay-unconfirmed and display-recovery-unconfirmed
  states so the effective claim can never assert a trusted restore surface.

The packet is metadata-only: raw secret blobs, machine-specific sensitive paths, plaintext payloads, and
endpoint refs never cross this boundary.
