# Migration center diff, rollback, and unsupported-gap taxonomy — release evidence

Reviewer-facing evidence packet for the lane that finishes the migration center
for switching users toward the Stable target: one canonical disclosure record
per imported source ecosystem that binds the before/after diff, the rollback,
and the Exact / Translated / Partial / Shimmed / Unsupported taxonomy to a
public claim ceiling, an automatic narrow-below-Stable verdict, recovery and route parity
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

## The stable-lane preview matrix

| Flow | Ecosystem | Claim | Taxonomy (E/T/P/S/U) | Rollback live |
| --- | --- | --- | --- | --- |
| `vs_code_code_oss.json` | VS Code / Code-OSS | beta (preview narrowed) | 1/1/1/1/1 | no |
| `jetbrains_family.json` | JetBrains IDEs | beta (narrowed) | 1/1/1/1/1 | no |
| `vim_neovim.json` | Vim / Neovim | beta (narrowed) | 1/1/1/1/1 | no |
| `emacs.json` | Emacs | beta (narrowed) | 1/1/1/1/1 | no |

The matrix spans the four incumbent ecosystems and the full taxonomy. Its
upstream wizard record is a dry-run preview, so all four rows carry the same
checkpoint requirement, expose no checkpoint/restore/Undo/Compare evidence,
and narrow to `beta` with `rollback_evidence_incomplete`. The matrix no longer
converts preview correlation into an apply session or a Stable verdict.

## What this packet proves

1. **The diff is reviewed before apply.** Each record's `diff` has
   `reviewed_before_apply`, `every_row_has_before_after`, and
   `every_row_uses_one_requirement` true, with `row_count` equal to the classified
   rows. The builder narrows any flow whose diff is not a reviewable before/after
   surface.

2. **Preview does not fabricate rollback.** Each record's `rollback` binds the
   wizard's `rollback_requirement_ref`. The optional checkpoint and restore refs
   are absent, the lifecycle booleans are false, and Undo/Compare routes are not
   emitted. The builder rejects a lifecycle claim without complete real refs.

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
   same wizard review, mapping report, rollback requirement, and scoreboard.

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

This is the honest posture for a preview-only seeded repository: the migration
center's diff and gap taxonomy are reviewable, but no seeded ecosystem claims a
live rollback or Stable status. Promotion requires real execution checkpoint,
restore, and validation evidence for the specific flow.
