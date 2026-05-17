# Learning Mode Packet

## Scope

This packet publishes the beta guided-tour and learning-mode contract used by the shell learning rail, learning-mode header, contextual inspector, learning digest, progress snapshot, and support export.

## Published Artifacts

| Artifact | Path |
| --- | --- |
| Tour package schema | `schemas/help/tour_package.schema.json` |
| Learning profile schema | `schemas/help/learning_mode_profile.schema.json` |
| Runtime model | `crates/aureline-shell/src/learning_mode/mod.rs` |
| Headless inspector | `crates/aureline-shell/src/bin/aureline_shell_learning_mode_beta.rs` |
| Manifest fixture | `fixtures/help/m3/guided_tours/manifest.json` |
| Surface projection fixture | `fixtures/help/m3/guided_tours/surface_projection.json` |
| Support export fixture | `fixtures/help/m3/guided_tours/support_export.json` |
| Contract docs | `docs/help/m3/guided_tours_and_learning_mode_beta.md` |

## Release Truth

- Beta and preview labels are stored on each package and projected into each surface row.
- Tour steps cite commands, docs nodes, files, or graph nodes before making product or repository claims.
- The mutation teaching fixture routes through `cmd:workspace.import_profile` with preview, approval, rollback, and evidence refs.
- Cached and graph-unavailable packages downgrade independently from onboarding core.
- Profile controls cover enable, pause, snooze, reset, and resume without changing trust or authority.

## Support Export

The support export is metadata-only. It records:

- active package versions and downgrade groups;
- profile state classes, tip intensity, jargon level, mutation guardrails, and optional-sync posture;
- progress state, exact reopen refs, and local-or-sync posture.

It omits raw step bodies, raw package bodies, raw profile notes, private workspace paths, and account identifiers.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_learning_mode_beta -- validate
cargo test -p aureline-shell --test learning_mode_beta_fixtures
```
