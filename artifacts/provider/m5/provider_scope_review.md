# Provider Scope Review

- Packet: `providers:scope_review:v1`
- Schema: `schemas/providers/provider_scope_review.schema.json`
- Fixture: `fixtures/providers/m5/provider_scope_review/page.json`
- Support export: `artifacts/provider/m5/provider_scope_review/support_export.json`
- Contract doc: `docs/providers/m5/provider_scope_review.md`

## Coverage

- Human account, installation/app grant, delegated credential, and
  policy-injected service identity authority all stay distinct.
- Effective scope is separated from provider-declared scope and local policy
  locks on every decision row.
- Least-privilege alternatives cover browser-only completion, local draft,
  inspect-only review, admin review, and copy/export.
- Invalidation events cover revoked authority, host mismatch, tenant switch,
  org-membership loss, and provider-health degradation.
- Desktop, CLI/headless, companion, and support/export projections all mirror
  the same canonical decision object.

## Guardrails

The seeded packet proves that provider mutation authority fails closed when
identity health degrades, host or tenant bindings drift, or policy narrows the
admitted path. Support exports preserve provider class, authority health,
acting-identity class, effective-scope result, least-privilege alternatives,
and invalidation lineage without exporting raw credentials or hidden
delegation material.
