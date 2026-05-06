# Workflow bundle class matrix, signer + support-boundary disclosure, and stable-claim thresholds

This document freezes how Aureline separates **bundle class**, **source/status class**, and **support posture** so Start Center, docs, diagnostics, support exports, and claim surfaces cannot over-claim stability.

It exists because bundles are used in multiple contexts (launch wedges, migration handoff, org-approved baselines, design-partner pilots, and local drafts). Without a single matrix, it is easy for a surface to treat “this is a bundle” as a license to imply certification, launch-wedge coverage, compatibility, or support readiness.

The document is normative. If it disagrees with the PRD, Technical Architecture Document, Technical Design Document, UI / UX Spec, or Design System Style Guide, those source documents win and this document plus its companion artifacts must be updated in the same change.

## Companion artifacts

- [`/artifacts/workflow/bundle_class_matrix.yaml`](../../artifacts/workflow/bundle_class_matrix.yaml)
  — machine-readable bundle-class matrix with signer requirements, disclosure expectations, and stable-claim threshold gates.
- [`/schemas/workflow/bundle_claim_threshold.schema.json`](../../schemas/workflow/bundle_claim_threshold.schema.json)
  — boundary schema for exporting a computed claim-threshold verdict (for Start Center, diagnostics, and later claim-manifest tooling).

This document composes with:

- [`/docs/workflow/workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
  and [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json)
  for the closed vocabularies, required fields, and allowed class linkages.
- [`/docs/ux/start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md)
  and the bundle card/detail schemas for surface projection requirements.
- [`/docs/workflow/bundle_evidence_freshness_contract.md`](./bundle_evidence_freshness_contract.md)
  for evidence freshness, badge downgrade, and publish-gate rules.
- [`/docs/product/launch_wedge_contract.md`](../product/launch_wedge_contract.md)
  for launch-wedge cutlines and replacement-grade proof obligations.
- [`/docs/ux/template_and_prebuild_contract.md`](../ux/template_and_prebuild_contract.md)
  for `support_class` and `policy_notice_class` semantics reused by bundle projections.

## 1. Scope

In scope:

- a **bundle-class matrix** that keeps `bundle_class`, `bundle_source_class`,
  `bundle_status_class`, and `support_class` as distinct axes;
- signer/source requirements and the minimum disclosure set per class;
- support-boundary rules (who is “official” within which boundary);
- mirror/offline and identity dependency disclosure rules; and
- stable-claim threshold gates for launch-wedge coverage, benchmark-backed compatibility, and managed support readiness.

Out of scope:

- authoring or publishing production bundle sets;
- claim-publication automation implementation (see release contracts); and
- runtime enforcement code (guards/admission checks).

## 2. Non-negotiable axis separation

### 2.1 `bundle_class` (product row class)

`bundle_class` answers: **what kind of bundle row is this across Start Center, diagnostics, and claim surfaces?**

It is a product taxonomy axis. It MUST NOT be treated as evidence of who authored the bundle, who signed it, whether it is certified, or whether it is supported.

### 2.2 `bundle_source_class` and `bundle_status_class` (author/trust and operational status)

`bundle_source_class` answers: **who authored/curated the bundle and under what trust posture?**

`bundle_status_class` answers: **what is the operational status of this bundle revision at `minted_at`?**

These are **not** bundle classes. A surface that maps “Certified” into “Launch bundle” (or vice versa) is non-conforming.

### 2.3 `support_class` (support posture) and support boundary

`support_class` answers: **what support posture is asserted for the bundle row (official, community, experimental, etc.)?**

Support posture is distinct from:

- who signed the bundle (`bundle_signer_source_class` / continuity / signature class); and
- whether the bundle is eligible to imply stable claims (the stable-claim threshold gates in §5).

Surfaces MUST keep `support_class` visible alongside signer/source so “official support” cannot be misread as “vendor support” when the signer boundary is org-managed or community-managed.

## 3. Bundle-class matrix (summary)

The authoritative machine-readable matrix is
[`/artifacts/workflow/bundle_class_matrix.yaml`](../../artifacts/workflow/bundle_class_matrix.yaml).
This section is the reviewer-facing summary.

