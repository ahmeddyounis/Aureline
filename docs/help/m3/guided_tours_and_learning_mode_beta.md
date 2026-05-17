# Guided Tours And Learning Mode Beta

This beta contract turns guided tours, guided-exercise rails, and learning-mode profiles into governed runtime objects. It is intentionally separate from first-run onboarding: learning surfaces can be enabled, paused, snoozed, reset, resumed, downgraded, or suppressed without blocking `Open folder`, `Clone`, `Import`, or first edit.

## Runtime Contract

- Tour packages are versioned records with `package_id`, package revision, source class, docs pack revision, command graph ref, semantic graph ref, citation refs, release label, availability state, freshness state, and independent downgrade group.
- Steps target stable object IDs: command IDs, file object IDs, docs node IDs, graph node IDs, symbol object IDs, or surface object IDs. They do not depend on brittle screen coordinates alone.
- Every step that claims product or repository truth carries source claims and citation refs. Generated or graph-limited statements use an explicit confidence class instead of presenting inference as fact.
- Guided exercises expose success criteria, hint/reveal state, skip/reset controls, sandbox or reversibility posture, rate limits, and restart-safe persistence.
- Mutation-capable teaching uses the ordinary command graph. The import-profile exercise uses `cmd:workspace.import_profile`, `preview:workspace.import_profile`, `approval:path:workspace.import_profile`, rollback semantics, and an evidence rule before any apply-capable action.
- Learning-mode profile changes alter framing only. Tip intensity, jargon level, dismissals, bookmarks, and educational AI posture never change data ownership, trust boundaries, approvals, or command authority.
- Progress snapshots are user-owned state. The default posture is local-only with optional sync requiring explicit user action, and support export is metadata-only.

## Degraded States

Learning content remains honest when evidence is incomplete:

- `cached_disclosed` rows remain usable but show cached source and freshness state.
- `graph_unavailable_placeholder` rows render as inactive placeholders and do not make uncited repository claims.
- Stale learning evidence can suppress beta or preview learning rows without disabling onboarding core.

## Runtime Consumers

The shell beta model is implemented in `crates/aureline-shell/src/learning_mode/mod.rs`.

The headless inspector emits:

- `fixtures/help/m3/guided_tours/manifest.json`
- `fixtures/help/m3/guided_tours/surface_projection.json`
- `fixtures/help/m3/guided_tours/support_export.json`

## Schemas

- `schemas/help/tour_package.schema.json` defines tour package, tour, step, target, source-claim, and action records.
- `schemas/help/learning_mode_profile.schema.json` defines the profile controls and ownership posture.

## Verification

```sh
cargo run -q -p aureline-shell --bin aureline_shell_learning_mode_beta -- validate
cargo test -p aureline-shell --test learning_mode_beta_fixtures
```
