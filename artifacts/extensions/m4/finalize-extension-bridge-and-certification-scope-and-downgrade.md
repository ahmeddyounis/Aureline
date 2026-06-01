# Artifact: Finalize extension bridge and certification scope and downgrade any non-qualified categories to preview

**Task:** Close the M4 ecosystem line by binding, for a certified extension category, the finalized bridge contract it drives (bridge kind, pinned bridge ABI version, bridge-finalized flag, enforcement-backed flag, control-plane boundary), the certification scope (category, scope status, certification evidence source, conformance-passed flag), the permission posture, compatibility, activation budget, and install/revocation/mirror posture into one validated packet, and derive the stability qualification with automatic narrowing below Stable — so a non-qualified category is downgraded to preview rather than inheriting a stable badge from an adjacent green category.

**Status:** Implemented
**Verification class:** Conformance / interoperability suite + Security / privacy review + Docs validation + Release evidence review

## Summary

This lane binds, for a stable ecosystem certification row, the identity (certification-scope descriptor ref, row identity, package identity, pinned certification-scope version, publisher namespace, pinned certification-evidence ref, publisher trust tier, lifecycle state), the **bridge surface binding** (bridge kind `language_bridge` / `debug_bridge` / `scm_bridge` / `task_bridge` / `terminal_bridge` / `filesystem_bridge` / `search_bridge` / `ui_panel_bridge` / `data_infra_bridge`, pinned bridge ABI version, bridge-finalized flag, enforcement-backed flag, control-plane boundary `guarded` / `advisory` / `unguarded`), the **certification scope** (category, scope status `certified` / `provisional` / `excluded` / `deprecated_scope`, certification evidence source, conformance-passed flag), the permission posture (declared / effective refs, widened-on-bridge flag), the compatibility label, the worst-case activation-budget instrumentation, and the install posture (install scope + disclosure, revocation posture, mirrorability, rollback support) into one validated packet, and derives the stability qualification it may claim. A `stable` bridge-certification claim is only allowed when the row pins the published certification-scope version and bridge ABI version, is evidence-backed, keeps its publisher trust tier out of quarantine, stays runnable, finalizes its bridge contract, keeps the bridge enforcement-backed with a guarded control plane, keeps its category inside the certified scope with conformance passed and non-inherited certification evidence, never widens permissions across the bridge, keeps verified non-parity-limited compatibility, keeps its activation cost bounded and within budget, discloses its install scope, keeps a clean revocation posture, stays mirrorable, and is fully attributed.

The headline behavior is the **certification-scope gate**: a category that is not in the certified scope — `provisional`, `excluded`, or `deprecated_scope`, or with failed conformance or inherited certification evidence — is **downgraded to preview**, never left rendering stable from an adjacent green category. A non-runnable lifecycle, an unguarded bridge control plane, a permission widened across the bridge, an unsupported compatibility, an unbounded activation cost, or a quarantined / revoked revocation posture withdraws the row; an advisory control plane, a parity-limited compatibility, an over-budget activation cost, an advisory revocation posture, or a not-mirrorable row narrows to `beta`. When any condition fails the visible tier is automatically narrowed below Stable with machine-readable reasons. The checked-in packet is canonical: marketplace result / detail rows, install review, the extension detail view, the bridge inspector, the certification dashboard, diagnostics, support exports, the CLI inspector, and release packets ingest it instead of cloning a "Certified" badge.

## What changed

- New Rust module: `crates/aureline-extensions/src/finalize_extension_bridge_and_certification_scope_and_downgrade/mod.rs` (+ `tests.rs`)
- Re-exported from `crates/aureline-extensions/src/lib.rs`
- New schema: `schemas/extensions/stable_bridge_certification_scope.schema.json`
- New fixtures: `fixtures/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade/`
  - `certified_language_tools_bridge_stable.json` — a certified language-tools category on a finalized language bridge; holds Stable.
  - `certified_debugger_bridge_stable.json` — a certified debuggers category on a finalized debug bridge; holds Stable.
  - `provisional_ai_assist_category_narrows_to_preview.json` — a provisional AI-assist category; narrows to `preview` (the headline downgrade).
  - `excluded_general_ui_category_narrows_to_preview.json` — a general-UI category excluded from the certified scope; narrows to `preview` with a banner.
  - `unfinalized_bridge_contract_narrows_to_preview.json` — a certified SCM-providers category whose bridge contract is not finalized; narrows to `preview`.
  - `over_budget_activation_narrows_to_beta.json` — a certified formatters/linters category over its activation budget; narrows to `beta`.
  - `widened_bridge_permission_withdrawn.json` — a data-infra adapter that widened authority across the bridge; `withdrawn` with a banner.
  - `unguarded_control_plane_withdrawn.json` — a task-runners category whose task bridge exposes an unguarded control plane; `withdrawn` with a banner.
