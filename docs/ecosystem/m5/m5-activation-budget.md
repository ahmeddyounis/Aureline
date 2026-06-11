# M5 activation budget

This document describes the canonical packet that freezes the **M5 activation
budget and exercised-capability** truth — per session, what each marketed M5
artifact family actually cost to activate and which of its declared capabilities it
actually exercised. It is the user-facing companion to the governed artifact at
`artifacts/ecosystem/m5/m5-activation-budget.json` and the typed model in the
`aureline-ecosystem` crate (`m5_activation_budget`).

Where the
[`M5 ecosystem install-governance matrix`](m5-ecosystem-install-governance-matrix.md)
freezes one governance row per family — including the family's published
activation-budget band — the [`M5 marketplace fact-views`](m5-marketplace-fact-views.md)
project that truth into the storefront, the [`M5 install/update review sheets`](m5-install-review.md)
freeze how an install or update is reviewed before commit, and the
[`M5 lifecycle actions`](m5-lifecycle-actions.md) freeze what happens to a package
after install, this packet freezes the **operational** dimension: runtime cost and
*actually used* capability, so ecosystem trust covers operational cost, not just
permissions and features.

## Activation cost is explicit, not a slow-extension toast

Each record names the runtime cost of one session:

- `activation_bucket` — `cold` (paid full start-up cost) or `warm` (reused a running
  runtime);
- `activation_trigger` — the lazy or eager trigger: `eager_on_startup`,
  `on_workspace_open`, `on_language_match`, `on_command_invoke`, `on_view_open`, or
  `manual`. Every trigger except `eager_on_startup` defers activation until the
  package is genuinely needed;
- `activation_budget_band` — the published band (`healthy_under_budget`,
  `approaching_ceiling`, `over_budget`, `budget_unknown`, `not_applicable`), reused
  from the governance matrix;
- `cold_start_pressure` and `memory_pressure` — `ResourcePressure` for each measured
  resource (`healthy`, `elevated`, `over_budget`, `unknown`, `not_applicable`);
- `restart_budget` — restarts permitted and used within the window plus a
  `crash_loop_detected` flag; and
- `runtime_host_class` — the host the runtime is bound to (`local`,
  `managed_workspace`, `remote_host`, `container`).

A reviewer or admin sees operational cost on the same surface as permissions and
features, for every marketed M5 artifact family.

## Declared versus exercised capability

Each record lists `declared_capabilities` (the manifest grant set) and a per-class
`exercised_capabilities` row whose `exercise_state` is one of:

- `declared_exercised` — declared and actually used this session;
- `declared_unused` — declared but never used — an **over-grant candidate** an admin
  may want to revoke; and
- `undeclared_exercised` — exercised without ever being declared — a **policy
  violation**.

The model is consistent by construction: a `declared_exercised` or `declared_unused`
row must be backed by a declared capability, an `undeclared_exercised` row must not
be, and every declared capability must carry exactly one usage row, so a declared
grant can never go unaccounted for. This is how users, admins, and support see what
permissions were *actually used* versus merely declared.

## Enforcement is recomputed, not stored by hand

When a session exceeds an activation, cold-start, memory, restart, crash-loop, or
undeclared-capability rule, the record names the exact `enforcement_reasons` and the
`enforcement_action` taken. Both are **recomputed** from the record's facts, and the
stored values must equal that recomputation or validation fails. Each reason forces a
minimum action and the record takes the strictest:

| Enforcement reason | Minimum action |
| --- | --- |
| `activation_budget_exceeded` | `throttled` |
| `cold_start_budget_exceeded` | `throttled` |
| `memory_budget_exceeded` | `downgraded` |
| `restart_budget_exhausted` | `paused` |
| `crash_loop_detected` | `quarantined` |
| `undeclared_capability_exercised` | `quarantined` |

`EnforcementAction` widens monotonically: `no_action` < `throttled` < `downgraded` <
`paused` < `quarantined`. A healthy session with every declared capability accounted
for takes `no_action`; an over-budget activation, memory pressure, an exhausted
restart budget, a crash loop, or an undeclared exercised capability routes through an
explicit action rather than a generic performance warning.

## Every enforced session names a way back

A record that takes any action other than `no_action` must carry a
`recovery_path_ref`, and a `no_action` record must not. So a throttle, downgrade,
pause, or quarantine always names a recovery path and an exact reason code — never a
bare banner — and a clean session stays clean.

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with each record's package
kind, scope, runtime host class, runtime origin, activation bucket and trigger,
activation-budget band, cold-start and memory pressure, restart counts, crash-loop
flag, declared-capability tokens, exercised-capability usage tokens, over-grant and
undeclared-exercise counts, enforcement action and reason tokens, and recovery-path
ref, plus an `intervention_count` and an `undeclared_exercised_count`. Support
bundles, admin audits, and release evidence ingest this projection directly, so field
triage uses the same activation-budget and exercised-capability vocabulary shown
in-product without scraping logs or depending on a private diagnostics build.

## Validation

`M5ActivationBudget::validate()` reports every violation, including an unsupported
schema version or record kind, non-canonical closed vocabularies, empty required
fields, duplicate record ids, a declared capability listed more than once, a
duplicate capability usage row, a usage state that disagrees with the declared
manifest, a declared capability with no usage row, an enforced record missing its
recovery path, an unimpeded record that still carries one, an enforcement-reason set
or action that disagrees with the recomputation, and a summary block that disagrees
with the records.
