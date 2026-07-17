# M5 change-intent accessibility & auto-narrowing parity (M05-1292)

This contract is the accessibility-and-auto-narrowing capstone over the frozen M5 change-intent and
engineering-lifecycle matrix (`m5_change_intent_and_engineering_lifecycle_matrix`). Where the freeze matrix
defines the reusable **change-intent record, start-work sheet, linked-change panel, ready-for-review handoff
sheet, resolve-close sheet, and blocked-escalate card** objects, and the 1285–1291 implementation lanes
resolve their per-surface truth, this lane certifies — per object — that every change-intent, start-work,
handoff, resolve, and blocker claim survives beyond the pointer-rich desktop view and **auto-narrows when its
provider-commit-state / side-effect-disclosure / linked-relation / handoff-publishability /
resolution-authority / blocker-continuity proof weakens**.

- **Module:** `crates/aureline-ui/src/m5_change_intent_accessibility_parity_and_narrowing_when_linked_branch_review_provider_scope_publishability_or_reconcile_state_is_stale/`
- **Schema:** `schemas/teamwork/m5-change-intent-accessibility-parity.schema.json`
- **Release proof:** `artifacts/release/m5-change-intent-accessibility-parity/`
  (`support_export.json`, `matrix.csv`) and `…-accessibility-parity.md`
- **Fixtures:** `fixtures/teamwork/m5-change-intent-accessibility-parity/`

## What the packet guarantees

1. **Non-visual + exported representations.** Every object exposes a keyboard-complete,
   screen-reader-reachable, high-zoom-legible, high-contrast-safe, and CLI/headless-reachable path into the
   same object identity, provider ownership, local-versus-provider commit state, linked branch / worktree /
   review identity, relation source, publishability, resolution authority, and blocker state the rich object
   shows — never a color-only relation badge, a hover-only authority pill, or a pointer-only publish
   affordance. The support / release / CLI export reconstructs each object's meaning from typed tokens and
   opaque refs **without a raw payload**, preserving the same provider ownership, commit state, relation
   source, publishability, resolution authority, and blocker labels visible in-product.

2. **Honest auto-narrowing.** When a change-intent commit state is local-only or reconcile-required, a
   start-work side effect is undisclosed, a linked branch / worktree / review relation is stale or broken,
   provider write scope is missing so a handoff packet is not publishable, a resolution is local-only, or a
   blocker is unresolved, the claim auto-narrows from `trusted_provider_committed_surface` /
   `local_reviewable_surface` to the matching projection, discloses the narrowing with a precise trigger and
   binding dimension, and preserves the canonical identity / last-known state. An object with every dimension
   intact must **not** carry a spurious narrowing, and a weakened object can never keep a fully
   provider-committed, publish-safe claim — a local handoff packet or queued publish never masquerades as a
   provider-committed update, and linked-by-provider, linked-locally, suggested, and stale-or-broken relations
   are never flattened into one badge.

3. **Cross-surface disclosure.** The same narrowed state surfaces in the work-item detail, start-work sheet,
   linked-change panel, review detail, ready-for-review handoff, resolve-close sheet, blocked-escalate card,
   support / export packet, and help / docs so product, help, and release publication stay aligned on
   downgrade behavior rather than drifting in copy.

## Claim tiers (strongest → weakest)

| Claim | Meaning |
| --- | --- |
| `trusted_provider_committed_surface` | Fully provider-owned, commit-stated, linked, publishable, resolution-authoritative, blocker-clear — publish-safe to start work, hand off, reopen, and export. |
| `local_reviewable_surface` | Self-sufficient, locally reviewable read-only object (a blocked-escalate card a user can inspect), not a provider-committed surface. |
| `provider_commit_state_unverified_projection` | The change-intent record's commit state is local-only / reconcile-required (change-intent-record). |
| `side_effect_disclosure_unverified_projection` | A create-branch / worktree / review-draft / provider-link side effect is undisclosed (start-work-sheet). |
| `linked_relation_unverified_projection` | A linked branch / worktree / review relation is stale or broken (linked-change-panel). |
| `handoff_publishability_unverified_projection` | The handoff packet is not publishable — offline / missing write scope / policy-blocked / partial (ready-for-review-handoff-sheet). |
| `resolution_authority_unverified_projection` | The resolution is local-only, not provider-accepted (resolve-close-sheet). |
| `blocker_continuity_unverified_projection` | The blocker is unresolved / a local handoff packet must not read as a provider escalation (blocked-escalate-card). |

## Weakening dimensions and their frozen triggers

Each object maps to a claim dimension; a weak condition state narrows to the matching projection and names
the on-topic frozen matrix downgrade trigger:

| Dimension (object) | Weak condition | Frozen trigger | Cannot be shown trusted |
| --- | --- | --- | --- |
| `provider_commit_state_clarity` (change-intent-record) | `local_only_or_reconcile_required` | `local_versus_provider_state_unstated` | yes |
| `side_effect_disclosure_clarity` (start-work-sheet) | `side_effect_undisclosed` | `silent_side_effect_created` | yes |
| `linked_relation_source_clarity` (linked-change-panel) | `linked_relation_stale_or_broken` | `relation_source_unstated` | yes |
| `handoff_publishability_clarity` (ready-for-review-handoff-sheet) | `handoff_publishability_blocked` | `local_handoff_shown_as_provider_committed` | yes |
| `resolution_authority_clarity` (resolve-close-sheet) | `resolution_authority_local_only` | `auto_resolved_with_open_blocker` | yes |
| `blocker_continuity_clarity` (blocked-escalate-card) | `blocker_unresolved_or_masquerade` | `blocker_state_unstated` | yes |

Every weak change-intent condition is a genuine truth degradation, so all six flag as
`cannot_be_shown_trusted`: none may keep a fully provider-committed, publish-safe claim.

## Structure-heavy objects

The **linked-change panel** (linked branch / worktree / hosted-review relation rows) and **blocked-escalate
card** (blocker class / dependency / escalation set) render a dense structured surface, so they must
additionally bind their structured layout to an equivalent flat list / textual path (a `structured` fallback
modality **plus** a non-visual list / textual / CLI path).

## Certified rows

Eight rows across the six objects: **2 green** (the provider-committed change-intent record — trusted; and the
continuity-bound blocked-escalate card — locally reviewable) and **6 yellow** — one per spec narrowing axis
(provider commit state, side-effect disclosure, linked-relation source, handoff publishability, resolution
authority, blocker continuity), each auto-narrowing to its permitted projection. **No red rows may ship.**

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To regenerate
after an intentional change:

```
GEN_CHANGE_INTENT_A11Y_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_change_intent_accessibility_parity_and_narrowing_when_linked_branch_review_provider_scope_publishability_or_reconcile_state_is_stale::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
