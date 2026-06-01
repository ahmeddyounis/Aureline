# Stabilize mirror/manual import, offline catalog, publisher-transfer continuity, and namespace-trust flows

**Status:** Stable mirror/manual import-truth lane — implemented in `crates/aureline-extensions`.

## Goal

Promote the imported / mirrored / offline marketplace row into the **stable line**. Every claimed stable row that arrives through a primary catalog, an approved mirror, an offline bundle, or a manual artifact carries one canonical, checked-in import truth: the **source class** (route, visibility, source-class-preserved flag, last-known-good pin, and offline-explainable flag), the **publisher-transfer continuity** binding (the continuity event and state, cooldown, audit trail and lineage, user/admin notification, high-trust auto-update gating, and transfer-history preservation), the **mapping outcome** generated from the real imported artifact (exact / translated / partial / shimmed / unsupported, with a preserved rollback checkpoint and diagnostics on failure), the **permission posture**, the **compatibility** label, the **activation-budget** instrumentation, and the **install posture** (install scope and disclosure, revocation posture, mirrorability, rollback support). The **stability qualification** that truth is allowed to claim is derived, not asserted: when the evidence can no longer back a `stable` import claim, the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. Marketplace result and detail rows, install / side-load / mirror-bundle review, the offline catalog view, diagnostics, support exports, the CLI inspector, and release packets ingest this packet instead of cloning a "Trusted" badge or going dark when a publisher is revoked or rehomed.

## Design principles

1. **Trust before catalog breadth** — A `stable` tier must be `evidence_backed`; a `catalog_asserted_only` basis narrows below Stable. A catalog or mirror row can never imply stable trust on its own (`allows_catalog_only_trust == false`).
2. **No ambient privilege on import** — The permission posture carries declared and effective refs plus a `widened_on_import` flag. A mirror or manual import that widens authority withdraws the row outright (`allows_ambient_privilege == false`).
3. **Source class travels and stays explainable offline** — The source class records the route (`primary_catalog` / `approved_mirror` / `offline_bundle` / `manual_artifact`), the visibility, whether the publisher / source class is preserved, whether a **last-known-good** is pinned, and whether the row stays explainable offline. A revoked or rehomed package keeps its last-known-good pin and stays explainable without a live status fetch; an unexplainable-offline row narrows and raises a banner.
4. **Publisher-transfer continuity is gated** — Namespace reservation, ownership transfer, signer/key rotation, orphaning/succession, and approved-mirror promotion each carry a state, a cooldown, an audit trail with a preserved lineage, user/admin notification, and a high-trust auto-update gate. For any transfer event, high-trust auto-update must be gated behind delay, audit, and notification before it resumes — an ungated transfer auto-update withdraws the row. A disputed or revoked continuity withdraws; an orphaned, missing, or pending-notification continuity narrows to `preview`; an in-cooldown or stale continuity narrows to `beta`.
5. **Outcome labels come from the real artifact** — The mapping outcome (`exact` / `translated` / `partial` / `shimmed` / `unsupported`) is generated from the real imported artifact (`generated_from_real_artifact`). An unsupported outcome withdraws; a shimmed or partial outcome narrows to `beta`; an outcome not generated from the real artifact narrows to `preview`.
6. **Rollback checkpoints survive failure** — When a mapping fails, the rollback checkpoint and diagnostics must be preserved; a failed mapping with no preserved checkpoint withdraws the row and raises a banner.
7. **Bounded activation cost** — The worst-case surface's activation cost carries a `budget_class` (`within_budget` / `over_budget` / `unbounded` / `not_measured`). An `unbounded` cost withdraws the row; `over_budget` narrows to `beta`; `not_measured` narrows to `preview` (`allows_unbounded_activation_cost == false`).
8. **Revocation, mirrorability, and install scope are visible** — The install posture carries the install scope (and whether it is disclosed), the revocation posture (`clean` / `advisory` / `quarantined` / `revoked`), the mirrorability, and rollback support. A quarantined or revoked posture withdraws; an advisory posture or a not-mirrorable row narrows to `beta`; an undisclosed install scope narrows to `preview`.
9. **No drift** — The effective tier, downgrade verdict, narrowing reasons, and banner are re-derived from the posture at validation time, so a stored packet cannot drift from its truth.

## Record kinds

