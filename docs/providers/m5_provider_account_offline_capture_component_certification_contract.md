# M5 Provider-Account / Offline-Capture Component Surface Certification (M05-923)

This is the **closing capstone** of the B108 provider-account / project-or-board-mapping /
sync-behavior / offline-capture / privacy-redaction component lane. Where the freeze matrix
(`m5-provider-account-offline-capture-component-matrix.md`, M05-916) defines the five reusable
components, the M05-917..920 primitive lanes narrow each one, the M05-921 consumer lane proves they
are reusable across the claimed work-item / status-transition / issue-intake / help / support-export
/ browser-handoff consumers, and the M05-922 accessibility / auto-narrowing capstone certifies
keyboard / screen-reader / CLI / export parity per family, this capstone **certifies** that the
shared provider-boundary component truth holds on every claimed M5 provider-backed team-workflow
surface — and auto-narrows any surface that cannot sustain it.

- Boundary schema: `schemas/ui/m5-provider-account-offline-capture-component-certification.schema.json`
- Canonical support export: `artifacts/release/m5-provider-account-offline-capture-component-certification/support_export.json`
- Machine-readable matrix: `artifacts/release/m5-provider-account-offline-capture-component-certification/matrix.csv`
- Markdown report: `artifacts/release/m5-provider-account-offline-capture-component-certification/report.md`
- Fixtures mirror: `fixtures/ui/m5-provider-account-offline-capture-component-certification/`
- Implementation: `crates/aureline-provider/src/certify_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_truth_on_every_claimed_m5_provider_backed_team_workflow_surface/`

## What it certifies

The packet is keyed on the claimed **surface** a user reads, drafts, publishes, replays, or exports
provider-backed work-item data on — not on component family or primitive lane. The eight certified
surfaces are:

`work_item_detail`, `status_transition_review`, `issue_intake`, `docs_help`, `support_export`,
`browser_handoff`, `provider_settings`, and `cli_headless`.

Each surface is scored on six truth axes:

1. `visual` — connection state, tenant scope, effective write scope, default-destination mapping,
   sync mode, queued-draft state, and redaction / export boundary are shown on the primary surface.
2. `keyboard` — the same truth and its controls are reachable without a pointer.
3. `screen_reader` — the same truth is announced non-visually, never color / glyph only.
4. `cli_export` — **always-on**: the certified surface state is reconstructable as text / JSON /
   Markdown from the same provider identity.
5. `degraded_state` — a limited effective write scope, a stale session, a policy-blocked mapping, or
   a local-only offline-capture packet honestly downgrades a `provider_committed` /
   `reviewable_projection` claim.
6. `provider_boundary_provenance` — connection state, tenant scope, effective write scope,
   default-destination mapping, sync mode, queued-draft state, and redaction / export boundary stay
   explicit before any read, draft, publish, replay, or export, never inheriting a healthier lane's
   truth, and **the boundary never drops account / mapping / queued-draft / redaction lineage**
   between a cached read, a local draft, and a committed publish.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps a
`provider_committed` / `reviewable_projection` claim while a truth axis is not current — the
effective write scope is limited, the session is stale, the project/board mapping is policy-blocked,
the offline-capture packet is local-only, or the connection / mapping / sync / redaction boundary is
unstated — is over-claiming and is blocked (`red`). A surface that discloses the reduction by
narrowing its provider claim (with a bound reason and a frozen downgrade trigger) is honestly
`yellow`. The always-on `cli_export` axis must always stay certified. **Provider truth never drops
lineage**: a narrowed surface preserves its account / mapping / queued-draft / redaction lineage
continuity rather than dropping it between a cached read, a local draft, and a committed publish
(`lineage_preserved` / `preserves_lineage_continuity`).

The provider-claim ladder (strongest first) is reused from the M05-922 accessibility capstone:
`provider_committed` (5) > `reviewable_projection` (4) > `limited_scope_projection` (3) >
`stale_session_projection` (2) > `policy_blocked_mapping` (1) > `local_only_packet` (0).
Certification may only narrow a claim, never strengthen it.

## Derived verdict

`derived_status` is never asserted by the author — it is recomputed from the axis outcomes, lineage
preservation, export parity, and claim narrowing. A row is `red` when it is malformed, drops
CLI/export parity, drops lineage, hides an undisclosed drift, retains a degraded axis behind a full
claim, or carries an inconsistent narrowing. It is `yellow` when a degraded axis is disclosed and
bound to a visible claim narrowing, and `green` when every axis certifies at the claimed tier.

## Coverage

The canonical packet certifies all eight surfaces exactly once (four `green`, four `yellow`, zero
`red`), every one of the five frozen component families on at least one surface, every axis on every
row, and lineage preservation on every surface. Every row cites the one canonical proof bundle
(`artifacts/release/m5-provider-account-offline-capture-proof/support_export.json`) plus the M05-921
consumer and M05-922 accessibility support exports as supporting evidence.

The `yellow` rows exercise the spec's four auto-narrowing conditions: a limited effective write scope
(`provider_settings` → `limited_scope_projection`), a stale session (`browser_handoff` →
`stale_session_projection`), a policy-blocked mapping (`docs_help` → `policy_blocked_mapping`), and a
local-only offline-capture packet (`cli_headless` → `local_only_packet`).

## Regenerating the artifacts

The seed builder (`seeded_m5_provider_account_offline_capture_component_certification_packet`) is the
one source of truth for both the tests and the on-disk export. To regenerate:

```
GEN_PROVIDER_CERT_ARTIFACTS=1 cargo test -p aureline-provider --lib \
  certify_provider_account_row_project_or_board_mapping_row_sync_behavior_row_offline_capture_row_and_privacy_redaction_row_truth_on_every_claimed_m5_provider_backed_team_workflow_surface::tests::generate_artifacts
```

The `checked_in_export_matches_seeded_builder` test fails if the on-disk export drifts from the seed
builder. The packet is metadata-only: raw provider payloads, captured draft bodies, redacted field
contents, and credentials never cross this boundary.
