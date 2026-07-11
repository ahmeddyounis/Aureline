# M5 docs-pane-header and boundary-fact-grid controls

The first implement lane over the frozen [M5 embedded-boundary component matrix](m5_embedded_boundary_components_contract.md). It turns the two documentation-facing embedded-boundary components — the **docs / help pane header** and the **boundary-fact grid** — into resolvers that produce export-safe, honest projections, so every claimed M5 docs/help pane makes explicit *what* it is showing and *when* a browser handoff is required, instead of leaving a user to guess whether a pane is project-local, mirrored vendor material, extension-contributed, or browser-handoff-required.

- Controls packet schema: `schemas/ui/m5-docs-pane-header-boundary-fact-grid-controls.schema.json`
- Support export: `artifacts/release/m5-docs-pane-header-boundary-fact-grid-controls-proof/support_export.json`
- Matrix CSV: `artifacts/release/m5-docs-pane-header-boundary-fact-grid-controls-proof/matrix.csv`
- Summary: `artifacts/release/m5-docs-pane-header-boundary-fact-grid-controls-proof/summary.md`
- Narrowed fixtures: `fixtures/ui/m5-docs-pane-header-boundary-fact-grid-controls/`
- Resolver + validator: `crates/aureline-shell` (module `implement_the_m5_docs_pane_header_and_boundary_fact_grid_...`)

## Reused, not re-minted

The lane binds directly to the frozen embedded-boundary object model so it can never fork its own owner, origin, boundary, or fallback wording:

- **Boundary disposition** reuses the single controlled `M5EmbeddedBoundaryDisposition` vocabulary from the matrix (live_first_party_local, live_first_party_hosted, live_provider_owned, stale_snapshot, offline_snapshot, provider_blocked, browser_handoff_only, capability_limited, not_evaluated).
- **Owner / origin** reuses `WebviewOwnerClass` (extension-owned, provider-owned, first-party-embedded, unknown/untrusted) from the auth-boundary object model.
- **Data boundary** reuses `DataExitBoundary` (no payload leaves product, metadata-safe object refs, proposal refs only, redacted support packet, security payloads only, external public browse, vendor/third-party outbound).
- **Capability limits** reuse `CapabilityLimitClass`, and **freshness** reuses the matrix `M5EmbeddedFreshnessState`.

## Docs-pane header resolver

`resolve_docs_pane_header` degrades first rather than ever letting an undistinguishable pane header read as a clean pass:

| Condition | Degrade reason |
| --- | --- |
| Source class unstated | `source_class_unstated` |
| Owner / origin undisclosed | `owner_or_origin_unstated` |
| Version / pack identity missing | `version_or_pack_identity_missing` |
| Last-updated state unstated | `last_updated_unstated` |
| Required browser handoff not exposed | `handoff_required_but_not_exposed` |
| Proof stale | `proof_stale` |

A clean header names its source class, owner/origin, version/pack identity, and last-updated state, and reports `distinguishable_source = true` — the **AC1** guarantee that a user can tell whether a docs/help pane is project-local, mirrored vendor material, extension-contributed, or browser-handoff-required without leaving the pane. A stale or offline snapshot never reads as fresh first-party local truth.

## Boundary-fact grid resolver

`resolve_boundary_fact_grid` names the data boundary and reading posture and explains why the pane is trustworthy for in-product reading but not high-risk approval:

| Condition | Degrade reason |
| --- | --- |
| Data boundary unstated | `data_boundary_unstated` |
| Claims approval / policy authority (masquerade) | `masquerades_as_approval_authority` |
| Offline / mirrored posture unstated | `offline_or_mirrored_posture_unstated` |
| Reading trust not explained | `reading_trust_not_explained` |
| Proof stale | `proof_stale` |

The `masquerades_as_approval_authority` degrade is the **AC2** guarantee: help panes never masquerade as approval or policy-authority surfaces, and — paired with the header's `handoff_required_but_not_exposed` degrade — an external handoff is always exposed when the source contract requires it.

## Guardrails

Every controls row asserts (and the validator enforces) that it never:

- masquerades as native permission or irreversible approval UI;
- hides owner/origin or the browser handoff behind menus only;
- renders a stale, offline, or provider-blocked pane as fresh first-party local truth;
- embeds a high-risk approval without a native step-up.

Acceptance criteria are proven by the resolved examples carried in the packet, not merely asserted by governance flags. Raw secret values and private endpoints never cross the export boundary.
