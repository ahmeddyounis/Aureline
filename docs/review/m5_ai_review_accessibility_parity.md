# M5 AI-review-assist accessibility & auto-narrowing parity (M05-1272)

This contract is the accessibility-and-auto-narrowing capstone over the frozen M5 AI-review-assist matrix
(`m5_ai_review_assist_matrix`). Where the freeze matrix defines the reusable **AI review finding row, review
scope selector, publish-to-review sheet, and resolution memory row** objects, and the 1266–1270
implementation lanes resolve their per-surface truth, this lane certifies — per object — that every AI
review claim survives beyond the pointer-rich desktop view and **auto-narrows when its provider-freshness /
diff-scope / publish-target / finding-lifecycle proof weakens**.

- **Module:** `crates/aureline-ui/src/m5_ai_review_accessibility_parity_and_narrowing_when_provider_freshness_diff_drift_publish_target_or_finding_lifecycle_state_is_stale/`
- **Schema:** `schemas/review/m5-ai-review-accessibility-parity.schema.json`
- **Release proof:** `artifacts/review/m5-ai-review-accessibility-parity/`
  (`support_export.json`, `matrix.csv`) and `…-accessibility-parity.md`
- **Fixtures:** `fixtures/review/m5-ai-review-accessibility-parity/`

## What the packet guarantees

1. **Non-visual + exported representations.** Every object exposes a keyboard-complete,
   screen-reader-reachable, high-zoom-legible, high-contrast-safe, and CLI/headless-reachable path into the
   same object identity, finding class / severity, analyzed diff scope, publish mode / provider destination,
   local-versus-provider state, and finding lifecycle state the rich object shows — never a color-only
   finding badge, a hover-only scope chip, or a pointer-only publish action. The support / release / CLI
   export reconstructs each object's meaning from typed tokens and opaque refs **without a raw payload**,
   preserving the same analyzed scope, destination class, and finding lifecycle labels visible in-product.

2. **Honest auto-narrowing.** When provider freshness is stale, diff drift invalidates prior findings, a
   publish target is unavailable, or a finding's lifecycle state falls outside live publish-safe conditions,
   the claim auto-narrows from `trusted_review_surface` / `reviewable_review_surface` to the matching
   projection, discloses the narrowing with a precise trigger and binding dimension, and preserves the
   canonical identity / last-known state. An object with every dimension intact must **not** carry a
   spurious narrowing, and a weakened object can never keep a trusted, publish-safe claim — AI review never
   auto-approves, auto-requests changes, or auto-merges, and a lost local draft never masquerades as a
   provider-committed publish.

3. **Cross-surface disclosure.** The same narrowed state surfaces in the review detail, AI review panel,
   finding row, review scope selector, publish-to-review sheet, pending-review tray, provider publish
   review, resolution memory ledger, and support / export packet so product, help, and release publication
   stay aligned on downgrade behavior rather than drifting in copy.

## Claim tiers (strongest → weakest)

| Claim | Meaning |
| --- | --- |
| `trusted_review_surface` | Fully fresh, diff-scoped, publish-target-available, live-lifecycle — publish-safe to inspect, rerun, dismiss, publish, export, or reopen. |
| `reviewable_review_surface` | Self-sufficient, reviewable read-only object (a resolution memory row a user can inspect), not an authoritative publish-driving surface. |
| `provider_freshness_unverified_projection` | The finding's provider freshness is stale (ai-review-finding-row). |
| `diff_scope_unverified_projection` | Diff drift invalidates prior findings (review-scope-selector). |
| `publish_target_unverified_projection` | The provider publish target is unavailable (publish-to-review-sheet). |
| `finding_lifecycle_unverified_projection` | The finding's lifecycle state is outdated / suppressed, outside live publish-safe conditions (resolution-memory-row). |

## Weakening dimensions and their frozen triggers

Each object maps to a claim dimension; a weak condition state narrows to the matching projection and names
the on-topic frozen matrix downgrade trigger:

| Dimension (object) | Weak condition | Frozen trigger | Cannot be shown trusted |
| --- | --- | --- | --- |
| `provider_freshness_clarity` (ai-review-finding-row) | `provider_freshness_stale` | `stale_finding_shown_as_current` | yes |
| `diff_scope_drift_clarity` (review-scope-selector) | `diff_drift_invalidates_findings` | `analyzed_scope_unstated` | yes |
| `publish_target_availability_clarity` (publish-to-review-sheet) | `publish_target_unavailable` | `publish_mode_unstated` | yes |
| `finding_lifecycle_clarity` (resolution-memory-row) | `lifecycle_outside_publish_safe` | `lifecycle_state_missing` | yes |

Every weak AI-review condition is a genuine truth degradation, so all four flag as
`cannot_be_shown_trusted`: none may keep a trusted, publish-safe review claim.

## Structure-heavy objects

The **publish-to-review sheet** (outbound action set) and **resolution memory row** (durable lifecycle
history) render a dense structured surface, so they must additionally bind their structured layout to an
equivalent flat list / textual path (a `structured` fallback modality **plus** a non-visual list / textual /
CLI path).

## Certified rows

Six rows across the four objects: **2 green** (the fresh, scoped finding row — trusted; and the live-
lifecycle resolution memory row — reviewable) and **4 yellow** — one per spec narrowing axis (provider
freshness, diff drift, publish target, finding lifecycle), each auto-narrowing to its permitted projection.
**No red rows may ship.**

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To regenerate
after an intentional change:

```
GEN_AI_REVIEW_A11Y_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_ai_review_accessibility_parity_and_narrowing_when_provider_freshness_diff_drift_publish_target_or_finding_lifecycle_state_is_stale::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
