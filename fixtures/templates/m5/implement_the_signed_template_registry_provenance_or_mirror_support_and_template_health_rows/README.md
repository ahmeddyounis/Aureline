# Signed Template Registry, Provenance/Mirror, and Template-Health Fixtures

These fixtures are valid, export-safe signed template-registry packets that
exercise the downgrade behavior the canonical support export keeps green. Each
keeps every canonical row present, the trust-review and consumer-projection
invariants satisfied, and proof freshness valid — the difference is which row is
narrowed and why. They are regenerated from the canonical builder via
`cargo run -p aureline-templates --example dump_signed_template_registry`.

## health_stale_narrowed.json

The official first-party row's template-health checks aged out of cadence, so it
narrows from `healthy_current` to `stale_but_inspectable` and gains the
`health_check_stale` downgrade trigger. Stale health is non-blocking, so the row
remains admitted for generation but shows a current stale-but-inspectable label
instead of an optimistic healthy claim. The org mirror, community, and repo-local
rows are unchanged.

## signature_failed_blocked.json

The community row's signature failed verification, so it is marked
`signature_or_trust_failed_blocks_starter`, its certification drops to
`certification_unknown_review_required`, its support to `support_unknown`, it is
withdrawn from generation, and it gains the `signature_unverified` trigger. The
row is labeled and blocked rather than hidden. The official, org mirror, and
repo-local rows are unchanged.
