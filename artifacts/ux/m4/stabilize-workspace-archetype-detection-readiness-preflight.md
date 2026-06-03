# Release Evidence: Workspace Archetype Detection, Readiness Preflight, Admission Checkpoints, and First-Useful-Work Routing

**Task:** M04-190 — Stabilize workspace archetype detection, readiness preflight, admission checkpoints, and first-useful-work routing
**Workstream batch:** B1 — Shell continuity, onboarding, settings, durable attention, and visual-system truth
**Execution wave:** W01
**Readiness wave:** R01
**Delivered:** 2026-06-02

## What changed

A new stable lane record — `WorkspaceArchetypeReadinessPreflightRecord` — was
minted in `crates/aureline-workspace/src/stabilize_workspace_archetype_detection_readiness_preflight/`
and wired into the workspace crate public API. The record is the canonical truth
source that shell, diagnostics, support exports, Help/About, and docs read
verbatim instead of cloning status text.

### Code changes

- `crates/aureline-workspace/src/stabilize_workspace_archetype_detection_readiness_preflight/mod.rs`
- `crates/aureline-workspace/src/stabilize_workspace_archetype_detection_readiness_preflight/model.rs`
- `crates/aureline-workspace/src/stabilize_workspace_archetype_detection_readiness_preflight/corpus.rs`
- `crates/aureline-workspace/src/bin/aureline_stabilize_workspace_archetype_detection_readiness_preflight.rs`
- `crates/aureline-workspace/tests/stabilize_workspace_archetype_detection_readiness_preflight_fixtures.rs`
- `crates/aureline-workspace/src/lib.rs` — module declaration and re-exports
- `crates/aureline-workspace/src/admission/checkpoint.rs` — added `open_minimal`
  to `MissingPrerequisite` route switch options (minimal fix to make the live
  builder conform to the M04-190 invariant)

### Schema

- `schemas/ux/stabilize-workspace-archetype-detection-readiness-preflight.schema.json`

### Fixtures

- `fixtures/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight/certified_ts_web_app.json`
- `fixtures/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight/probable_python_service.json`
- `fixtures/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight/mixed_ts_python_repo.json`
- `fixtures/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight/unknown_plain_folder.json`
- `fixtures/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight/restricted_policy_block.json`
- `fixtures/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight/missing_devcontainer_engine.json`

### Documentation

- `docs/ux/m4/stabilize-workspace-archetype-detection-readiness-preflight.md`

## Honesty invariants enforced

The builder refuses to mint a record that would:

1. Allow auto-install (`auto_install_allowed` must be `false`).
2. Allow auto-trust (`auto_trust_allowed` must be `false`).
3. Execute hidden setup (`hidden_setup_executed` must be `false`).
4. Widen trust silently (`trust_widened` must be `false`).
5. Present certified/probable without evidence freshness rows.
6. Present stale evidence as certified (forces downgrade).
7. Collapse readiness work into one undifferentiated list (blocking_now,
   recommended_soon, optional_later remain structurally distinct).
8. Omit source signal refs on readiness tasks.
9. Force a single boundary on mixed or ambiguous workspaces (all four choices
   must be present).
10. Omit same-weight bypass actions when setup is recommended (`Set up later`,
    `Open minimal`, `Dismiss recommendation`).
11. Hide `Open minimal` on restricted or missing-prerequisite routes.

## Verification

Run the following commands from the repository root:

```bash
cd crates/aureline-workspace
cargo test --lib stabilize_workspace_archetype_detection_readiness_preflight
cargo test --test stabilize_workspace_archetype_detection_readiness_preflight_fixtures
cargo run --bin aureline_stabilize_workspace_archetype_detection_readiness_preflight -- --corpus
```

All tests must pass and the corpus JSON must match the checked-in fixtures.

## Consumer wiring status

- **Shell surfaces** — not yet wired; they currently render their own status text.
- **Diagnostics / support exports** — not yet wired; they should ingest
  `WorkspaceArchetypeReadinessPreflightRecord` verbatim.
- **Help/About / docs** — not yet wired; contract doc is canonical narrative.

The record is ready for consumption; wiring into live UI surfaces is the next
step for this lane.
