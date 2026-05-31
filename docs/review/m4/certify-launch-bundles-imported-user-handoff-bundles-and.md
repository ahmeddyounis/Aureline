# Certify launch bundles, imported-user handoff bundles, and org-approved bundles

**Scope:** Bind bundle identity, a machine-readable compatibility scorecard, and the certification claim a bundle is allowed to make on the stable line into a single previewable, attributable artifact that downgrades automatically when its evidence narrows.

**Status:** Stable switching lane — implemented in `crates/aureline-workspace`.

## Goal

The workflow-bundle review packet (`crates/aureline-workspace/src/bundles`) owns install, update, drift, and rollback truth. This module owns the layer above it: the **certification claim** a bundle may render — `Certified`, `Managed approved`, `Community`, `Imported`, or `Local draft` — and the **machine-readable compatibility scorecard** that claim must resolve to.

A stable bundle claim (`Certified` or `Managed approved`) may never be implied from prose alone. Any claimed stable handoff must point to a *current* scorecard row whose freshness, bridge state, downgrade state, and known-gap state still support it. When the row goes stale, the bridge parity narrows, a gap is detected, or the row is missing, the visible badge is **automatically downgraded** rather than left asserting parity the evidence no longer backs.

## Design principles

1. **Identity matches everywhere** — `bundle_id`, signer/source, archetype class, compatible Aureline range, and certification state live on one `certified_bundle_identity` that Start Center, diagnostics, CLI/headless install, export packets, and docs all read instead of cloning status text.
2. **Claims resolve to machine-readable scorecard rows** — every stable claim points at a `bundle_compatibility_scorecard_row` linking bundle id, imported-extension class, bridge state, supported deployment/profile rows, and certified reference-workspace ids, each with explicit freshness, downgrade, and known-gap state.
3. **Downgrade is automatic, not advisory** — the effective badge is *derived* from the claimed badge and the resolved row. Stale freshness, narrowed bridge parity, a disclosed gap, a missing/mismatched row, a prose-only basis, a stale certification state, or a missing reference workspace each narrows the badge and records a machine-readable reason.
4. **No parity from prose** — a `prose_only` claim can never produce a stable effective badge; `no_prose_only_stable_claim` is enforced.
5. **Imported-user handoff truth is preserved** — imported-user bundles must carry a preserved `imported_handoff_report` with a migration report reference and unsupported/partial item lists, so post-import help stays traceable rather than collapsing into one green banner.
6. **Offline and mirror parity** — every distribution class (public registry, mirror-first, offline archive) carries the same populated, machine-readable scorecard; offline/mirror flows may not degrade into opaque archive import.
7. **No hidden authority** — bundle application may never carry hidden imperative setup hooks, secret injection, or silent trust/egress/provider widening, even when certified or managed.

## Record kinds

| Record kind | Purpose |
|---|---|
| `bundle_archetype_certification_packet` | Top-level packet consumed by Start Center, diagnostics, CLI/headless install, support export, and docs. |
| `certified_bundle_identity` | Bundle identity that must match on every surface. |
| `bundle_compatibility_scorecard` | Machine-readable scorecard binding the bundle to certification evidence. |
| `bundle_compatibility_scorecard_row` | One scorecard row: imported-extension class, bridge state, supported deployment/profile rows, reference-workspace ids, freshness, downgrade, and known-gap state. |
| `bundle_certification_claim` | Claimed badge, effective badge after the scorecard is applied, support claim, and downgrade reasons. |
| `imported_handoff_report` | Preserved migration report and unsupported/partial item lists for imported-user bundles. |
| `bundle_archetype_certification_inspection` | Compact boolean projection for CLI and inspector surfaces. |

## Closed vocabularies

### Bundle classes
- `launch_bundle`, `imported_user_bundle`, `org_approved_bundle`, `design_partner_bundle`, `local_draft_bundle`

### Source classes
- `certified`, `managed_approved`, `community`, `imported`, `local_draft`

### Effective badge classes
- `certified`, `managed_approved`, `community`, `imported`, `local_draft`, `retest_pending`, `limited`, `status_unknown`
- Stable badges (`certified`, `managed_approved`) require a current scorecard row.

### Bridge states
- `exact_bridge`, `partial_bridge`, `approximate_bridge`, `unsupported_bridge`, `bridge_unknown`
- Only `exact_bridge` and `partial_bridge` support a stable parity claim.

### Scorecard freshness
- `fresh_current`, `aging_within_window`, `stale_past_window`, `imported_evidence`, `evidence_unknown`

### Downgrade reasons
- `scorecard_freshness_expired`, `bridge_parity_narrowed`, `known_gap_detected`, `missing_scorecard_row`, `scorecard_row_bundle_mismatch`, `prose_only_claim`, `reference_workspace_missing`, `certification_state_stale`

## Automatic downgrade

`derive_effective_badge` applies the resolved row to the claimed badge:

- A non-stable claim (`community`, `imported`, `local_draft`) passes through unchanged — it is already honest.
- A stable claim collects downgrade reasons from: a prose-only basis, a stale certification state, a missing/mismatched row, `stale_past_window` freshness (or `downgraded_freshness_expired`), narrowed bridge parity (or `downgraded_bridge_narrowed`), a disclosed gap (or `downgraded_gap_detected`), and a missing reference workspace when one is asserted.
- If any reason applies, the badge falls to `retest_pending` (freshness/certification-state expiry) or `limited` (every other failure), the support claim falls to `limited_retest_pending_claim`, and the reasons are recorded.

`validate()` recomputes the effective badge from the stored scorecard and rejects any packet whose stored badge, downgrade flag, or reasons drift from the derived truth, so a producer cannot hand-write a stronger claim than the evidence backs.

## How to verify

```bash
cargo test -p aureline-workspace --test certify_launch_bundles_imported_user_handoff_bundles_and_alpha
cargo test -p aureline-workspace --lib certify
```

Schema conformance of the materialized packets is covered against `schemas/review/certify-launch-bundles-imported-user-handoff-bundles-and.schema.json`.
