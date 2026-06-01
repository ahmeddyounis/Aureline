# Artifact: Stabilize policy-pack diff/explain/export and admin-facing ecosystem governance on claimed enterprise lanes

**Task:** Make the admin-facing governance of an extension on a claimed enterprise lane inspectable, reviewable, and exportable on the stable line — binding the policy-pack diff (base / target versions, added / removed / modified rule counts, completeness, breaking-change acknowledgement), the policy-decision explanation (decision, typed reason, governing-rule ref, explained flag), the admin-facing export (scope, source, redaction posture), the enterprise-lane binding (lane class, claim basis, tenancy scope), the permission posture, compatibility, and install / activation-cost / revocation / mirror posture into one validated packet, and derive the stability qualification with automatic narrowing below Stable.

**Status:** Implemented
**Verification class:** Conformance / interoperability suite + Security / privacy review + Docs validation + Release evidence review

## Summary

This lane binds, for a stable admin-facing governance row, the identity (governance-profile descriptor ref, row identity, package identity, policy-pack id and version, the pinned governance-profile version, the admin and publisher namespaces, the pinned governance-evidence ref, publisher trust tier, lifecycle state), the **policy-pack diff** (base / target pack versions, diff completeness `complete` / `partial` / `missing`, added / removed / modified rule counts, breaking-change and acknowledgement flags), the **policy-decision explain** (decision `allowed` / `constrained` / `blocked` / `quarantined`, typed reason `within_policy` / `permission_capped` / `publisher_below_trust_floor` / `version_out_of_range` / `on_admin_denylist` / `superseded_by_pack`, governing-rule ref, explained flag, explanation ref), the **admin-facing export** (scope `full` / `summary` / `decision_only`, source `mechanically_generated` / `hand_authored`, redaction `metadata_safe` / `contains_private`, export ref), the **enterprise-lane binding** (lane class, claim basis `attested` / `asserted`, tenancy scope, attestation ref), the permission posture (declared / effective / policy-cap refs, widened flag), the compatibility label, and the install posture (install scope, activation cost class, revocation posture, mirrorability, rollback) into one validated packet, and derives the stability qualification it may claim. A `stable` governance claim is only allowed when the row pins the published governance-profile version, is evidence-backed, keeps its trust tier out of quarantine, stays runnable, carries a complete diff with no unacknowledged breaking change, explains its decision, emits a mechanically-generated metadata-safe export at full / summary scope, attests its enterprise lane, never widens permissions beyond the declared manifest or the policy cap, keeps its activation cost bounded, keeps verified non-parity-limited compatibility, discloses its install scope, keeps a clean revocation posture, stays mirrorable, and is fully attributed.

When any condition fails the visible tier is automatically narrowed below Stable with machine-readable reasons. A permission widened beyond the declared manifest or policy cap, an unbounded activation cost, a governance export carrying raw private data, a non-runnable lifecycle, an unsupported compatibility, or a quarantined / revoked revocation posture withdraws the row; a missing diff, an unexplained decision, a hand-authored export, an unattested enterprise lane, an unpublished profile version, a catalog-asserted basis, a quarantined trust tier, an unverified compatibility, an undisclosed install scope, or incomplete attribution narrows to `preview`; a partial diff, an unacknowledged breaking change, a decision-only export scope, a parity-limited compatibility, an advisory revocation posture, or a not-mirrorable row narrows to `beta`. The diff counts are cross-checked against the completeness and base / target versions so a diff record cannot be internally inconsistent. The checked-in packet is canonical: the admin governance console, the policy-pack diff view, the policy-decision explainer, install review, the extension detail view, diagnostics, support exports, the CLI inspector, and release packets ingest it instead of cloning an "Enterprise-approved" badge.

## What changed

- New Rust module: `crates/aureline-extensions/src/stabilize_policy_pack_diff_explain_export_and_admin/mod.rs` (+ `tests.rs`)
- Re-exported from `crates/aureline-extensions/src/lib.rs`
- New schema: `schemas/extensions/stable_policy_pack_governance.schema.json`
- New fixtures: `fixtures/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin/`
  - `within_policy_managed_lane_stable.json` — allow decision, complete diff, attested managed lane, mechanically-generated metadata-safe export; holds Stable.
  - `constrained_air_gapped_lane_stable.json` — a policy-capped (constrained) decision with an acknowledged breaking change on an attested air-gapped lane; still Stable.
  - `partial_diff_narrows_to_beta.json` — the policy-pack diff is only partial; narrows to `beta`.
  - `unexplained_decision_narrows_to_preview.json` — a blocked decision with no attached explanation; narrows to `preview`.
  - `hand_authored_export_narrows_to_preview.json` — the governance export is hand-authored, not mechanically generated; narrows to `preview`.
  - `asserted_enterprise_lane_narrows_to_preview.json` — the enterprise-lane claim is asserted, not attested; narrows to `preview`.
  - `private_export_withdrawn.json` — the export carries raw private bytes; `withdrawn` with a banner.
  - `widened_permission_withdrawn.json` — effective permissions widened beyond the declared manifest and policy cap; `withdrawn` with a banner.
