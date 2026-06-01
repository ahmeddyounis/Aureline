# Finalize extension bridge and certification scope and downgrade any non-qualified categories to preview

**Status:** Stable bridge-certification lane — implemented in `crates/aureline-extensions`.

## Goal

Close the M4 ecosystem line. Every claimed stable ecosystem row carries one canonical, checked-in certification truth: the **finalized bridge contract** the category drives (bridge kind, pinned bridge ABI version, bridge-finalized flag, enforcement-backed flag, control-plane boundary) and the **certification scope** that truth lives in (category, scope status, certification evidence source, conformance-passed flag), alongside the permission posture, compatibility, activation budget, and install/revocation/mirror posture. The **stability qualification** that truth is allowed to claim is derived, not asserted: when the evidence can no longer back a `stable` certification, the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. A category that is not in the certified scope is **downgraded to preview** rather than inheriting a stable badge from a certified neighbor. Marketplace result and detail rows, install review, the extension detail view, the bridge inspector, the certification dashboard, diagnostics, support exports, the CLI inspector, and release packets ingest this packet instead of cloning a "Certified" badge.

## Design principles

1. **Trust before catalog breadth** — A `stable` tier must be `evidence_backed`; a `catalog_asserted_only` basis narrows below Stable. A catalog row can never imply stable certification on its own (`allows_catalog_only_trust == false`).
2. **The bridge contract is finalized and enforced** — The bridge surface binding pins the bridge ABI version, records whether the bridge contract is **finalized** (frozen for the stable line), whether the boundary is **enforcement-backed**, and the control-plane boundary. An unfinalized bridge or a catalog-asserted (unenforced) bridge narrows to `preview`; an **unguarded** mutating control plane withdraws the row; an `advisory` control plane narrows to `beta`.
3. **The certification scope is honest** — The certification scope records the category, the scope status (`certified` / `provisional` / `excluded` / `deprecated_scope`), the certification evidence source, and whether conformance passed. Only a `certified` category with conformance passed and non-inherited evidence backs a stable claim. A `provisional`, `excluded`, or `deprecated_scope` category, a failed conformance, or inherited certification evidence is **downgraded to preview** — a non-qualified category never renders stable (`non_qualified_category_never_stable`).
4. **No ambient bridge privilege** — The permission posture carries declared and effective refs plus a `widened_on_bridge` flag. A bridge that widens authority withdraws the row outright (`allows_ambient_bridge_privilege == false`).
5. **Bounded activation cost** — The worst-case surface's activation cost carries a `budget_class` (`within_budget` / `over_budget` / `unbounded` / `not_measured`). An `unbounded` cost withdraws the row; `over_budget` narrows to `beta`; `not_measured` narrows to `preview` (`allows_unbounded_activation_cost == false`).
6. **Compatibility comes from the bridge scorecard** — The compatibility record carries a parity label, a scorecard ref, and a verified flag. An unsupported compatibility withdraws; a parity-limited compatibility narrows to `beta`; an unverified compatibility narrows to `preview`.
7. **Revocation, mirrorability, and install scope are visible** — The install posture carries the install scope (and whether it is disclosed), the revocation posture (`clean` / `advisory` / `quarantined` / `revoked`), the mirrorability, and rollback support. A quarantined or revoked posture withdraws; an advisory posture or a not-mirrorable row narrows to `beta`; an undisclosed install scope narrows to `preview`.
8. **No drift** — The effective tier, downgrade verdict, narrowing reasons, and banner are re-derived from the posture at validation time, so a stored packet cannot drift from its truth.

## Record kinds

