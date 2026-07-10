# M5 scaffold preflight cards and template health rows

The scaffold preflight card and the template health row are two of the six governed scaffold /
project-entry components frozen by the
[M5 scaffold-component matrix](m5_scaffold_component_matrix.md). This lane implements those two
families as two co-equal control vectors in one export-safe packet,
[`ScaffoldPreflightCardTemplateHealthRowControlsPacket`](../../crates/aureline-templates/src/ship_scaffold_preflight_cards_and_template_health_rows_with_generated_file_counts_immediate_versus_deferred_actions_blocked_warning_optional_checks_and_create_empty_parity_across_claimed_m5_bootstrap_lanes/mod.rs),
so a claimed M5 start-center, scaffold-preflight, template-health, or CLI surface can project a
preflight card and a health row that make **what a starter writes, which checks are current, which
actions run immediately versus later, and how to recover** explicit before a user commits — never
inferred, never routing creation through a generic Create that hides a side effect, and never
letting a health row monopolize the plain create-without-starter path.

## What the resolvers decide

The module has two derived resolvers so the honesty of each component is computed, never asserted.

### `resolve_preflight_disclosure`

Given a preflight card's frozen **result state**, the resolver derives a **severity**:

- `passed` -> `clear`
- `warning` -> `advisory` (must carry a warning note)
- `blocked` -> `blocked_prerequisite` (must carry a blocked note; a blocking prerequisite)
- `skipped_optional` -> `optional_skipped` (must carry a skipped note)
- `not_run` / `unknown` -> `needs_attention` (must carry an attention note)

A blocked prerequisite can therefore never read as an optional optimization, and a not-run or
unknown check can never read as passed. The card independently names its concrete
**side-effect kind** — one of `package_install`, `dependency_restore`, `remote_provisioning`,
`trust_prompt`, `script_execution`, `extension_install` (or `no_side_effect`) — and its
**action timing** — `runs_immediately`, `deferred_for_later`, `requires_confirmation`,
`blocked_until_resolved`, or `not_applicable` — so a generic Create can never hide a package
install, dependency restore, remote provisioning, trust prompt, script execution, or extension
install, and the immediate-versus-deferred boundary stays explicit.

### `resolve_health_disclosure`

Given a health row's frozen **freshness state**, the resolver derives a **freshness posture**:

- `fresh` -> `current`
- `aging` -> `aging`
- `stale` / `expired` -> `stale_or_expired` (must carry a stale note)
- `never_checked` -> `never_checked` (must carry a never-checked note)
- `unavailable` -> `unavailable` (must carry an unavailable note)

A stale, expired, never-checked, or unavailable signal can therefore never read as fresh. The row
independently names its **severity** — the acceptance-criteria `blocker` / `warning` / `info`
label — and its **fix kind** — `auto_fix_available`, `manual_fix_required`, or `no_fix_needed` —
so it distinguishes a blocked prerequisite from a warning and from an optional optimization.

## What each component names

### Scaffold preflight card

- Target path and name.
- Generated **file and folder counts**, with a generated-impact note.
- Dependency, task, and extension impact labels.
- The concrete side effect it discloses and whether that action runs immediately or is deferred,
  with an immediate-action label and a deferred-action label.
- A named checkpoint or **delete-generated recovery path**.
- Bounded actions — always `review_side_effects`, `review_generated_impact`,
  `review_recovery_path`, and a same-weight `create_empty` — plus a stable
  manifest / registry / docs / policy deep link.

### Template health row

- Check name, status, and freshness / source.
- `Blocker` / `Warning` / `Info` severity and an auto-fix or manual-fix note.
- Bounded actions — always `rerun_check` and `open_detail`, plus an explicit **same-weight
  path to `create_empty` or `continue_without_starter`** — and a stable deep link.

## Hard invariants

Every preflight card and every health row keeps four hard invariants `false`:

- `hides_side_effect_behind_generic_create`
- `hides_generated_impact_or_recovery_path`
- `monopolizes_plain_create_without_starter_path`
- `invents_alternate_state_label`

## Acceptance criteria

- **Preflight no longer hides package install, dependency restore, remote provisioning, trust,
  script execution, or extension install under a generic Create action.** Each preflight card
  names its `side_effect_kind` from the exact acceptance-criteria vocabulary, carries a
  side-effect note whenever that kind is a real write, and the seed covers all six real
  side-effect kinds. `create_empty` is a mandatory same-weight action.
- **Health rows distinguish blocked prerequisites, warnings, and optional optimizations without
  monopolizing the plain open / create-without-starter path.** Each health row names its
  `Blocker` / `Warning` / `Info` severity and must offer a same-weight `create_empty` or
  `continue_without_starter` action; a row that drops both fails validation.

## Provenance and export safety

The checked-in support export
(`artifacts/release/m5-scaffold-preflight-card-template-health-row-proof/support_export.json`),
the machine-readable matrix CSV, the two scenario fixtures
(`fixtures/ui/m5-scaffold-preflight-card-template-health-row-controls/`), and the Markdown design
report are all regenerated deterministically from the canonical seed builders by the
`dump_scaffold_readiness_controls` example. Raw file bodies, raw secret values, pasted local
paths, repository URLs, credentials, and secrets never cross the export boundary.
