# M5 review-pack evaluator matrix — operations contract

Status: **frozen** (B152 opening freeze-matrix, row M05-1274).

This document is the human-readable contract for the frozen review-pack evaluator
object model. The authoritative gate is the Rust validator in
`crates/aureline-ui/src/m5_review_pack_evaluator_matrix`; the checked-in support
export, matrix CSV, dashboard, and narrowed fixtures are minted from the single
seed builder and must never be hand-edited.

## What this matrix freezes

Aureline's review-pack evaluator model — the set of repo-defined review-pack
objects that local, hosted, CI, browser/companion follow-up, and AI review lanes
all bind to as **executable shared truth** rather than summary metadata or
provider-specific behavior. Six governed object classes:

| Object class | Canonical domain schema | First consumers |
| --- | --- | --- |
| `review_pack_record` | `schemas/review/m5-review-pack.schema.json` | review detail, review-pack summary, merge-readiness, AI review panel, support/export |
| `ownership_signal` | `schemas/review/m5-ownership-signal.schema.json` | review detail, ownership overlay, merge-readiness, support/export |
| `required_evidence_check_row` | `schemas/review/m5-review-pack-result.schema.json` | merge-readiness, review detail, review-pack summary, support/export |
| `local_ci_parity_strip` | `schemas/review/m5-local-ci-parity.schema.json` | local-CI parity strip, merge-readiness, review detail, provider handoff, support/export |
| `ai_policy_hook` | `schemas/review/m5-ai-policy-hook.schema.json` | AI review panel, review detail, provider handoff, support/export |
| `review_template_packet` | `schemas/review/m5-review-template-packet.schema.json` | review detail, review-pack summary, support/export, help/docs |

Every class is bound to the same shared role taxonomy, the same required visible
state (pack label, pack version and digest, owner provenance, evaluator result
class, local-versus-provider parity, pack freshness state, template attribution),
and the same hard invariants regardless of the surface that renders it.

## Stable vocabulary

- **Parity / evaluator result class** (`M5ReviewPackParityState`):
  `provider_authoritative`, `local_parity_estimate`, `stale_relative_to_base_head`,
  `not_evaluated_here`, `ci_only`, `provider_unavailable`, `draft_only_review_state`.
  `is_provider_authoritative()` is a positive match on `provider_authoritative`
  only, so a local parity estimate is mechanically distinct from provider-authoritative
  mergeability or approval truth.
- **Owner authority** (`M5ReviewPackOwnerAuthority`): `advisory_owner`,
  `enforced_owner`, `no_owner_declared`, `ownership_unavailable`. `is_enforced()`
  matches `enforced_owner` only so advisory-owner is never flattened into an
  enforced-owner merge gate.
- **Pack freshness / invalidation** (`M5ReviewPackFreshness`): `pack_fresh`,
  `stale_pack`, `partial_scope`, `slice_omitted`, `pack_invalid`.
  `is_stale_or_partial()` covers everything except `pack_fresh`.

## Hard invariants (all must be `false` on every row)

1. `lets_a_local_parity_estimate_masquerade_as_provider_authoritative`
2. `hides_ci_only_not_evaluated_here_or_provider_unavailable_behind_a_green_summary`
3. `flattens_advisory_owner_and_enforced_owner_into_one_owner_pill`
4. `lets_ai_review_run_under_a_different_pack_version_without_disclosure`
5. `loses_review_pack_version_digest_or_template_attribution_when_exporting_publishing_or_reopening`

These are the batch guardrails: a local parity estimate never masquerades as
provider-authoritative; `ci-only`, `not-evaluated-here`, and `provider-unavailable`
are never hidden behind a green summary; advisory-owner and enforced-owner stay
distinct; AI review never runs under a different pack version without disclosure;
and review-pack version/digest and template attribution survive export, publish,
and reopen.

## Mint-from-truth (regenerating checked-in artifacts)

```text
cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- support-export  > artifacts/review/m5-review-pack-results/support_export.json
cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- csv             > artifacts/review/m5-review-pack-results/matrix.csv
cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- report          > artifacts/review/m5-review-pack-evaluator-matrix.md
cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- dashboard       > dashboards/m5-review-pack-health.json
cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- fixture-local-ci-parity-beta-narrowed   > fixtures/review/m5-review-pack-parity/local_ci_parity_beta_narrowed.json
cargo run -p aureline-ui --example dump_m5_review_pack_matrix -- fixture-ai-policy-hook-preview-narrowed  > fixtures/review/m5-review-pack-parity/ai_policy_hook_preview_narrowed.json
```

The narrowed fixtures hold `local_ci_parity_strip` at Beta and `ai_policy_hook`
at Preview while keeping every object class visible, proving claims narrow without
dropping an object from the matrix.
