# Migration center diff, rollback, and unsupported-gap taxonomy — release evidence

Reviewer-facing evidence packet for the lane that finishes the migration center
for switching users on claimed stable rows: one canonical disclosure record per
imported source ecosystem that binds the before/after diff, the rollback, and the
Exact / Translated / Partial / Shimmed / Unsupported taxonomy to a public claim
ceiling, an automatic narrow-below-Stable verdict, recovery and route parity
across the migration center / settings import history / command palette / menus,
accessibility across normal / high-contrast / zoomed layouts, and rows that stay
available without an account or managed services.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Records / fixtures: [`/fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported/`](../../../fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported/)
- Schema: [`/schemas/ux/finish-the-migration-center-diff-rollback-and-unsupported.schema.json`](../../../schemas/ux/finish-the-migration-center-diff-rollback-and-unsupported.schema.json)
- Companion doc: [`/docs/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md`](../../../docs/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported.md)
- Typed source: `aureline_shell::migration_center_stable` (`model`, `corpus`)
- Headless emitter: `aureline_shell_migration_center_stable`
- Replay + invariant gate: `crates/aureline-shell/tests/migration_center_stable_fixtures.rs`

## The claimed-stable matrix

| Flow | Ecosystem | Claim | Taxonomy (E/T/P/S/U) | Rollback live |
| --- | --- | --- | --- | --- |
| `vs_code_code_oss.json` | VS Code / Code-OSS | **stable** | 1/1/1/1/1 | yes |
| `jetbrains_family.json` | JetBrains IDEs | beta (narrowed) | 1/1/1/1/1 | no |
| `vim_neovim.json` | Vim / Neovim | beta (narrowed) | 1/1/1/1/1 | no |
| `emacs.json` | Emacs | beta (narrowed) | 1/1/1/1/1 | no |

The matrix spans the four incumbent ecosystems and the full taxonomy, and it
deliberately spans both sides of the cutline: the VS Code flow is a live apply
session and qualifies Stable, while the other three project from the corpus,
reference the same checkpoint but carry no live per-ecosystem rollback evidence,
and are narrowed to `beta` with `rollback_evidence_incomplete`.

## What this packet proves

1. **The diff is reviewed before apply.** Each record's `diff` has
   `reviewed_before_apply`, `every_row_has_before_after`, and
   `every_row_uses_one_checkpoint` true, with `row_count` equal to the classified
   rows. The builder narrows any flow whose diff is not a reviewable before/after
   surface.

2. **Rollback is only claimed when it is live.** Each record's `rollback` binds
   the wizard's pre-apply checkpoint (`created_before_apply`,
   `protects_every_domain`) and exposes Undo/Compare routes **only** when
   `verified_for_this_flow` holds. A flow that merely references an adjacent
   flow's checkpoint cannot assert rollback availability — the gate replays this
   for all four records.

3. **The unsupported-gap taxonomy is visible before apply.** Each record's
   `taxonomy` carries the Exact/Translated/Partial/Shimmed/Unsupported counts and
   the union of Unsupported and Shimmed `gaps`, each with
   `visible_before_apply: true`. Gap counts are cross-checked against the taxonomy
   counts.

4. **No row over-claims.** Each `claim_ceiling` field is bound to the real
   evidence: the diff-reviewed, rollback-available, no-unsupported-gaps, and
   full-fidelity assertions are rejected by the builder when unprovable.

5. **Unqualified flows narrow automatically.** `stable_qualification.claim_class`
   is derived, not supplied; a narrowed flow drops below the cutline and names a
   reason from the closed set instead of inheriting an adjacent green row.

6. **Recovery, routes, and surfaces share one model.** Each record exposes the
   required recovery routes (keyboard reachable), reaches the same flow from all
   four surfaces, keeps the settings/help/support_export reopen surfaces, and the
   replay gate asserts the migration center page and the stable lane pivot on the
   same wizard session, mapping report, checkpoint, and scoreboard.

7. **Accessibility holds in every layout.** Each record's `accessibility` carries
   the tab order, a narration that discloses the source ecosystem, action labels
   matching the recovery routes, and per-mode reachability for normal,
   high-contrast, and zoomed layouts.

8. **No-account / no-managed-services availability.**
   `available_without_account` and `available_without_managed_services` are fixed
   true on every record.

## How to verify

```sh
# Replay + invariant gate.
cargo test -p aureline-shell --test migration_center_stable_fixtures

# In-code corpus invariants.
cargo test -p aureline-shell --lib migration_center_stable

# Refresh the fixtures (must produce no diff).
cargo run -q -p aureline-shell --bin aureline_shell_migration_center_stable -- \
  emit-fixtures fixtures/ux/m4/finish-the-migration-center-diff-rollback-and-unsupported

# Reviewer index and per-flow plaintext truth block.
cargo run -q -p aureline-shell --bin aureline_shell_migration_center_stable -- index
cargo run -q -p aureline-shell --bin aureline_shell_migration_center_stable -- plaintext
```

## Honest posture

This is the honest posture for a pre-implementation repository: the migration
center's diff, rollback, and gap-taxonomy *disclosure* is replacement-grade and
qualifies Stable for the one ecosystem backed by a live apply session, and the
remaining ecosystems are narrowed below Stable with a named reason until their
own live rollback evidence lands — rather than papering over the gap by
inheriting the VS Code green row.
