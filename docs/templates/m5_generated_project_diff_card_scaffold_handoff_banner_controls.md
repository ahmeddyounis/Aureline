# M5 generated-project diff cards and scaffold handoff banners

The generated-project diff card and the scaffold handoff banner are the last two of the six
governed scaffold / project-entry components frozen by the
[M5 scaffold-component matrix](m5_scaffold_component_matrix.md). This lane implements those two
families as two co-equal control vectors in one export-safe packet,
[`GeneratedProjectDiffCardScaffoldHandoffBannerControlsPacket`](../../crates/aureline-templates/src/implement_generated_project_diff_cards_and_scaffold_handoff_banners_with_create_modify_rename_delete_counts_dependency_task_extension_impact_trust_state_and_run_now_later_review_recovery_truth_across_claimed_m5_generation_flows/mod.rs),
so a claimed M5 diff-review, workspace-handoff, start-center, or CLI surface can project a diff card
and a handoff banner that keep **what a starter wrote and how to recover** explicit *after* Aureline
writes files — never inferred, never presenting a conflict or failed bootstrap as a clean create,
and never assuming the safest next step for the user.

## What the resolvers decide

The module has two derived resolvers so the honesty of each component is computed, never asserted.

### `resolve_diff_disclosure`

Given a diff card's frozen **generated-zone class** and **diff-review state**, the resolver derives a
**review disposition** and a **generated-versus-user-owned boundary posture**:

- `preview_ready` -> `reviewable_preview`
- `review_required` -> `review_required_before_write` (must carry a review-required note)
- `no_changes` -> `no_changes_to_review` (must carry a no-changes note)
- `conflict_detected` -> `conflict_blocked` (must carry a conflict note; blocking)
- `diff_unavailable` / `blocked` -> `diff_unavailable_blocked` (must carry an unavailable note; blocking)

- `generated_only` -> `generated_owned`
- `user_owned` -> `user_owned`
- `generated_then_edited` -> `generated_then_user_edited`
- `runtime_only` -> `runtime_only`
- `mixed_zone` -> `mixed_ownership`
- `zone_unknown` -> `ownership_unknown`

A conflict or unavailable diff can therefore never read as a clean applied change, and a user-owned
zone can never read as free-to-overwrite generated output. The card independently counts its changes
with the exact **create / modify / rename / delete** vocabulary — the same one Aureline uses for AI
patches, importers, and refactors — and names its **source kind** (`template_starter`,
`framework_generator`, `codemod`, `imported_source`, or `user_authored`).

### `resolve_handoff_disclosure`

Given a handoff banner's frozen **outcome class**, the resolver derives an **outcome posture**:

- `create_succeeded` -> `clean_create`
- `partial_bootstrap` -> `partial_needs_recovery` (must carry a partial note; needs recovery)
- `create_failed` -> `failed_needs_recovery` (must carry a failed note; needs recovery)
- `continued_without_starter` -> `continued_without_starter`
- `created_empty` -> `created_empty`
- `provisioning_pending` -> `provisioning_pending` (must carry a pending note)

A partial or failed bootstrap can therefore never read as a clean create. The banner independently
names its **trust state** — `trusted`, `trust_prompt_pending`, `restricted_trust`,
`untrusted_blocked`, or `trust_not_applicable` — carrying a trust note whenever the workspace is not
fully trusted.

## What each component names

### Generated-project diff card

- Created / modified / renamed / deleted **counts**, with a change-summary note.
- Template or generator **source** and source label.
- Config, dependency, task, and extension **impact** labels.
- A named **checkpoint** and a **rollback / delete-generated** note.
- A generated-versus-user-owned **boundary cue**.
- Bounded actions — always `review_generated_diff`, `review_change_impact`,
  `review_ownership_boundary`, and a `rollback_generated` recovery path — plus a stable
  manifest / registry / docs / policy deep link.

### Scaffold handoff banner

- Created-workspace **identity** (id and name) and **trust state**.
- A **health summary**.
- Bounded actions — always `run_now`, `run_later`, `review_files`, and `open_manifest`, so optional
  setup stays visibly optional — plus a stable deep link.
- Explicit **recovery actions** (`delete_generated`, `continue_without_starter`, `retry_bootstrap`,
  or `keep_partial_review`) and a reopen-preflight route, so the safest next step is never assumed.

## Hard invariants

Every diff card and every handoff banner keeps four hard invariants `false`:

- `hides_generated_versus_user_owned_boundary`
- `hides_side_effect_or_trust_state`
- `assumes_safest_next_step_without_recovery`
- `invents_alternate_state_label`

## Acceptance criteria

- **Generated output is previewable and reviewable using the same create / modify / rename / delete
  vocabulary Aureline uses for AI patches, importers, and refactors.** Each diff card counts its
  changes with the exact `created` / `modified` / `renamed` / `deleted` tokens, and the seed covers
  all four across the card set. A card that drops the `rollback_generated` recovery action fails
  validation.
- **Post-create handoff keeps optional setup visibly optional and preserves recovery /
  delete-generated routes instead of assuming the safest next step for the user.** Each handoff
  banner must offer `run_now` and `run_later` (setup is never the assumed default) and a real
  recovery route; a banner that offers neither `delete_generated`, `continue_without_starter`,
  `retry_bootstrap`, nor `keep_partial_review` fails validation.

## Provenance and export safety

The checked-in support export
(`artifacts/release/m5-generated-project-diff-card-scaffold-handoff-banner-proof/support_export.json`),
the machine-readable matrix CSV, the two scenario fixtures
(`fixtures/ui/m5-generated-project-diff-card-scaffold-handoff-banner-controls/`), and the Markdown
design report are all regenerated deterministically from the canonical seed builders by the
`dump_scaffold_generation_controls` example. Raw file bodies, raw secret values, pasted local paths,
repository URLs, credentials, and secrets never cross the export boundary.
