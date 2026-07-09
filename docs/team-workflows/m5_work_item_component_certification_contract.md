# M5 Work-Item Component Surface Certification (M05-987)

This is the **closing capstone** of the B116 work-item-row / provider-chip-group / relation-strip /
sync-pending-pill / work-item-detail-header / status-transition-sheet / related-evidence-card /
offline-handoff-packet-card component lane. Where the freeze matrix
(`m5-work-item-component-matrix.schema.json`, M05-980) defines the eight reusable components, the
M05-981..984 primitive lanes narrow each one, the M05-985 consumer lane proves they are reusable
across the claimed inbox / detail / review / incident / help / support-export / exported consumers,
and the M05-986 accessibility / auto-narrowing capstone certifies keyboard / screen-reader / CLI /
export parity per family, this capstone **certifies** that the shared work-item component truth holds
on every claimed M5 provider-backed team-workflow surface — and auto-narrows any surface that cannot
sustain it.

- Boundary schema: `schemas/ui/m5-work-item-component-certification.schema.json`
- Canonical support export: `artifacts/release/m5-work-item-component-certification/support_export.json`
- Machine-readable matrix: `artifacts/release/m5-work-item-component-certification/matrix.csv`
- Markdown report: `artifacts/release/m5-work-item-component-certification/report.md`
- Fixtures mirror: `fixtures/ui/m5-work-item-component-certification/`
- Implementation: `crates/aureline-provider/src/certify_work_item_row_provider_chip_group_relation_strip_sync_pending_pill_work_item_detail_header_status_transition_sheet_related_evidence_card_and_offline_handoff_packet_card_truth_on_every_claimed_m5_provider_backed_team_workflow_surface/`

## What it certifies

The packet is keyed on the claimed **surface** a user reads, drafts, transitions, retries, or exports
provider-backed work-item data on — not on component family or primitive lane. The eight certified
surfaces are:

`work_item_inbox`, `work_item_detail`, `status_transition_review`, `incident_review`, `docs_help`,
`support_export`, `offline_handoff_export`, and `cli_headless`.

Each surface is scored on six truth axes:

1. `visual` — canonical work-item identity, provider authority, local-versus-provider state, linked
   engineering context, side-effect preview, and publish-later continuity are shown on the primary
   surface.
2. `keyboard` — the same truth and its controls are reachable without a pointer.
3. `screen_reader` — the same truth is announced non-visually, never color / glyph only.
4. `cli_export` — **always-on**: the certified surface state is reconstructable as text / JSON /
   Markdown from the same provider identity.
5. `degraded_state` — a stale provider freshness, a read-only / policy-blocked write scope, a
   local-only sync state, or an unpublishable offline-handoff packet honestly downgrades a
   `provider_committed` / `reviewable_projection` claim.
6. `provider_boundary_provenance` — canonical identity, provider authority, effective write scope,
   local-versus-provider state, linked engineering context, side-effect preview, and publish-later
   continuity stay explicit before any read, draft, transition, retry, or export, never inheriting a
   healthier lane's truth, and **the boundary never drops canonical-id / provider-authority /
   linked-context / queued-draft / publish-later lineage** between a cached read, a local draft, and
   a committed publish.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps a
`provider_committed` / `reviewable_projection` claim while a truth axis is not current — the provider
freshness is stale, the effective write scope is read-only or policy-blocked, the sync state is
local-only, or the offline-handoff packet is unpublishable — is over-claiming and is blocked (`red`).
A surface that discloses the reduction by narrowing its provider claim (with a bound reason and a
frozen downgrade trigger) is honestly `yellow`. The always-on `cli_export` axis must always stay
certified. **Work-item truth never drops lineage**: a narrowed surface preserves its canonical-id /
provider-authority / linked-context / queued-draft / publish-later lineage continuity rather than
dropping it between a cached read, a local draft, and a committed publish (`lineage_preserved` /
`preserves_lineage_continuity`).

The provider-claim ladder (strongest first) is reused from the M05-986 accessibility capstone:
`provider_committed` (5) > `reviewable_projection` (4) > `stale_freshness_projection` (3) >
`read_only_projection` (2) > `local_only_projection` (1) > `unpublishable_packet_projection` (0).
Certification may only narrow a claim, never strengthen it.

## Derived verdict

`derived_status` is never asserted by the author — it is recomputed from the axis outcomes, lineage
preservation, export parity, and claim narrowing. A row is `red` when it is malformed, drops
CLI/export parity, drops lineage, hides an undisclosed drift, retains a degraded axis behind a full
claim, or carries an inconsistent narrowing. It is `yellow` when a degraded axis is disclosed and
bound to a visible claim narrowing, and `green` when every axis certifies at the claimed tier.

## Coverage

The canonical packet certifies all eight surfaces exactly once (four `green`, four `yellow`, zero
`red`), every one of the eight frozen component families on at least one surface, every axis on every
row, and lineage preservation on every surface. Every row cites the one canonical proof bundle
(`artifacts/release/m5-work-item-component-proof/support_export.json`) plus the M05-985 consumer and
M05-986 accessibility support exports as supporting evidence.

The `yellow` rows exercise the spec's four auto-narrowing conditions: a stale provider freshness
(`incident_review` → `stale_freshness_projection`), a read-only / policy-blocked write scope
(`docs_help` → `read_only_projection`), a local-only sync state (`cli_headless` →
`local_only_projection`), and an unpublishable offline-handoff packet (`offline_handoff_export` →
`unpublishable_packet_projection`).

## Regenerating the artifacts

The seed builder (`seeded_m5_work_item_component_certification_packet`) is the one source of truth for
both the tests and the on-disk export. To regenerate:

```
GEN_WORK_ITEM_CERT_ARTIFACTS=1 cargo test -p aureline-provider --lib \
  certify_work_item_row_provider_chip_group_relation_strip_sync_pending_pill_work_item_detail_header_status_transition_sheet_related_evidence_card_and_offline_handoff_packet_card_truth_on_every_claimed_m5_provider_backed_team_workflow_surface::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the on-disk export drifts from the seed
builder. The packet is metadata-only: raw provider payloads, captured draft bodies, redacted field
contents, and credentials never cross this boundary.
