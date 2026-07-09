# M5 Companion Component Surface Certification (M05-1003)

This is the **closing capstone** of the B118 notification-row / mobile-review-card / CI-status-card /
session-follow-tile / incident-snapshot-card / desktop-handoff-sheet component lane. Where the freeze
matrix (`m5-companion-component-matrix.schema.json`, M05-996) defines the six reusable components, the
M05-997..999 primitive lanes narrow each one, the M05-1000 degraded-state lane governs their cached /
offline / auth-blocked / policy-blocked states, the M05-1001 consumer lane proves they are reusable
across the claimed inbox / review / CI / session-follow / incident / advisory / help / support /
handoff / export consumers, and the M05-1002 accessibility / auto-narrowing capstone certifies
keyboard / screen-reader / share / export parity per family, this capstone **certifies** that the
shared companion component truth holds on every claimed M5 companion and handoff surface — and
auto-narrows any surface that cannot sustain it.

- Boundary schema: `schemas/ui/m5-companion-component-certification.schema.json`
- Canonical support export: `artifacts/release/m5-companion-component-certification/support_export.json`
- Machine-readable matrix: `artifacts/release/m5-companion-component-certification/matrix.csv`
- Markdown report: `artifacts/release/m5-companion-component-certification/report.md`
- Fixtures mirror: `fixtures/ui/m5-companion-component-certification/`
- Implementation: `crates/aureline-companion/src/certify_companion_component_truth_on_every_claimed_m5_companion_and_handoff_surface/`

## What it certifies

The packet is keyed on the claimed **surface** a user actually triages a notification on, reviews a
change on, reads CI on, follows a session on, stays aware of an incident on, hands work back to
desktop from, or exports / gets help on — not on component family or primitive lane. The eight
certified surfaces are:

`notification_inbox`, `mobile_review_queue`, `ci_status_dashboard`, `session_follow`,
`incident_awareness`, `desktop_handoff`, `support_export`, and `help_docs`.

Each surface is scored on six truth axes:

1. `visual` — object identity, workspace/repo client scope, freshness, companion-versus-desktop
   capability boundary, severity, and exact handoff target are shown on the primary surface.
2. `keyboard` — the same truth and its controls are reachable without a pointer.
3. `screen_reader` — the same truth is announced non-visually, never color / glyph only.
4. `share_export` — **always-on**: the certified surface state is reconstructable as text / JSON /
   Markdown from the same object identity, with the raw code-bearing payload excluded.
5. `degraded_state` — a stale object, a limited companion authority, a narrowed tenant scope, or a
   revoked handoff honestly downgrades a `live_companion_safe` / `cached_continuity_safe` claim.
6. `companion_boundary_provenance` — object identity, client scope, freshness, capability boundary,
   severity, and exact handoff target stay explicit before any triage, review, follow, escalation, or
   handoff, never inheriting a healthier lane's truth, never letting friendly companion wording
   conceal object scope / freshness / the desktop-required capability boundary, and **the boundary
   never drops identity / scope / freshness / capability / severity / handoff continuity** between a
   triage, a review, a follow, an escalation, and a desktop handoff.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps a
`live_companion_safe` / `cached_continuity_safe` claim while a truth axis is not current — the object
is stale, the companion authority is limited, the tenant scope has narrowed, or the handoff validity
is revoked — is over-claiming and is blocked (`red`). A surface that discloses the reduction by
narrowing its companion claim (with a bound reason and a frozen downgrade trigger) is honestly
`yellow`. The always-on `share_export` axis must always stay certified. **Companion truth never drops
continuity**: a narrowed surface preserves its object-identity / client-scope / freshness /
capability / severity / handoff continuity rather than dropping it between a triage, a review, a
follow, an escalation, and a desktop handoff (`companion_truth_preserved` /
`preserves_companion_truth_continuity`).

The companion-claim ladder (strongest first) is reused from the M05-1002 accessibility capstone:
`live_companion_safe` (5) > `cached_continuity_safe` (4) > `stale_freshness_projection` (3) >
`limited_authority_projection` (2) > `narrowed_tenant_projection` (1) > `revoked_handoff_projection`
(0). Certification may only narrow a claim, never strengthen it — so "certified" never implies a live,
in-authority, companion-safe surface when the object is stale, the authority is limited, the tenant
scope has narrowed, or the handoff is revoked.

## Derived verdict

`derived_status` is never asserted by the author — it is recomputed from the axis outcomes, companion
truth preservation, export parity, and claim narrowing. A row is `red` when it is malformed, drops
share/export parity, drops companion truth, hides an undisclosed drift, retains a degraded axis behind
a full claim, or carries an inconsistent narrowing. It is `yellow` when a degraded axis is disclosed
and bound to a visible claim narrowing, and `green` when every axis certifies at the claimed tier.

## Coverage

The canonical packet certifies all eight surfaces exactly once (four `green`, four `yellow`, zero
`red`), every one of the six frozen component families on at least one surface, every axis on every
row, and companion-truth preservation on every surface. Every row cites the one canonical proof bundle
(`artifacts/release/m5-companion-component-proof/support_export.json`) plus the M05-1001 consumer and
M05-1002 accessibility support exports as supporting evidence.

The `yellow` rows exercise the spec's four auto-narrowing conditions: a limited companion authority
(`mobile_review_queue` → `limited_authority_projection`), a narrowed tenant scope (`session_follow` →
`narrowed_tenant_projection`), a revoked handoff (`desktop_handoff` → `revoked_handoff_projection`),
and a stale object freshness (`help_docs` → `stale_freshness_projection`).

## Regenerating the artifacts

The seed builder (`seeded_m5_companion_component_certification_packet`) is the one source of truth for
both the tests and the on-disk export. To regenerate:

```
GEN_COMPANION_CERT_ARTIFACTS=1 cargo test -p aureline-companion --lib \
  certify_companion_component_truth_on_every_claimed_m5_companion_and_handoff_surface::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the on-disk export drifts from the seed
builder. The packet is metadata-only: a raw object body, a code-bearing payload, and
companion-bearing material never cross this boundary.
