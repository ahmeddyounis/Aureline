# M5 lifecycle actions

This document describes the canonical packet that freezes the **M5 lifecycle
actions** — disable, uninstall, rollback, quarantine, re-enable, and registry-status
transitions for marketed M5 artifact families, modeled as distinct reviewed actions.
It is the user-facing companion to the governed artifact at
`artifacts/ecosystem/m5/m5-lifecycle-actions.json` and the typed model in the
`aureline-ecosystem` crate (`m5_lifecycle_actions`).

Where the
[`M5 ecosystem install-governance matrix`](m5-ecosystem-install-governance-matrix.md)
freezes one governance row per marketed artifact family, the
[`M5 marketplace fact-views`](m5-marketplace-fact-views.md) project that truth into
the storefront, and the [`M5 install/update review sheets`](m5-install-review.md)
freeze how an install or update is **reviewed before commit**, this packet freezes
what happens to a package **after** install: how it is disabled, uninstalled, rolled
back, quarantined, re-enabled, or moved through a registry-status change such as
revocation, yank, deprecation, or publisher transfer.

## Distinct actions, distinct states

Each record names one `action_kind` and the `resulting_status` it resolves to:

| Action kind | Scope rule | Resulting status |
| --- | --- | --- |
| `disable_workspace` | bound to `workspace` | `disabled_workspace` |
| `disable_global` | bound to `global` | `disabled_global` |
| `uninstall` | any scope | `uninstalled` |
| `rollback` | any scope | `rolled_back` |
| `quarantine` | any scope | `quarantined` |
| `reenable` | any scope | `active` |
| `apply_registry_status` | any scope | `revoked` / `yanked` / `deprecated` / `publisher_transferred` (keyed by trigger) |

Disable for a workspace and disable globally are deliberately separate actions bound
to separate scopes, so a local troubleshooting moment can never silently disable a
package everywhere. A registry-status action pairs each status with its matching
trigger: `revoked`⇐`revocation`, `yanked`⇐`yank`, `deprecated`⇐`deprecation`, and
`publisher_transferred`⇐`publisher_transfer`.

## Continuity: what already-open work does

Every record states what already-open views and background work do through its
`continuity` field, naming the affected M5-contributed surfaces in
`contributed_surface_refs`:

- `keeps_running_temporarily` — open views keep running until the next activation
  boundary;
- `stops_immediately` — open views stop at once;
- `converts_to_placeholder_at_next_activation` — open views convert to placeholders
  at the next activation boundary; and
- `not_applicable` — no open views or background work are affected.

This keeps contributed surfaces from silently disappearing: a globally disabled docs
pack converts its viewers to placeholders rather than vanishing, and a crash-looped
framework pack stops its surfaces immediately.

## Retained state: the no-silent-deletion guardrail

Each record lists per-class `retained_state` impacts. A `data_class` is one of
`user_recipes`, `docs_pack_content`, `local_history`, `rollback_checkpoints`,
`workspace_settings`, `cached_indexes`, `model_weights`, or `generated_placeholders`;
its `disposition` is `retained`, `removed_with_explicit_consent`, or
`regenerable_cache`.

The first four classes are **protected** user-owned data. The lane guardrail is that
an uninstall or disable can never silently delete protected data: if a protected
class is removed and `destructive_consent_captured` is false, the action is
**blocked** (`unconsented_protected_data_removal`). If the removal carries captured
consent, it is disclosed and **reviewed** (`disclosed_protected_data_removal`) rather
than silent. Recipes, docs packs, local history, and rollback checkpoints therefore
survive a lifecycle action unless the user has explicitly consented to their removal.

## Rollback and restore are never implied risk free

Each record carries a `rollback` note with a `rollback_compatibility`
(`clean`, `requires_recompat`, `state_loss_possible`, `not_reversible`,
`not_applicable`), a `rollback_posture`, the `last_known_good_ref` target, and a
`retained_state_note_ref`. A `rollback` action must name a `last_known_good_ref` and
a non-`not_applicable` compatibility. A `not_reversible` rollback is **blocked**
(`rollback_not_reversible`); a `requires_recompat` or `state_loss_possible` rollback
is **reviewed** (`rollback_not_risk_free`). The record's `restorable` flag and
`restore_path_ref` state whether and how the package can be brought back; a `revoked`
or `yanked` package is never restorable on its own.

## The disposition is recomputed, not stored by hand

The `action_disposition` a record publishes — `proceed_allowed`, `review_required`,
or `blocked` — and its `review_reasons` set are **not** hand-entered. They are
recomputed from the record's facts, and the stored values must equal that
recomputation or validation fails. Each reason forces a minimum disposition and the
record takes the strictest:

- **`unconsented_protected_data_removal`** / **`rollback_not_reversible`** force
  `blocked`.
- **`automated_health_trigger`** (crash loop, integrity failure, performance budget),
  **`moderation_or_policy_trigger`**, **`registry_status_trigger`**,
  **`publisher_transfer_trigger`**, **`open_work_disruption`**,
  **`irreversible_action`**, **`rollback_not_risk_free`**, and
  **`disclosed_protected_data_removal`** force `review_required`.

This routes crash-loop, integrity-failure, performance-budget, moderation, policy,
revocation, yank, deprecation, and publisher-transfer events through explicit
lifecycle states rather than generic warning banners: any reactive trigger forces at
least `review_required`. A clean user-initiated disable, a clean rollback, or a
re-enable with full retention stays `proceed_allowed`.

## Scopes stay visibly distinct

Each record declares a `scope`, and **every** `offered_actions` entry must carry that
same scope. A blocked record never exposes an enabled primary action, so a blocked
uninstall or irreversible rollback cannot be triggered from the surface.

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with each record's package
kind, action kind, scope, trigger, resulting status, continuity, action disposition,
review-reason tokens, rollback compatibility, last-known-good target, whether the
package is restorable, and whether protected data is preserved, plus
`review_required_count` and `blocked_count`. Support bundles and docs/help ingest
this projection directly so the product, support exports, and release evidence can
say *why* a package or pack is degraded, disabled, revoked, quarantined, or
rollback-available without scraping logs.

## Validation

`M5LifecycleActions::validate()` reports every violation, including an unsupported
schema version or record kind, non-canonical closed vocabularies, empty required
fields, duplicate record ids, an action kind that disagrees with its bound scope, a
resulting status that disagrees with its action kind, a registry-status/trigger
pairing mismatch, a quarantine carrying a registry/publisher trigger, a revoked or
yanked record that claims to be restorable, a rollback missing its last-known-good
target, an offered action scoped outside its record's scope, a missing primary
action, a blocked record that still enables its primary action, a review-reason set
or action disposition that disagrees with the recomputation, and a summary block that
disagrees with the records.
