# M5 review-pack accessibility & auto-narrowing parity (M05-1282)

This contract is the accessibility-and-auto-narrowing capstone over the frozen M5 review-pack evaluator
matrix (`m5_review_pack_evaluator_matrix`). Where the freeze matrix defines the reusable **review-pack
record, ownership signal, required-evidence-check row, local-CI parity strip, AI policy hook, and
review-template packet** objects, and the 1275–1280 implementation lanes resolve their per-surface truth,
this lane certifies — per object — that every review-pack claim survives beyond the pointer-rich desktop view
and **auto-narrows when its pack-version / owner-provenance / evidence-check / local-parity / ai-pack-binding
/ template-attribution proof weakens**.

- **Module:** `crates/aureline-ui/src/m5_review_pack_accessibility_parity_and_narrowing_when_pack_version_digest_owner_provenance_parity_difference_or_template_attribution_is_stale/`
- **Schema:** `schemas/review/m5-review-pack-accessibility-parity.schema.json`
- **Release proof:** `artifacts/review/m5-review-pack-accessibility-parity/`
  (`support_export.json`, `matrix.csv`) and `…-accessibility-parity.md`
- **Fixtures:** `fixtures/review/m5-review-pack-accessibility-parity/`

## What the packet guarantees

1. **Non-visual + exported representations.** Every object exposes a keyboard-complete,
   screen-reader-reachable, high-zoom-legible, high-contrast-safe, and CLI/headless-reachable path into the
   same object identity, pack version / digest, owner provenance, evaluator result class,
   local-versus-provider parity state, pack freshness, and template attribution the rich object shows — never
   a color-only parity badge, a hover-only owner pill, or a pointer-only pack affordance. The support /
   release / CLI export reconstructs each object's meaning from typed tokens and opaque refs **without a raw
   payload**, preserving the same pack version / digest, owner provenance, parity state, and template
   attribution labels visible in-product.

2. **Honest auto-narrowing.** When a pack version / digest is stale, owner provenance is missing, a required
   check is unevaluated (ci-only / not-evaluated-here / provider-unavailable), a local parity estimate
   diverges from provider-authoritative state, an AI review runs under an undisclosed pack version, or a
   template attribution is stale, the claim auto-narrows from `trusted_review_surface` /
   `reviewable_review_surface` to the matching projection, discloses the narrowing with a precise trigger and
   binding dimension, and preserves the canonical identity / last-known state. An object with every dimension
   intact must **not** carry a spurious narrowing, and a weakened object can never keep a fully
   provider-aware, publish-safe claim — a local parity estimate never masquerades as provider-authoritative,
   and advisory-owner and enforced-owner are never flattened into one owner pill.

3. **Cross-surface disclosure.** The same narrowed state surfaces in the review detail, merge-readiness, AI
   review panel, provider handoff, review-pack summary, ownership overlay, local-CI parity strip, support /
   export packet, and help / docs so product, help, and release publication stay aligned on downgrade
   behavior rather than drifting in copy.

## Claim tiers (strongest → weakest)

| Claim | Meaning |
| --- | --- |
| `trusted_review_surface` | Fully pack-versioned, owner-provenanced, evidence-evaluated, parity-disclosed, pack-bound, template-attributed — publish-safe to inspect, rerun, compare, export, or reopen. |
| `reviewable_review_surface` | Self-sufficient, reviewable read-only object (a review-template packet a user can inspect), not an authoritative mergeability-driving surface. |
| `pack_version_unverified_projection` | The pack version / digest is stale (review-pack-record). |
| `owner_provenance_unverified_projection` | The advisory-versus-enforced owner provenance is missing (ownership-signal). |
| `evidence_check_unverified_projection` | A required check is unevaluated here — ci-only / not-evaluated-here / provider-unavailable (required-evidence-check-row). |
| `local_parity_unverified_projection` | A local parity estimate diverges from provider-authoritative state (local-ci-parity-strip). |
| `ai_pack_version_unverified_projection` | The AI review ran under an undisclosed or different pack version (ai-policy-hook). |
| `template_attribution_unverified_projection` | The comment / summary template attribution is stale (review-template-packet). |

## Weakening dimensions and their frozen triggers

Each object maps to a claim dimension; a weak condition state narrows to the matching projection and names
the on-topic frozen matrix downgrade trigger:

| Dimension (object) | Weak condition | Frozen trigger | Cannot be shown trusted |
| --- | --- | --- | --- |
| `pack_version_digest_clarity` (review-pack-record) | `pack_version_digest_stale` | `pack_version_or_digest_dropped` | yes |
| `owner_provenance_clarity` (ownership-signal) | `owner_provenance_missing` | `owner_provenance_unstated` | yes |
| `evidence_check_state_clarity` (required-evidence-check-row) | `evidence_check_unevaluated` | `unevaluated_check_hidden_behind_green_summary` | yes |
| `local_provider_parity_clarity` (local-ci-parity-strip) | `local_parity_capability_difference` | `local_estimate_shown_as_provider_authoritative` | yes |
| `ai_pack_binding_clarity` (ai-policy-hook) | `ai_pack_version_undisclosed` | `ai_review_ran_under_undisclosed_pack_version` | yes |
| `template_attribution_clarity` (review-template-packet) | `template_attribution_stale` | `template_attribution_dropped` | yes |

Every weak review-pack condition is a genuine truth degradation, so all six flag as
`cannot_be_shown_trusted`: none may keep a fully provider-aware, publish-safe review claim.

## Structure-heavy objects

The **required-evidence-check row** (required evidence / check set) and **review-template packet** (rationale
blocks / checklist / bundle manifest) render a dense structured surface, so they must additionally bind their
structured layout to an equivalent flat list / textual path (a `structured` fallback modality **plus** a
non-visual list / textual / CLI path).

## Certified rows

Eight rows across the six objects: **2 green** (the fresh, scoped review-pack record — trusted; and the
attribution-bound review-template packet — reviewable) and **6 yellow** — one per spec narrowing axis (pack
version / digest, owner provenance, evidence-check state, local-versus-provider parity, AI pack binding,
template attribution), each auto-narrowing to its permitted projection. **No red rows may ship.**

## Regenerating the artifacts

The checked-in support export, CSV, report, and fixtures are byte-locked to the seed builder. To regenerate
after an intentional change:

```
GEN_REVIEW_PACK_A11Y_ARTIFACTS=1 cargo test -p aureline-ui \
  m5_review_pack_accessibility_parity_and_narrowing_when_pack_version_digest_owner_provenance_parity_difference_or_template_attribution_is_stale::tests::regenerate_checked_artifacts_when_requested
```

Then run the suite without the flag to confirm the byte-lock holds.
