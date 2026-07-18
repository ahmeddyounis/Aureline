# M5 change-orchestration accessibility & auto-narrowing parity (M05-1302)

This contract is the accessibility-and-auto-narrowing capstone over the frozen M5 change-object, patch-stack,
and landing matrix (`m5_change_object_patch_stack_and_landing_matrix`). Where the freeze matrix defines the
reusable **change object, patch-stack queue, stack-edit review sheet, landing-candidate sheet, portable shelf,
and worktree-cleanup preview** objects, and the 1295–1301 implementation lanes resolve their per-surface
truth, this lane certifies — per object — that every stack, landing, shelf, and cleanup claim survives beyond
the pointer-rich desktop view and **auto-narrows when its selected-change-binding / stack-membership /
stack-order / landing-authority / validation-freshness / cleanup-evidence proof weakens**.

- **Module:** `crates/aureline-ui/src/m5_change_orchestration_accessibility_parity_and_narrowing_when_stack_membership_validation_queue_authority_protected_branch_or_orphan_cleanup_evidence_is_stale/`
- **Schema:** `schemas/teamwork/m5-change-orchestration-accessibility-parity.schema.json`
- **Release proof:** `artifacts/release/m5-change-orchestration-accessibility-parity/`
  (`support_export.json`, `matrix.csv`) and `…-accessibility-parity.md`
- **Fixtures:** `fixtures/teamwork/m5-change-orchestration-accessibility-parity/`

## What the packet guarantees

1. **Non-visual + exported representations.** Every object exposes a keyboard-complete,
   screen-reader-reachable, high-zoom-legible, high-contrast-safe, and CLI/headless-reachable path into the
   same object identity, selected worktree / base binding, stack membership source, stack order, landing
   state, validation freshness, and cleanup safety the rich object shows — never a color-only stack-dependency
   chip, a hover-only landing-authority pill, or a pointer-only cleanup affordance. The support / release / CLI
   export reconstructs each object's meaning from typed tokens and opaque refs **without a raw payload**,
   preserving the same worktree binding, stack membership, stack order, landing state, validation freshness,
   and cleanup-safety labels visible in-product.

2. **Honest auto-narrowing.** When a change object's selected worktree binding is unbound, a patch-stack
   queue's membership is inferred or unverifiable, a stack-edit review sheet's order is drifted or
   restack-required, a landing candidate's queue authority or protected-branch rule cannot be proven, a
   portable shelf's validation or approval evidence is stale, or a worktree-cleanup preview's evidence is
   partial, the claim auto-narrows from `trusted_provider_landed_surface` / `local_reviewable_surface` to the
   matching projection, discloses the narrowing with a precise trigger and binding dimension, and preserves the
   canonical identity / last-known state. An object with every dimension intact must **not** carry a spurious
   narrowing, and a weakened object can never keep a fully provider-authoritative, landed claim — a local
   landing estimate never masquerades as a provider-authoritative land, ambient branch state is never shown as
   a reviewed landing candidate, and stack membership is never inferred from branch names alone.

3. **Cross-surface disclosure.** The same narrowed state surfaces in the change-object detail, patch-stack
   queue, stack-edit review sheet, review detail, provider merge queue, portable shelf, worktree-cleanup
   preview, support / export packet, and help / docs so product, help, and release publication stay aligned on
   downgrade behavior rather than drifting in copy.

## Claim tiers (strongest → weakest)

| Claim | Meaning |
| --- | --- |
| `trusted_provider_landed_surface` | Fully bound, stack-verified, order-fresh, landing-authoritative, validation-fresh, cleanup-safe — landed-safe to inspect, stack, land, reopen, and export. |
| `local_reviewable_surface` | Self-sufficient, locally reviewable read-only object (a worktree-cleanup preview a user can inspect), not a provider-authoritative surface. |
| `selected_change_binding_unverified_projection` | The change object's selected worktree / base binding is unbound (change-object). |
| `stack_membership_unverified_projection` | The patch-stack membership is inferred or unverifiable (patch-stack-queue). |
| `stack_order_unverified_projection` | The stack order is drifted or a restack is required (stack-edit-review-sheet). |
| `landing_authority_unverified_projection` | The queue authority or protected-branch rule cannot be proven (landing-candidate-sheet). |
| `validation_freshness_unverified_projection` | The validation or approval evidence is stale (portable-shelf). |
| `cleanup_evidence_unverified_projection` | The orphan-cleanup evidence is partial (worktree-cleanup-preview). |

## Weakening dimensions and their frozen triggers

Each object maps to a claim dimension; a weak condition state narrows to the matching projection and names
the on-topic frozen matrix downgrade trigger:

| Dimension (object) | Weak condition | Frozen trigger | Cannot be shown trusted |
| --- | --- | --- | --- |
| `selected_change_binding_clarity` (change-object) | `selected_change_binding_unbound` | `worktree_binding_unstated` | yes |
| `stack_membership_clarity` (patch-stack-queue) | `stack_membership_inferred_or_unverifiable` | `stack_membership_inferred_from_branch_name_alone` | yes |
| `stack_order_integrity_clarity` (stack-edit-review-sheet) | `stack_order_drifted_or_restack_required` | `stack_members_silently_reordered` | yes |
| `landing_authority_clarity` (landing-candidate-sheet) | `queue_or_protected_branch_unprovable` | `landed_from_ambient_branch_state` | yes |
| `validation_freshness_clarity` (portable-shelf) | `validation_or_approval_stale` | `validation_freshness_unstated` | yes |
| `cleanup_evidence_clarity` (worktree-cleanup-preview) | `cleanup_evidence_partial` | `orphan_deleted_without_safety_preview` | yes |

Every weak change-orchestration condition is a genuine truth degradation, so all six flag as
`cannot_be_shown_trusted`: none may keep a fully provider-authoritative, landed claim.

## Structure-heavy objects

The **stack-edit review sheet** (original / proposed order and parent-child link rows) and **worktree-cleanup
preview** (affected running work / uncommitted-change scope / recovery-checkpoint set) render a dense
structured surface, so they must additionally bind their structured layout to an equivalent flat list /
textual path (a `structured` fallback modality **plus** a non-visual list / textual / CLI path).

## Certified rows

Eight rows across the six objects: **2 green** (the selected-change-bound change object — trusted; and the
evidence-bound worktree-cleanup preview — locally reviewable) and **6 yellow** — one per spec narrowing axis
(selected-change binding, stack membership, stack order, landing authority, validation freshness, cleanup
evidence), each auto-narrowing to its permitted projection. **No red rows may ship.**

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To regenerate
after an intentional change:

```
GEN_CHANGE_ORCHESTRATION_A11Y_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_change_orchestration_accessibility_parity_and_narrowing_when_stack_membership_validation_queue_authority_protected_branch_or_orphan_cleanup_evidence_is_stale::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
