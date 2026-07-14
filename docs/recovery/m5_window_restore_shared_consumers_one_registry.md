# M5 Window-Restore Shared Consumers: One Registry Across Surfaces

**Status:** Stable · B141 consumer-adoption lane
**Module:** `aureline_ui::m5_window_restore_shared_consumers_one_registry_across_surfaces`
**Schema:** [`schemas/shell/m5-window-restore-shared-consumers.schema.json`](../../schemas/shell/m5-window-restore-shared-consumers.schema.json)
**Proof:** [`artifacts/release/m5-window-restore-shared-consumers-proof/`](../../artifacts/release/m5-window-restore-shared-consumers-proof/)
**Fixtures:** [`fixtures/ui/m5-window-restore-shared-consumers/`](../../fixtures/ui/m5-window-restore-shared-consumers/)

This lane is the consumer-adoption capstone for the five reusable workspace-restore families frozen in
the [window-restore matrix](m5_window_restore_contract.md) and implemented by the workspace-authority /
window-topology, skeleton-first-restore / session-hydration, no-rerun session-recovery / authority-replay
-fence, and display-topology-recovery / role-continuity lanes. It binds each shared window-restore family
to the concrete restore-coordinator, shell, workspace, session, diagnostics, docs / help, CLI / export,
support-export, and general product consumers that render it, and proves — by fixtures, not screenshots —
that the same restore profile presents the **same registry** everywhere it appears.

## Why this exists

The batch already hardens shell-zone and multi-window continuity, remembered-state and portable-state
review artifacts, native desktop external-path recovery, and concrete shell geometry and platform-fit
rules, but it left Aureline's actual workspace-window ownership and restore orchestration too implicit for
each windowed surface. This lane wires those rules into the daily-driver windowed surfaces so restore
class, no-rerun semantics, placeholder posture, and display-affinity cannot drift between the shell,
editor, review, notebook, debug, terminal, collaboration, companion-handoff, and support / export
surfaces: every windowed surface consumes the shared registry rather than private wording or hand-copied
window notes.

## The three honesty axes

1. **Reuse.** Each of the five window-restore families is adopted by **at least two distinct consumers**,
   so a family is proven shared restore-engine infrastructure rather than a one-surface fork of
   workspace-authority, window-topology, restore-fidelity, or session-hydration copy.
2. **One registry / no drift.** For a given restore profile every consumer surface presents the identical
   six-word grammar — `window_restore_role_word`, `family_word`, `registry_reference_word`,
   `restore_context_word`, `surface_context_word`, and `session_continuity_word`. The role word must be a
   token from the frozen `M5WindowRestoreRole` vocabulary (`workspace_authority`, `window_topology`,
   `pane_role`, `layout_skeleton`, `session_hydration`, `restore_fidelity`, `display_affinity`), so no
   surface rewrites a role in its own words. A surface may narrow *how much* it shows across desktop,
   compact, remote, and exported representations, but never reword the grammar per surface — and a role
   that carries workspace-authority, session-hydration, restore-fidelity, or display-affinity meaning may
   never let a restore rerun or reattach session-scoped work implicitly, delete layout structure silently,
   strand a window or dialog off-screen after a display-topology remap, merge workspace-authority and
   window-topology into one opaque blob, or overclaim restore fidelity when only context or evidence
   reopened.
3. **Map back to one family.** Support and CLI/export consumers point at the canonical per-domain schema
   and the frozen matrix by id, so an exported packet always maps a window-restore surface back to one
   shared contract family.

## Guardrails (each MUST be false on every binding)

- `reruns_commands_or_reattaches_privileged_sessions_implicitly_during_restore`
- `deletes_layout_structure_silently_on_missing_extension_or_remote_target`
- `leaves_windows_or_dialogs_unreachable_after_display_topology_remap`
- `merges_workspace_authority_and_window_topology_into_one_opaque_blob`
- `overclaims_restore_fidelity_when_only_context_or_evidence_reopened`

## Narrowing is disclosed, never hidden

A compact, remote, or exported representation carries an explicit `narrow_note` naming the reason, the
preserved grammar, and the next action; a remote representation names its remote source, and an exported
representation names its export-safe detail boundary rather than collapsing the profile out of view.
Stale proof or a missing canonical reference **narrows** the claim via a
`WindowRestoreSharedConsumersDowngradeTrigger` rather than hiding the family.

## Seeded coverage

Five restore profiles — one per family — fan out to fifteen consumer bindings covering all nine consumers
and all four representations:

| Family | Role | Consumers |
| --- | --- | --- |
| `shared_workspace_authority` | `workspace_authority` | restore coordinator, shell, CLI export |
| `window_local_topology` | `window_topology` | workspace service, shell, support export |
| `skeleton_first_restore` | `layout_skeleton` | diagnostics, restore coordinator, product |
| `no_rerun_session_hydration` | `session_hydration` | session service, diagnostics, product |
| `display_topology_recovery` | `display_affinity` | docs/help, workspace service, support export |

Two checked narrowed fixtures prove the grammar survives compact / remote and exported / redacted forms
without rewording.

## Regenerating the proof

```text
cargo run -p aureline-ui --example dump_m5_window_restore_shared_consumers -- support-export
cargo run -p aureline-ui --example dump_m5_window_restore_shared_consumers -- csv
cargo run -p aureline-ui --example dump_m5_window_restore_shared_consumers -- report
cargo run -p aureline-ui --example dump_m5_window_restore_shared_consumers -- fixture-compact-remote-narrowed
cargo run -p aureline-ui --example dump_m5_window_restore_shared_consumers -- fixture-exported-redaction-narrowed
```

The example is the only mint-from-truth path for the checked support export, matrix CSV, Markdown summary,
and narrowed fixtures; the module tests fail if any drifts from the seed builder.
