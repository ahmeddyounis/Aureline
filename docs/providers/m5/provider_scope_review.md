# Provider Scope Review

This packet freezes the canonical M5 provider scope-review vocabulary for
acting identity, installation grants, delegated credentials, effective provider
scope, least-privilege fallbacks, and cached-scope invalidation.

## Contract

- Rust schema of record:
  [`crates/aureline-provider/src/scope_review/mod.rs`](../../../crates/aureline-provider/src/scope_review/mod.rs)
- Page schema:
  [`schemas/providers/provider_scope_review.schema.json`](../../../schemas/providers/provider_scope_review.schema.json)
- Effective-scope boundary vocabulary:
  [`schemas/providers/effective_scope_resolution.schema.json`](../../../schemas/providers/effective_scope_resolution.schema.json)
- Seeded fixture:
  [`fixtures/providers/m5/provider_scope_review/page.json`](../../../fixtures/providers/m5/provider_scope_review/page.json)
- Seeded support export:
  [`artifacts/provider/m5/provider_scope_review/support_export.json`](../../../artifacts/provider/m5/provider_scope_review/support_export.json)

## What the page proves

- Requested action, target object, provider-declared scope, effective scope,
  policy locks, trust posture, acting identity, and authority health are
  inspectable on one decision record.
- Installation/app grants, delegated credentials, policy-injected service
  identities, and human accounts remain distinct and do not collapse into a
  generic connected badge.
- Least-privilege alternatives stay explicit: inspect-only review, local draft,
  deferred publish, browser-only completion, admin review, and copy/export all
  remain typed options instead of hidden fallback behavior.
- Revocation, suspension, host mismatch, tenant switch, org-membership loss,
  approval-ticket loss, and provider-health degrade invalidate cached decisions
  through typed events that name the forced downgrade and repair hook.
- Desktop, CLI/headless, companion, and support/export projections must mirror
  the same decision object rather than rephrasing authority locally.

## Guardrails

- Sign-in alone never implies write scope.
- Bot and installation grants are labeled as non-human authority.
- Support/export surfaces preserve provider class, authority health,
  acting-identity class, decision, reason, effective scope, alternatives, and
  invalidation lineage without exporting raw credentials or hidden delegation
  material.
