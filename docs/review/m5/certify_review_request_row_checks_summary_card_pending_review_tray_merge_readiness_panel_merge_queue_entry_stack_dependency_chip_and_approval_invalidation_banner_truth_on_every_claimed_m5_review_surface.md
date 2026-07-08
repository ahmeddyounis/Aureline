# M5 Review-Component Surface Certification

Closing certification capstone for batch **B112**. It certifies that the seven
shared M5 review components — review-request-row, checks-summary-card,
pending-review-tray, merge-readiness-panel, merge-queue-entry, stack-dependency-chip,
and approval-invalidation-banner — present the **same controlled component truth on
every claimed M5 review surface**, with no hidden provider drift.

- Module: `crates/aureline-review/src/certify_review_request_row_checks_summary_card_pending_review_tray_merge_readiness_panel_merge_queue_entry_stack_dependency_chip_and_approval_invalidation_banner_truth_on_every_claimed_m5_review_surface`
- Boundary schema: [`schemas/ui/m5-review-request-check-queue-component-certification.schema.json`](../../../schemas/ui/m5-review-request-check-queue-component-certification.schema.json)
- Checked export: `artifacts/review/m5/…/support_export.json`
- Release proof: `artifacts/release/m5-review-request-check-queue-certification-proof/`
- Fixtures: `fixtures/ui/m5-review-request-check-queue-component-certification/`

## What it builds on

| Lane | Contract |
| --- | --- |
| Component matrix (M05-948) | `schemas/ui/m5-review-request-check-queue-component-matrix.schema.json` |
| Shared consumers (M05-953) | `schemas/ui/m5-review-component-consumer.schema.json` |
| A11y / headless / export parity (M05-954) | `schemas/ui/m5-review-component-accessibility-parity.schema.json` |
| review-request-row | `schemas/ui/m5-review-request-row.schema.json` |
| checks-summary-card | `schemas/ui/m5-checks-summary-card.schema.json` |
| merge-readiness / merge-queue / stack-dependency | `schemas/ui/m5-merge-readiness-panel.schema.json` |
| pending-review-tray / approval-invalidation | `schemas/ui/m5-pending-review-tray.schema.json` |

It reuses `M5ReviewComponent` (the seven frozen components) and
`ReviewComponentClaimTier` (the five-tier provider-backed → handoff-required claim
ladder) directly; it does not re-mint them.

## Certified surfaces

Eight claimed M5 review surfaces are certified:
`desktop_review_list`, `review_detail_pane`, `companion_review_queue`,
`help_review_surface`, `support_export`, `exported_review_packet`, `cli_headless`,
and `diagnostics`.

## Certification axes

Each surface row scores six axes:

- **visual**, **keyboard**, **screen_reader**, **cli_export** — always-on parity
  axes every claimed component must pass on every surface.
- **degraded_state** — narrows the claim honestly when provider freshness, queue
  authority, approval lineage, or a stack relation weakens.
- **provider_local_provenance** — the certification-specific separation axis. It
  keeps the provider-backed-vs-local distinction explicit so a **certified surface
  never implies its provider-backed truth is fresh or authoritative**.

## Status ladder

`derive_review_component_surface_claim_status` scores each surface:

- **`certified_parity`** (green): certified claim equals the claimed claim, no axis
  narrows, and component truth is preserved.
- **`narrowed_parity`** (yellow): a claim narrowed or an axis narrowed, but the
  component's meaning is preserved and the narrowing is disclosed with a trigger.
- **`parity_blocked`** (red): the component's provider/local distinction, queue
  owner, check class, approval invalidation, or freshness truth was flattened out of
  the surface. This is the delta of the capstone — certification may narrow a claim,
  but it may never drop the component's meaning.

## Acceptance criteria

- **AC1 — no hidden provider drift.** Every claimed surface presents the same
  controlled component truth. Enforced by the trust-review invariants, the
  per-surface axis coverage, the `raw_review_material_in_export` guard, and the
  `all_surfaces_covered` / `all_components_covered` summary flags.
- **AC2 — parity, not just workflow maturity.** The certified claim may never exceed
  the claimed one (`certified_claim_exceeds_claimed`), status must match the derived
  status (`status_mismatch`), and `apply_downgrade_automation` narrows a surface the
  moment its provider backing goes stale — proving the release evidence tracks
  component parity, not earlier workflow-level review maturity rows. The
  `certified_never_implies_fresh` theme is proven by the orthogonal
  `provider_local_provenance` axis.

## Regenerating artifacts

```
GEN_REVIEW_COMPONENT_CERTIFICATION_ARTIFACTS=1 \
  cargo test -p aureline-review --lib regenerate_review_component_certification_artifacts
```

Then rebuild and run the module suite:

```
cargo test -p aureline-review --lib certify_review_request_row_checks_summary_card
```
