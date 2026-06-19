# Locality descriptors and tenant-boundary surfaces

This contract turns each claimed managed, self-hosted, and sovereign continuity
row into two things a person can read directly in the product and in exportable
evidence:

1. A **locality descriptor** — plain-language processing location, storage
   location, an explicit region pin with a honored / cannot-honor state, and the
   retention/export class in force.
2. A **tenant-boundary card** — plain-language tenant/org scope, isolation
   posture, and key mode.

The packet is produced by
`aureline_continuity::m5_locality_descriptors_and_tenant_cards`. It sits on top
of the frozen continuity-claim matrix
(`aureline_continuity::m5_locality_tenant_keymode_and_drill_matrix`), reusing the
matrix's profile, lane, locality, tenant, and key-mode vocabulary so a person
reads the same words everywhere. This lane adds the region-pin, retention, and
isolation truth the matrix does not carry, plus the per-surface projection that
keeps the vocabulary identical across desktop, CLI/headless inspect,
service-health, support exports, About/Help, and docs/public-truth pages.

## What every surface answers the same way

- Where does processing happen, and where does durable storage live?
- Is this row pinned to a region, and is that pin currently honored?
- Which tenant/org boundary applies, and is its isolation verified?
- Which retention or export class is in force?

## Stable conditions

A page qualifies `stable` only when all of the following hold at once:

1. Every claimed row discloses its processing location, storage location, and
   retention class.
2. Every row is projected onto every surface it is required to reach (managed
   rows reach all six surfaces; local-core rows reach all but support export).
3. Every managed-scope row declares and names an explicit region pin, and that
   pin is honored.
4. Every managed-scope row discloses an explicit tenant scope and a verified
   tenant boundary isolation.
5. No self-hosted or sovereign row claims a broader vendor region than it
   operates.
6. Every surface renders the identical locality and tenant summary line for a
   given row.

## Fail-closed and claim narrowing

Region-pinned or tenant-scoped rows on the protected managed lane **fail closed**
when the declared boundary cannot be honored; the claim is withdrawn rather than
silently downgraded. Local-core desktop work never enters these rules and stays
accurately labeled. Each row carries a computed outcome with its own
qualification and reasons:

- `region_pin_unhonored` — a managed-lane region pin cannot be honored; the row
  **fails closed** and is **withdrawn**.
- `self_hosted_locality_overclaimed` — a self-hosted or sovereign row implies a
  broad vendor region it does not operate; the claim is **withdrawn**.
- `region_pin_undeclared_on_managed`, `tenant_boundary_unverified`,
  `locality_vocabulary_drift` — a region pin is missing, a tenant boundary is
  unverified, or a surface drifts from the descriptor vocabulary (**preview**).
- `processing_location_undisclosed`, `storage_location_undisclosed`,
  `retention_class_undisclosed`, `tenant_scope_undisclosed`,
  `surface_projection_incomplete` — a required disclosure or surface projection
  is missing (**beta**).

## Guardrails

- Processing location, storage location, and tenant scope are never compressed
  into one generic "managed" badge; each is its own typed, plain-language field.
- A self-hosted or sovereign claim may not imply stronger locality than the
  running topology provides.

## Output shape

The packet contains:

- one locality descriptor and one tenant-boundary card per claimed row
- a surface projection per (row, surface) pair carrying the exact locality and
  tenant summary lines rendered on that surface
- per-row outcomes joining each row to its computed qualification, narrow
  reasons, managed-scope flag, and fail-closed flag
- a summary and typed defects, plus a support-export wrapper

The packet is metadata-only. It intentionally excludes raw hostnames, raw tenant
identifiers, raw KMS handles, and all secret material.

## Canonical paths

- Doc: `docs/m5/continuity/locality-and-tenant-boundary-surfaces.md`
- Artifact: `artifacts/m5/continuity/locality_and_tenant_boundary_surfaces.md`
- Schema: `schemas/continuity/locality_descriptor.schema.json`
- Fixtures: `fixtures/continuity/locality_tenant_cases/`

This checked-in lane, schema, and fixtures are the canonical M5 source for
processing/storage/tenant/retention truth on claimed managed, self-hosted, and
sovereign surfaces. Desktop, CLI/headless inspect, service-health, support
exports, and docs/public-truth pages consume it instead of restating locality
claims by hand.

## Verify

```sh
cargo test -p aureline-continuity m5_locality_descriptors --locked
cargo run -q -p aureline-continuity --example dump_m5_locality_tenant_cards_fixtures -- page
cargo run -q -p aureline-continuity --bin aureline_locality_tenant_inspect -- fixtures/continuity/locality_tenant_cases/page.json
python3 tools/validate_m5_locality_tenant_cards_fixtures.py
```