| `bundle_class` | Primary intent | Minimum required identity extras (summary) | Default review posture (summary) |
|---|---|---|---|
| `launch_bundle` | First-party launch-wedge entry | `archetype_bindings[]`, `workflow_bundle_id_register_ref`, `cutline_refs[]`, scoreboard-backed `evidence_link_refs[]` | may be recommended, but install still requires preview + explicit commit |
| `imported_user_bundle` | Migration handoff artifact | `imported_source` ref; narrowing notices when unreviewed | always review-first; never implies parity or stability beyond evidence links |
| `org_approved_bundle` | Org-curated baseline | org signer; mirror/offline posture explicit; policy notices when identity/entitlement required | review required; support boundary and identity dependencies must be explicit |
| `design_partner_bundle` | Time-bounded pilot bundle | pilot scope + expiry/review cadence | always treated as pilot/preview unless promoted; stale evidence narrows claims |
| `local_draft_bundle` | Local-only draft | local-only signer; removal/rollback not applicable | always user-review required; never claim-bearing |

The allowed `bundle_source_class` / `bundle_status_class` pairings and many of the identity requirements are enforced structurally by
[`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json)
and summarized in
[`/docs/workflow/workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
§7.

## 4. Disclosure requirements per bundle class

The matrix distinguishes two obligations:

1. **What the manifest/card MUST disclose** (fields and notices).
2. **What the surface MUST NOT imply** (claim ceilings).

### 4.1 Signer/source disclosure (all classes)

Every projection (Start Center card, detail page, diagnostics row, support export row, claim/badge row) MUST preserve these as separate fields (no collapsing):

- `bundle_class`
- `bundle_source_class`
- `bundle_status_class`
- `bundle_signer_source_class`, `signer_continuity_class`, `signature_class`
- `support_class`
- `mirror_or_offline_packaging_posture` (plus `mirror_freshness_ref` / `offline_bundle_freshness_ref` when applicable)
- `policy_notice_ref` when availability or trust is narrowed

### 4.2 Mirror/offline disclosure

Mirror/offline posture MUST remain explicit across all classes:

- a bundle delivered through a mirror MUST NOT render like a live-origin row;
- signed offline bundles MUST keep their pinned freshness visible; and
- `offline_no_bundle` is a visible narrowed state, not an absence.

### 4.3 Managed identity / entitlement dependency disclosure

If a bundle depends on:

- connected providers (code host, CI, companion, managed services),
- managed workspaces, or
- org policy / admin entitlements,

then projections MUST carry a `policy_notice_ref` whose notice class resolves to one of:
`connected_provider_policy_notice`, `managed_workspace_policy_notice`,
`fleet_policy_notice`, or `admin_policy_notice` (see template contract policy notice vocabulary).

Dependency markers MAY additionally cite companion bundles or required runtime ranges, but a dependency that narrows availability MUST NOT be expressed only as a hidden error or a post-click toast.

## 5. Stable-claim threshold gates (what a bundle may imply)

Stable-claim thresholds are **gates on implication**, not marketing labels. A bundle row may be excellent and still fail a stable-claim gate if proof is stale, signer continuity is broken, packaging posture is unknown, or the row is not eligible by class.

The gate output shape is frozen by
[`/schemas/workflow/bundle_claim_threshold.schema.json`](../../schemas/workflow/bundle_claim_threshold.schema.json).

### 5.1 Launch-wedge coverage implication

A bundle row MAY imply launch-wedge coverage only when all are true:

- `bundle_class = launch_bundle`;
- `bundle_source_class = certified` and signer source is core (`core_signing_root` or `core_signing_root_via_mirror`);
- `bundle_status_class = certified_current`;
- evidence is current enough to render `badge.certified` (freshness and continuity pass the badge-freshness gates); and
- the bundle cites at least one `cutline_ref` and carries `evidence_link_refs[]` covering every scoreboard family required by that cutline.

Any other state MUST render as non-launch-wedge (or as launch-wedge *candidate* with explicit narrowing) and MUST NOT project stable launch-wedge wording.

### 5.2 Benchmark-backed compatibility implication

A bundle row MAY imply benchmark-backed compatibility only when all are true:

- `bundle_source_class = certified` and `bundle_status_class = certified_current`;
- evidence links include a `compatibility_report` and at least one `benchmark_packet` or `scoreboard_packet` ref; and
- freshness/continuity gates admit a non-stale posture for the referenced proof.

Bundles that are `managed_approved` or `community_*` MAY still carry compatibility evidence links for review, but they MUST NOT imply certified compatibility without the certified threshold above.

### 5.3 Managed support readiness implication

A bundle row MAY imply managed support readiness only when all are true:

- `bundle_class = org_approved_bundle`;
- `bundle_source_class = managed_approved` and `bundle_status_class = managed_approved_current`;
- signer source is org-managed (`org_policy_signing_root` or `org_mirror_signing_root`);
- `support_class = officially_supported`; and
- any identity/entitlement dependency is disclosed via `policy_notice_ref`.

This gate does not turn an org bundle into a first-party certified launch bundle. It only permits the surface to describe the row as “org-managed and supported” within its declared boundary.

