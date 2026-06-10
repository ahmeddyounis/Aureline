# Recorded-Macro Promotion, Recipe Insertion, And Headless-Safe Result Packets For User Automation

- Packet: `user-automation:stable:0001`
- Schema: `schemas/ai/add-recorded-macro-promotion-recipe-insertion-and-headless-safe-result-packets-for-user-automation.schema.json`
- Support export: `artifacts/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation/support_export.json`
- Fixture: `fixtures/ai/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation/`
- Contract: `docs/automation/m5/add_recorded_macro_promotion_recipe_insertion_and_headless_safe_result_packets_for_user_automation.md`

## Coverage

The packet carries user-authored automation into one row per recorded macro. Every
row binds the content-addressed capture, the promotion that graduates the macro
into a reusable recipe, how that recipe inserts into a target surface, the
headless-safe result it produces, and a step disclosure for every effect it can
produce.

- The format-and-organize-imports macro resolves to the local mode at Stable:
  recorded from a user session and promoted into a recipe behind a one-time
  promotion prompt, with an inspect-only read and a reversible edit pre-authorized
  to run headless under policy that previews a diff first, is one-time approved, and
  is audited to the run-record timeline. It inserts into the automation queue with a
  preview, its headless run completed every step safely, and its downgrade rules
  narrow to Beta on stale proof and to Unavailable on provider outage.
- The release-notes publish macro resolves to the managed mode at Beta: imported
  from a signed recipe pack and promoted behind an admin approval, with an
  inspect-only read and an irreversible external publish deferred to interactive
  review when run headless, previewing a diff, requiring admin approval, and audited
  to the support export. It inserts into the composer for review, its headless run
  completed with the publish deferred, and its downgrade rules narrow to Preview and
  Held.
- The symbol-tour macro resolves to the local mode at Preview: recorded from a user
  session and pending promotion review, inspect-only with no side effect, inserted
  from the command palette, completing headless with no change applied, and
  narrowing to Experimental.
- The deploy-trigger macro resolves to BYOK but claims Held: its capture is tainted,
  so its promotion is blocked, its irreversible publish is denied by policy and held
  back fail-closed when run headless, its headless result is blocked fail-closed, and
  every downgrade rule narrows to Unavailable.

## Invariants

The support export validates against the same closed rule set the shell, docs, and
release tooling enforce: every mutating step previews before it applies, gates, and
audits; an inspect-only step is headless-safe inspect-only and a mutating step never
claims it; an irreversible publish never runs unattended headless; a step
pre-authorized to run headless still carries a gate and is audited; a promoted macro
names its recipe and an unpromoted one names none; promoting a mutating macro is
gated; insertion is preview-first and a headless target does not rely on a recurring
interactive prompt; the headless result reconciles its step counts and state and is
externally audited when a mutating step ran; a blocked promotion drops its public
claim; and every claimed automation carries evidence, a verified reversible
rollback, and the proof-stale and provider-unavailable downgrade triggers.

## Boundary

The packet carries content addresses, classes, and review-safe labels only. Raw
shell fragments, raw filesystem paths, raw endpoint URLs, credential bodies, raw
API keys, OAuth tokens, and raw captured buffer bytes never cross this boundary.