- New dump example: `crates/aureline-extensions/examples/dump_stable_bridge_certification_scope_records.rs`
- New docs: `docs/extensions/m4/finalize-extension-bridge-and-certification-scope-and-downgrade.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current and self-describing (schema, fixtures, and docs reference one another) rather than ad hoc notes. (`stable_bridge_certification_scope.schema.json`, fixtures dir, this packet)
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable, with machine-readable reasons, instead of inheriting an adjacent green row. (`provisional_ai_assist_category_narrows_to_preview.json`, `excluded_general_ui_category_narrows_to_preview.json`, `unfinalized_bridge_contract_narrows_to_preview.json`, `over_budget_activation_narrows_to_beta.json`, `widened_bridge_permission_withdrawn.json`, `unguarded_control_plane_withdrawn.json`)
- [x] Non-qualified categories are downgraded to preview. (`provisional_category_narrows_to_preview`, `excluded_category_narrows_to_preview_with_banner`, `deprecated_scope_narrows_to_preview`, `failed_conformance_narrows_to_preview`, `inherited_certification_evidence_narrows_to_preview`; `non_qualified_category_never_stable` invariant)
- [x] Users and admins can inspect permissions (permission posture, declared/effective refs), compatibility range (compatibility label + scorecard ref), activation cost (activation budget), lifecycle label, publisher provenance (trust tier + certification scope), and rollback/revocation state (revocation posture + rollback support) for the touched ecosystem row. (`stable_bridge_certification_inspection`, `stable_bridge_certification_permission_posture`, `stable_bridge_certification_compatibility`, `stable_bridge_certification_activation_budget`, `stable_bridge_certification_scope`, `stable_bridge_certification_install_posture`)
- [x] Conformance fixtures, activation-budget instrumentation, and publisher continuity packets make the ecosystem claims supportable and mirrorable on the M4 line. (`conformance_report_ref` + `conformance_passed`, `stable_bridge_certification_activation_budget`, `mirrorability_class`, all eight fixtures)
- [x] One stable manifest / permission / lifecycle / compatibility vocabulary is shared across install review, runtime, mirror/manual import, disable/rollback, and revocation paths. (trust-tier, lifecycle, certification-evidence, activation-budget, install-scope, revocation-posture, mirrorability, and stability-tier vocabularies are the same closed string sets shared with the catalog-truth, lifecycle-flow, external-host, and mirror-import stable lanes.)

## Guardrails honored

- No ambient bridge privilege: a permission set widened across the bridge withdraws the row (`widened_bridge_permission_withdraws_the_row`); `allows_ambient_bridge_privilege` is pinned false.
- No catalog-only trust: a `catalog_asserted_only` basis can never back a stable claim (`catalog_asserted_basis_cannot_back_stable`, `no_catalog_only_stable_claim`); `allows_catalog_only_trust` is pinned false.
- No unbounded activation cost: an `unbounded` budget withdraws the row (`unbounded_activation_cost_withdraws_the_row`); `over_budget` narrows to `beta`; `allows_unbounded_activation_cost` is pinned false.
- No widened public scope from this row alone: a non-certified category never renders stable (`non_qualified_category_never_stable`); the certification-scope gate lands every out-of-scope category at preview or below.
- A narrower stable claim is published rather than papered over: the effective tier, downgrade flag, reasons, and banner are re-derived from the posture at validation time, so the packet cannot drift.

## How to verify

```bash
cargo test -p aureline-extensions finalize_extension_bridge
cargo run -q -p aureline-extensions --example dump_stable_bridge_certification_scope_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_bridge_certification_scope.schema.json` (checked with a Draft 2020-12 validator).

## Risks / follow-ups

- The bridge kind, scope status, and certification evidence source are producer-supplied closed strings; when the certification registry exposes typed enums, these should be sourced from that model directly rather than re-declared as strings here.
- The certification scope carries a single category + scope status; a later revision could carry a per-bridge-capability scorecard so a reviewer can see exactly which bridge surfaces a category certified against.
- The activation budget is summarized as a single worst-case surface posture; a later revision should carry a per-surface activation row so a reviewer can see which surface drove the worst case.
- Trust-tier, lifecycle, certification-evidence, activation-budget, install-scope, revocation-posture, mirrorability, stability-tier, and claim-basis vocabularies are closed string sets shared with the catalog-truth, lifecycle-flow, external-host, and mirror-import stable lanes; when those crates stabilize typed enums, these should be narrowed to share them rather than re-declared as strings.
- The compatibility record carries a single label + scorecard ref; a later revision should resolve the bound scorecard's own freshness rather than accepting a producer-supplied `compatibility_label_class`.