| Record kind | Purpose |
|---|---|
| `stable_mirror_import_truth_packet` | Top-level packet consumed by marketplace result / detail rows, install / side-load / mirror-bundle review, the offline catalog view, diagnostics, support export, docs/help, release packets, and the CLI inspector. |
| `stable_mirror_import_truth_identity` | Catalog descriptor ref, row identity, package identity, pinned import-truth version, publisher namespace, source artifact ref, publisher trust tier, lifecycle state. |
| `stable_mirror_import_source_class` | Import route, source visibility, source-class-preserved flag, last-known-good ref + pin, offline-explainable flag. |
| `stable_mirror_import_continuity` | Publisher-transfer continuity event and state, cooldown, audit trail + lineage, user/admin notification, high-trust auto-update gate, transfer-history preservation, continuity-packet ref. |
| `stable_mirror_import_mapping_outcome` | Outcome class, generated-from-real-artifact flag, rollback-checkpoint ref + preserved flag, diagnostics ref, mapping-failed flag. |
| `stable_mirror_import_permission_posture` | Declared / effective permission refs, widened-on-import flag, re-consent-required flag. |
| `stable_mirror_import_compatibility` | Compatibility label, compatibility-window ref, evidence source, verified flag. |
| `stable_mirror_import_activation_budget` | Worst-case surface activation-cost posture, measured-cost and ceiling refs. |
| `stable_mirror_import_install_posture` | Install scope + disclosure, revocation posture, mirrorability, rollback support. |
| `stable_mirror_import_truth_qualification_claim` | Claimed tier, effective tier after the posture is applied, support claim, narrowing reasons. |
| `stable_mirror_import_downgraded_banner` | Whether a row-review banner must display and why. |
| `stable_mirror_import_truth_inspection` | Compact boolean projection for CLI and dashboard surfaces. |
| `stable_mirror_import_truth_support_export` | Metadata-safe support / partner / mirror export row that preserves source class, continuity, transfer history, and last-known-good pinning so a revoked or rehomed package stays explainable offline. |

## Narrowing buckets

| Tier | Triggered by (examples) |
|---|---|
| **withdrawn** | non-installable lifecycle, disputed/revoked continuity, ungated transfer auto-update, unsupported mapping, missing rollback checkpoint on failure, permission widened on import, unsupported compatibility, unbounded activation cost, quarantined/revoked revocation posture |
| **preview** | unpublished import version, catalog-asserted basis, quarantined trust tier, source class not preserved, not explainable offline, no last-known-good, pending-notification/orphaned/missing continuity, incomplete audit lineage, unpreserved transfer history, missing user/admin notification, mapping not from the real artifact, inherited/unverified compatibility, unmeasured activation cost, undisclosed install scope, incomplete attribution |
| **beta** | in-cooldown/stale continuity, shimmed/partial mapping, parity-limited compatibility, over-budget activation cost, advisory revocation posture, not mirrorable |

## Canonical fixtures

Under `fixtures/extensions/m4/stabilize-mirror-manual-import-offline-catalog-and-publisher/`:

- `verified_publisher_offline_bundle_stable.json` — a verified-publisher offline-bundle import; source class preserved, last-known-good pinned, continuity current, exact mapping from the real artifact, no permission widening, full parity, activation within budget, clean revocation, mirrorable. It holds **Stable**.
- `approved_mirror_promotion_settled_stable.json` — an approved-mirror promotion whose cooldown is satisfied, audit lineage preserved, and user/admin notified before high-trust auto-update resumed. The transfer event still holds **Stable** because it was gated correctly.
- `signer_key_rotation_in_cooldown_narrows_to_beta.json` — a signer key rotation in cooldown (auto-update gated, both parties notified, audit preserved); it narrows to `beta`.
- `ownership_transfer_pending_notification_narrows_to_preview.json` — an ownership transfer pending user/admin notification; it narrows to `preview`.
- `namespace_dispute_withdrawn.json` — a namespace reservation under dispute; the row is `withdrawn`, a review banner is raised, and the last-known-good keeps it explainable offline.
- `orphan_succession_narrows_to_preview.json` — an orphaned publisher awaiting successor adoption; it narrows to `preview` with a banner.
- `manual_artifact_shimmed_narrows_to_beta.json` — a manual artifact whose mapping required a shim; it narrows to `beta`.
- `unsupported_mapping_failed_withdrawn.json` — a manual artifact whose mapping failed as unsupported; the row is `withdrawn` while the rollback checkpoint and diagnostics are preserved.

## How to verify

```bash
cargo test -p aureline-extensions stabilize_mirror_manual_import
cargo run -q -p aureline-extensions --example dump_stable_mirror_import_truth_records -- validate
```

Materialized packets for every fixture validate against
[`schemas/extensions/stable_mirror_import_truth.schema.json`](../../../schemas/extensions/stable_mirror_import_truth.schema.json)
(checked with a Draft 2020-12 validator).
