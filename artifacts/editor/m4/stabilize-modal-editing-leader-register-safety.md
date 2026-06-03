# M04-191 — Stabilize modal editing, leader-key discovery, register routing, macro-safe replay, and keyboard-mode downgrade truth

## Release evidence

This artifact documents the governed, export-safe modal-editing safety packet produced by `crates/aureline-editor/src/stabilize-modal-editing-leader-register-safety/`.

## Record family

| Record | Kind | Schema | Version |
|---|---|---|---|
| `ModalEditingSafetyPacket` | `modal_editing_safety_packet` | `schemas/editor/mode_state_record.schema.json` | 1 |
| `EditorModeStateRecord` | `editor_mode_state_record` | (embedded in above) | 1 |
| `SurfaceDowngradeRecord` | (embedded) | (embedded in above) | 1 |
| `KeymapImportRegressionRecord` | (embedded) | (embedded in above) | 1 |

## Honesty invariants (all must pass)

1. Mode-state record covers all seven required register route kinds.
2. Blocked or unsupported register routes fail closed with a visible reason.
3. Unsafe macro replays (crosses files, run-capable, mutates settings, unstable timing) are reviewed or rejected.
4. Partial and unsupported sequence states are visible with notes.
5. Recovery paths to keymap diagnostics, command palette, and safe-mode reset exist.
6. Every surface downgrade carries a visible reason and accessibility announcement.
7. Import regression outcomes use the closed vocabulary: exact, translated, partial, shimmed, unsupported.
8. Modal discovery is unified with the command graph (`command_graph_unified == true`).
9. Modal cue latency budget is ≤ 1,000 µs on claimed stable rows.

## Claimed-stable corpus

Generated and pinned under `fixtures/editor/m4/stabilize-modal-editing-leader-register-safety/`.

| Scenario | Mode | Downgrades | Regressions |
|---|---|---|---|
| vim_normal_full_fidelity | normal | 0 | 0 |
| ime_downgrade | insert | 1 | 0 |
| accessibility_downgrade | normal | 1 | 0 |
| browser_companion_downgrade | normal | 1 | 0 |
| restricted_mode_downgrade | normal | 1 | 0 |
| large_file_downgrade | normal | 1 | 0 |
| import_exact | normal | 0 | 1 |
| import_translated | normal | 0 | 1 |
| import_partial | normal | 0 | 1 |
| import_shimmed | normal | 0 | 1 |
| import_unsupported | normal | 0 | 1 |
| macro_cross_file_review | normal | 0 | 0 |
| macro_run_settings_rejected | normal | 0 | 0 |

## Verification

Build and run the corpus emitter:

```sh
cargo run --bin aureline_modal_editing_safety
```

Run the integration test:

```sh
cargo test --test modal_editing_safety_replay
```

Check contract validity for every scenario:

```sh
cargo test -p aureline-editor stabilize_modal_editing
```

## Risks and follow-ups

- **Latency budget is a claim, not a measured guarantee.** The 1,000 µs budget needs benchmark-lab backing before it can be promoted to a published performance claim.
- **Browser-companion downgrade is irreversible in the current model.** The restore path requires opening desktop Aureline; a future iteration may support reversible narrowing via a companion protocol upgrade.
- **Import regression corpus covers Vim only.** Emacs, VS Code, IntelliJ, and Helix regressions should be added when their import adapters reach alpha.
- **Macro replay review does not yet integrate with the recipe runner.** Promoting a rejected macro to a reviewable recipe requires follow-up work in the command/recipe system.
