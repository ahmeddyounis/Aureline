# M5 scaffold-component matrix contract

This document is the human-readable companion to the frozen M5 scaffold-component matrix.
The authoritative gate is the Rust validator in
`crates/aureline-templates/src/freeze_the_m5_scaffold_template_card_starter_parameter_row_scaffold_preflight_card_template_health_row_generated_project_diff_card_and_scaffold_handoff_banner_component_matrix`.
The checked-in support export under `artifacts/release/m5-scaffold-component-proof/` is the
single source of truth; the schemas under `schemas/ui/` document the shape.

## Purpose

The matrix freezes the reusable scaffold / project-entry components so start-center, template
gallery, preflight, generation-diff, and workspace-handoff surfaces stop drifting across
claimed M5 project-entry and starter-generation flows. It names each component family once and
binds it to starter source truth, parameter source layers, side-effect disclosure,
generated-versus-user-owned boundaries, and recovery / delete-generated language before
widening consumer coverage.

## Component families

- `scaffold_template_card` — where a starter comes from (first-party, team-managed, community,
  local-only, mirrored, unknown source) and how it is supported (officially supported,
  community supported, experimental, bridge behavior, deprecated, unsupported).
- `starter_parameter_row` — where a parameter value comes from (default value, user provided,
  profile inherited, environment derived, computed derived, unset required) and whether its
  action is applied immediately or deferred (applied immediately, deferred after create,
  requires confirmation, blocked invalid, optional skippable, not applicable).
- `scaffold_preflight_card` — what is checked before a starter writes files (tooling present,
  dependency availability, network access, workspace writable, host boundary, credential
  scope) and each check's outcome (passed, warning, blocked, skipped optional, not run,
  unknown).
- `template_health_row` — which health facet is reported (build health, dependency freshness,
  security advisories, test status, maintenance cadence, compatibility) and how current the
  signal is (fresh, aging, stale, expired, never checked, unavailable).
- `generated_project_diff_card` — what a starter wrote versus what the user owns (generated
  only, user owned, generated then edited, runtime only, mixed zone, zone unknown) and its
  diff-review state (preview ready, review required, no changes, conflict detected, diff
  unavailable, blocked).
- `scaffold_handoff_banner` — the bootstrap outcome (create succeeded, partial bootstrap,
  create failed, continued without starter, created empty, provisioning pending) and the
  recovery path it keeps explicit (open workspace, retry bootstrap, delete generated, continue
  without starter, keep partial review, no recovery needed).

## Controlled disposition vocabulary

Every consumer binds one controlled disposition vocabulary, the exact acceptance-criteria
labels, so no surface invents a parallel word:

`first_party`, `team_managed`, `community`, `local_only`, `create_empty`,
`continue_without_starter`, `blocked`, `warning`, `optional`.

## Mandatory labels

Every component must be able to show `identity`, `state`, and `keyboard_route`. Scaffold
components additionally close the acceptance-criteria ambiguity with
`starter_source_and_support`, `side_effect_disclosure`, and
`recovery_and_ownership_boundary`.

## Hard invariants

Each component row asserts five hard invariants that must be `false`:

- `hides_starter_source_or_support_class` — never mask where a starter comes from or its
  support class.
- `hides_side_effect_behind_generic_create` — never hide a network, dependency-install,
  remote-provisioning, trust, or managed-workspace side effect behind a generic Create.
- `hides_generated_versus_user_owned_boundary` — never blur what is generated versus what the
  user owns.
- `omits_recovery_or_continue_without_starter_path` — keep Continue without starter, Create
  empty, and delete-generated recovery paths explicit.
- `invents_alternate_state_label` — never invent a parallel label for a governed state.

## Narrowed variants

Two checked-in narrowed fixtures under `fixtures/ui/m5-scaffold-components/` demonstrate honest
narrowing without hiding a component: the scaffold preflight card held at Beta while
environment-dependent network / dependency / host-boundary checks earn parity proof, and the
scaffold handoff banner narrowed to Preview pending remote-provisioning and delete-generated
recovery parity proof. Every component stays visible in both variants.

## Regenerating the artifacts

The canonical seed builder is the single producer of the support export, the matrix CSV, the
Markdown report, and both narrowed fixtures. Regenerate deterministically with the example
emitter:

```text
cargo run -p aureline-templates --example dump_scaffold_component_matrix -- support-export
cargo run -p aureline-templates --example dump_scaffold_component_matrix -- csv
cargo run -p aureline-templates --example dump_scaffold_component_matrix -- report
cargo run -p aureline-templates --example dump_scaffold_component_matrix -- fixture-scaffold-preflight-card-beta-narrowed
cargo run -p aureline-templates --example dump_scaffold_component_matrix -- fixture-scaffold-handoff-banner-preview-narrowed
```
