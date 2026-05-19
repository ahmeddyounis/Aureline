# Dependency Marker Downgrade Guidance

This document is the migration-facing guidance for capability
dependency markers when an artifact moves to a target that lacks the
required capability. It exists so support, docs, product, and shiproom
can decide, from one shared vocabulary, what happens to the user's
artifact on every downgrade scenario M3 supports — and what the user
needs to do to recover.

The contract is defined in
[`docs/ux/m3/capability_dependency_marker_beta.md`](../../ux/m3/capability_dependency_marker_beta.md).
The release-evidence packet that proves this guidance against the
fixture corpus lives at
[`artifacts/release/m3/dependency_marker_conformance_report.md`](../../../artifacts/release/m3/dependency_marker_conformance_report.md).
The runtime evaluator lives at
[`crates/aureline-capabilities/src/dependency_markers/downgrade.rs`](../../../crates/aureline-capabilities/src/dependency_markers/downgrade.rs).

## Principles

- **No silent narrowing.** Every downgrade scenario produces a
  [`CompareApplyReviewSheet`](../../../crates/aureline-capabilities/src/dependency_markers/downgrade.rs)
  row before apply. Apply is held until the user sees the row.
- **No silent drop.** Every `effective_effect_on_import` is one of the
  five closed `_preserve_data` variants. The user's authored data is
  never lost; it is either rendered, held, narrowed, blocked, or
  tombstoned, with the source preserved as the row of truth.
- **No silent upgrade.** The
  [`support_rank`](../../../crates/aureline-capabilities/src/dependency_markers/downgrade.rs)
  ordering refuses to elevate a recorded support promise. Promotions
  only happen for the explicit `preview_to_stable` scenario, and only
  to (at least) standard support.
- **Same vocabulary on every surface.** Settings inspectors, import
  review sheets, bundle detail pages, downgrade flows, headless / CLI
  inspect output, and docs / help pages render the same review sheet
  fields. The closed scenario, effect, and support vocabulary never
  drift across surfaces.

## Scenario guidance

The table below is the full, closed map of downgrade scenarios M3
supports. The runtime evaluator returns these effective values for
every marker; downstream UI may quote the `portability_consequence`
copy verbatim but MUST NOT narrow the effective values further.

### `stable_to_preview`

Source observed the capability as stable; target only ships it as a
preview cohort.

- **Effective effect on import:** `emulated_downgrade_preserve_data`
  (or `block_apply_preserve_data` if the recorded effect already
  blocked apply).
- **Effective support promise:** narrows down to `best_effort`. A
  recorded promise weaker than `best_effort` (e.g. `no_support`) is
  preserved untouched.
- **Apply held:** yes.
- **Recovery path:** open the capability in Settings → Preview, or
  apply the artifact on a target where the capability is stable. The
  original authored data is preserved.

### `preview_to_stable`

Source observed the capability as preview; target has promoted it to
stable.

- **Effective effect on import:** preserves the recorded effect; the
  lifecycle delta is still disclosed.
- **Effective support promise:** at least `standard_support`. A
  recorded promise stronger than `standard_support` (e.g.
  `extended_support`) is preserved untouched.
- **Apply held:** yes (until the user acknowledges the lifecycle
  promotion).
- **Recovery path:** apply once acknowledged. The recorded effect
  carries through.

### `host_change`

Target host is outside the marker's admitted scope.

- **Effective effect on import:** `render_tombstone_preserve_data`.
- **Effective support promise:** narrows to `no_support`.
- **Apply held:** yes.
- **Recovery path:** request access to a host that admits the
  dependency, or accept the tombstone (the source is preserved for
  migration disclosure).

### `mirror_only`

Target is applying from a curated mirror with no upstream control
plane.

- **Effective effect on import:** `narrow_behavior_preserve_data` (or
  `block_apply_preserve_data` if the recorded effect blocked apply).
- **Effective support promise:** narrows down to
  `community_supported`. Weaker promises stay weak.
- **Apply held:** yes.
- **Recovery path:** reconnect to upstream when the user can, or
  accept the narrowed behavior. Mirror-only operation is the safe
  bounded default.

### `offline_cache_only`

Target is applying from the local cache because no upstream lane is
reachable.

- **Effective effect on import:** `hold_for_later_preserve_data`.
- **Effective support promise:** narrows down to `best_effort`.
- **Apply held:** yes.
- **Recovery path:** wait for the upstream lane to recover. The
  artifact is held; the user's authored data is preserved in the
  review sheet.

### `policy_disabled`

Target admin policy disables the capability (active kill switch / policy
bundle).

- **Effective effect on import:** `block_apply_preserve_data`.
- **Effective support promise:** `no_support`.
- **Apply held:** yes (apply is blocked).
- **Recovery path:** request an admin policy change. The original
  payload is preserved in the review sheet for follow-up.

## Compare / apply review sheet fields

Every row the evaluator emits carries the same fields. Surfaces MUST
render every field; partial rendering is a contract violation.

| Field | Meaning |
| --- | --- |
| `scenario` | One of the six closed scenario tokens above. |
| `marker_id` / `artifact_ref` / `artifact_class` | Source marker identity preserved bit-for-bit. |
| `required_capability_id` | Capability id the marker depends on. |
| `dependency_class` | One of `labs`, `preview`, `beta_only`, `policy_gated`, `host_specific`. |
| `producer_lifecycle_state` / `target_lifecycle_state` | Lifecycle state at source and target. |
| `recorded_support_promise` / `effective_support_promise` | Source promise + (narrowed) effective promise on the target. |
| `recorded_effect_on_import` / `effective_effect_on_import` | Source effect + (narrowed) effective effect on the target. |
| `portability_consequence` | Short reviewer-facing copy describing what the user observes. |
| `safe_fallback` | Bounded recover / dismiss / wait path. Same string the marker carried. |
| `apply_held_until_disclosed` | Always `true` for every downgrade row. |
| `support_claim_narrowed` | `true` when the effective promise is weaker than the recorded promise. |
| `kill_switch_active` | `true` when an active kill switch / policy disable narrowed the dependency. |
| `user_authored_data_preserved` | Always `true`; the closed effect vocabulary forbids drop. |

## Support / shiproom checklist

When a row of the dependency-marker fixture corpus regresses, the
matrix in
[`artifacts/release/m3/dependency_marker_conformance_report.md`](../../../artifacts/release/m3/dependency_marker_conformance_report.md)
tells shiproom whether the regression should:

- block the release (silent drop, vocabulary drift, or apply happening
  without disclosure),
- narrow the support claim of the affected artifact path (effective
  support weaker than the recorded support without
  `support_claim_narrowed = true`), or
- narrow the lifecycle claim (a beta-claimed artifact path that no
  longer survives all transport lanes drops back to alpha or labs in
  the release-evidence claim manifest).

Silent marker loss on any lane is a release-truth bug. Prefer
narrowing the claim over hiding lifecycle-sensitive dependencies.

## Out of scope

- Full ecosystem-wide dependency introspection beyond the closed
  Aureline-owned artifact families.
- Live RPC of raw provider tokens, raw policy-bundle bytes, or raw
  kill-switch material on any downgrade scenario.
- UX rollout decisions (the review-sheet record is data-only; product
  UI composes it with the existing import-review and downgrade
  surfaces).