- New dump example: `crates/aureline-extensions/examples/dump_stable_policy_pack_governance_records.rs`
- New docs: `docs/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current and self-describing (schema, fixtures, and docs reference one another) rather than ad hoc notes. (`stable_policy_pack_governance.schema.json`, fixtures dir, this packet)
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable, with machine-readable reasons, instead of inheriting an adjacent green row. (`partial_diff_narrows_to_beta.json`, `unexplained_decision_narrows_to_preview.json`, `hand_authored_export_narrows_to_preview.json`, `asserted_enterprise_lane_narrows_to_preview.json`, `private_export_withdrawn.json`, `widened_permission_withdrawn.json`)
- [x] Users and admins can inspect permissions (permission posture, declared / effective / policy-cap refs), compatibility range (compatibility label + scorecard ref), activation cost (`activation_cost_class`), lifecycle label, publisher provenance (trust tier + governance profile + admin namespace), and rollback / revocation state (revocation posture + rollback support) for the touched ecosystem row. (`stable_policy_pack_governance_inspection`, `stable_policy_governance_install_posture`, `stable_policy_governance_permission_posture`, `stable_policy_governance_compatibility`, `stable_policy_governance_identity`)
- [x] Conformance fixtures, activation-budget instrumentation, and publisher continuity packets make the ecosystem claims supportable and mirrorable on the M4 line. (`activation_cost_class`, `mirrorability_class`, `governance_evidence_ref`, all eight fixtures, the metadata-safe support export)
- [x] One stable manifest / permission / lifecycle / compatibility vocabulary is shared across install review, runtime, mirror / manual import, disable / rollback, and revocation paths. (trust-tier, lifecycle, compatibility-label, install-scope, revocation-posture, mirrorability, stability-tier, and claim-basis vocabularies are the same closed string sets shared with the catalog-truth, lifecycle-flow, bridge-certification, mirror-import, and performance-budget stable lanes.)

## Guardrails honored

- No ambient extension privilege: a permission set widened beyond the declared manifest or policy cap withdraws the row (`widened_permission_withdraws_the_row`); `allows_ambient_extension_privilege` is pinned false. A governance export carrying raw private data also withdraws (`private_export_withdraws_the_row`).
- No catalog-only trust: a `catalog_asserted_only` basis can never back a stable claim (`catalog_asserted_basis_cannot_back_stable`, `no_catalog_only_stable_claim`), and a hand-authored export — a governance claim with no mechanical source behind it — narrows to `preview`; `allows_catalog_only_trust` is pinned false.
- No unbounded activation cost: an `unbounded` activation-cost class withdraws the row (`unbounded_activation_cost_withdraws_the_row`); `allows_unbounded_activation_cost` is pinned false.
- No widened public scope from this row alone: the packet only narrows; it never promotes a row to a wider claim than the posture supports.
- A narrower stable claim is published rather than papered over: the effective tier, downgrade flag, reasons, and banner are re-derived from the posture at validation time, and the diff counts are cross-checked against the completeness, so the packet cannot drift.

## How to verify

```bash
cargo test -p aureline-extensions stabilize_policy_pack
cargo run -q -p aureline-extensions --example dump_stable_policy_pack_governance_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_policy_pack_governance.schema.json` (checked with a Draft 2020-12 validator).

## Risks / follow-ups

- The policy-decision `reason_class` vocabulary is a fixed closed set; when the governance crate exposes a typed policy-rule decision model, the reason should be sourced from it directly rather than re-declared as a parallel string set here.
- The diff carries added / removed / modified rule *counts*; a later revision should carry a per-rule change list (rule id + change kind) so the diff view can render the individual rule deltas rather than aggregate counts.
- The enterprise-lane attestation is carried by a claim basis + attestation ref; a later revision should resolve the bound attestation's own scope and expiry rather than accepting a producer-supplied `lane_claim_basis_class`.
- The activation cost is a producer-supplied closed string here; when the stable performance-budget lane's measured cost is available for a row, this should ingest that measurement instead of re-declaring a bare cost class.
- Trust-tier, lifecycle, compatibility-label, install-scope, revocation-posture, mirrorability, stability-tier, and claim-basis vocabularies are closed string sets shared with the other stable extension lanes; when those crates stabilize typed enums, these should be narrowed to share them rather than re-declared as strings.
- The admin / policy-pack governance here is modeled for one governed row; a later revision should carry a pack-level rollup so an admin can see the whole pack's governed-extension posture in one packet rather than one row at a time.