| Record kind | Purpose |
|---|---|
| `stable_bridge_certification_scope_packet` | Top-level packet consumed by marketplace result / detail rows, install review, the extension detail view, the bridge inspector, the certification dashboard, diagnostics, support export, docs/help, release packets, and the CLI inspector. |
| `stable_bridge_certification_identity` | Certification-scope descriptor ref, row identity, package identity, pinned certification-scope version, publisher namespace, certification-evidence ref, publisher trust tier, lifecycle state. |
| `stable_bridge_surface_binding` | Bridge kind, pinned bridge ABI version, bridge surface ref, bridge-finalized flag, enforcement-backed flag, control-plane boundary. |
| `stable_bridge_certification_scope` | Certification category, scope status, certification evidence source, conformance-passed flag, conformance report ref. |
| `stable_bridge_certification_permission_posture` | Declared / effective permission refs, widened-on-bridge flag, re-consent-required flag. |
| `stable_bridge_certification_compatibility` | Compatibility label, scorecard ref, verified flag. |
| `stable_bridge_certification_activation_budget` | Worst-case surface activation-cost posture, measured-cost and ceiling refs. |
| `stable_bridge_certification_install_posture` | Install scope + disclosure, revocation posture, mirrorability, rollback support. |
| `stable_bridge_certification_qualification_claim` | Claimed tier, effective tier after the posture is applied, support claim, narrowing reasons. |
| `stable_bridge_certification_downgraded_banner` | Whether a row-review banner must display and why. |
| `stable_bridge_certification_inspection` | Compact boolean projection for CLI and dashboard surfaces. |
| `stable_bridge_certification_support_export` | Metadata-safe support / partner / mirror export row that preserves the certification scope, bridge finalization, and conformance posture so a reviewer can see why a category is or is not certified. |

## Narrowing buckets

| Tier | Triggered by (examples) |
|---|---|
| **withdrawn** | non-runnable lifecycle, unguarded bridge control plane, permission widened across the bridge, unsupported compatibility, unbounded activation cost, quarantined/revoked revocation posture |
| **preview** | unpublished certification-scope version, unpublished bridge ABI, catalog-asserted basis, quarantined trust tier, bridge contract not finalized, bridge not enforcement-backed, provisional/excluded/deprecated category scope, failed conformance, inherited certification evidence, unverified compatibility, unmeasured activation cost, undisclosed install scope, incomplete attribution |
| **beta** | advisory bridge control plane, parity-limited compatibility, over-budget activation cost, advisory revocation posture, not mirrorable |

The certification-scope gate lives in the **preview** bucket: a non-qualified category lands at preview (or lower if a more severe condition also applies), never stable.

## Canonical fixtures

Under `fixtures/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade/`:

- `certified_language_tools_bridge_stable.json` — a certified language-tools category on a finalized, enforcement-backed language bridge with a guarded control plane, conformance passed, full parity, activation within budget, clean revocation, mirrorable. It holds **Stable**.
- `certified_debugger_bridge_stable.json` — a certified debuggers category on a finalized debug bridge; holds **Stable**.
- `provisional_ai_assist_category_narrows_to_preview.json` — a provisional AI-assist category; every other axis is stable-grade, but the category is not yet certified, so it narrows to `preview`.
- `excluded_general_ui_category_narrows_to_preview.json` — a general-UI category excluded from the certified scope; narrows to `preview` with a review banner.
- `unfinalized_bridge_contract_narrows_to_preview.json` — a certified SCM-providers category whose bridge contract is not finalized; narrows to `preview`.
- `over_budget_activation_narrows_to_beta.json` — a certified formatters/linters category over its activation budget; narrows to `beta`.
- `widened_bridge_permission_withdrawn.json` — a data-infra adapter that widened authority across the bridge; the row is `withdrawn` with a banner.
- `unguarded_control_plane_withdrawn.json` — a task-runners category whose task bridge exposes an unguarded control plane; the row is `withdrawn` with a banner.

## How to verify

```bash
cargo test -p aureline-extensions finalize_extension_bridge
cargo run -q -p aureline-extensions --example dump_stable_bridge_certification_scope_records -- validate
```

Materialized packets for every fixture validate against
[`schemas/extensions/stable_bridge_certification_scope.schema.json`](../../../schemas/extensions/stable_bridge_certification_scope.schema.json)
(checked with a Draft 2020-12 validator).
