# Stabilize policy-pack diff/explain/export and admin-facing ecosystem governance on claimed enterprise lanes

**Status:** Stable policy-pack governance lane — implemented in `crates/aureline-extensions`.

## Goal

Make the **admin-facing governance** of an extension on a claimed **enterprise lane** inspectable, reviewable, and exportable on the stable ecosystem line. Every claimed stable row carries one canonical, checked-in governance truth: the **policy-pack diff** (base / target pack versions, completeness, added / removed / modified rule counts, breaking-change acknowledgement), the **policy-decision explanation** (the decision the active pack produces, the typed reason, the governing-rule ref, and whether the decision is explained), the **admin-facing export** (scope, mechanically-generated-vs-hand-authored source, redaction posture), the **enterprise-lane binding** (lane class, tenancy scope, attested-vs-asserted claim), the permission posture, compatibility, and install / activation-cost / revocation / mirror posture. The **stability qualification** that truth is allowed to claim is derived, not asserted: when the evidence can no longer back a `stable` governance claim, the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. The admin governance console, the policy-pack diff view, the policy-decision explainer, install review, the extension detail view, diagnostics, support exports, the CLI inspector, and release packets ingest this packet instead of cloning an "Enterprise-approved" badge.

## Design principles

1. **Policy-pack diffs are complete and reviewable** — The diff carries the base / target pack versions, the added / removed / modified rule counts, and a completeness class. A `missing` diff narrows to `preview`; a `partial` diff narrows to `beta`. The diff counts are cross-checked against the completeness and base / target versions at validation time, so a "complete" diff across differing pack versions that reports no changed rule is rejected outright.
2. **Breaking changes are acknowledged, not silent** — A diff that introduces a breaking change must record an admin acknowledgement. An unacknowledged breaking change narrows to `beta` and raises a review banner.
3. **Every governance decision is explained** — The explain record carries the decision (`allowed` / `constrained` / `blocked` / `quarantined`), the typed reason, and the governing-rule ref. A decision with no attached explanation narrows to `preview` — an admin (and the affected user) must be able to see *why* a row is allowed, constrained, or blocked. A correctly-explained `constrained` or `blocked` decision is a valid stable governance outcome.
4. **Admin exports are mechanically sourced and metadata-safe** — The export record carries the scope, source, and redaction posture. A hand-authored export — a governance claim with no mechanical source behind it — narrows to `preview` (catalog-only trust); a `decision_only` scope narrows to `beta`. An export carrying raw private bytes (`contains_private`) withdraws the row outright.
5. **Enterprise-lane claims are attested** — The lane binding carries the lane class, tenancy scope, and claim basis. An `asserted` (unattested) enterprise-lane claim narrows to `preview` — an enterprise claim may never be asserted on a stable row.
6. **No ambient extension privilege** — The permission posture carries declared / effective / policy-cap refs plus a `widened` flag. A permission set widened beyond the declared manifest or the policy cap withdraws the row (`allows_ambient_extension_privilege == false`).
7. **No unbounded activation cost** — The install posture carries a headline activation-cost class. An `unbounded` cost withdraws the row (`allows_unbounded_activation_cost == false`).
8. **No catalog-only trust** — A `stable` tier must be `evidence_backed`; a `catalog_asserted_only` basis narrows below Stable (`allows_catalog_only_trust == false`).
9. **Revocation, mirrorability, and install scope are visible** — A quarantined / revoked revocation posture withdraws; an advisory posture or a not-mirrorable row narrows to `beta`; an undisclosed install scope narrows to `preview`.
10. **No drift** — The effective tier, downgrade verdict, narrowing reasons, and banner are re-derived from the posture at validation time, so a stored packet cannot drift from its truth.

## Record kinds

| Record kind | Purpose |
|---|---|
| `stable_policy_pack_governance_packet` | Top-level packet consumed by the admin governance console, the policy-pack diff view, the policy-decision explainer, install review, the extension detail view, diagnostics, support export, docs/help, release packets, and the CLI inspector. |
| `stable_policy_governance_identity` | Governance-profile ref, row identity, package identity, policy-pack id and version, pinned profile version, admin and publisher namespaces, governance-evidence ref, publisher trust tier, lifecycle state. |
| `stable_policy_pack_diff` | Base / target pack versions, completeness, added / removed / modified rule counts, breaking-change and acknowledgement flags, diff ref. |
| `stable_policy_decision_explain` | Decision, typed reason, governing-rule ref, explained flag, explanation ref. |
| `stable_admin_governance_export` | Export scope, source (mechanical vs hand-authored), redaction posture, export ref. |
| `stable_enterprise_lane_binding` | Enterprise-lane class, claim basis (attested vs asserted), tenancy scope, attestation ref. |
| `stable_policy_governance_permission_posture` | Declared / effective / policy-cap permission refs, no-widening flag, re-consent flag. |
| `stable_policy_governance_compatibility` | Compatibility label, scorecard ref, verified flag. |
| `stable_policy_governance_install_posture` | Install scope + disclosure, activation cost class, revocation posture, mirrorability, rollback support. |
| `stable_policy_governance_qualification_claim` | Claimed tier, effective tier after the posture is applied, support claim, narrowing reasons. |
| `stable_policy_governance_downgraded_banner` | Whether an admin-review banner must display and why. |
| `stable_policy_pack_governance_inspection` | Compact projection (diff / decision / export / lane posture) for CLI and dashboard surfaces. |
| `stable_policy_pack_governance_support_export` | Metadata-safe support / partner / mirror export row that preserves the diff / decision / export posture so a reviewer can see why a row is or is not a stable governance claim. |

## Narrowing buckets

| Tier | Triggered by (examples) |
|---|---|
| **withdrawn** | non-runnable lifecycle, permission widened beyond the declared manifest / policy cap, unbounded activation cost, export carrying raw private data, unsupported compatibility, quarantined/revoked revocation posture |
| **preview** | unpublished profile version, catalog-asserted basis, quarantined trust tier, missing diff, unexplained decision, hand-authored export, unattested enterprise lane, unverified compatibility, undisclosed install scope, incomplete attribution |
| **beta** | partial diff, unacknowledged breaking change, decision-only export scope, parity-limited compatibility, advisory revocation posture, not mirrorable |

## Canonical fixtures

Under `fixtures/extensions/m4/stabilize-policy-pack-diff-explain-export-and-admin/`:

- `within_policy_managed_lane_stable.json` — allow decision, complete diff, attested managed lane, mechanically-generated metadata-safe export. Holds **Stable**.
- `constrained_air_gapped_lane_stable.json` — a policy-capped (constrained) decision with an acknowledged breaking change on an attested air-gapped lane; still **Stable**.
- `partial_diff_narrows_to_beta.json` — the policy-pack diff is only partial; narrows to `beta`.
- `unexplained_decision_narrows_to_preview.json` — a blocked decision with no attached explanation; narrows to `preview`.
- `hand_authored_export_narrows_to_preview.json` — the governance export is hand-authored rather than mechanically generated; narrows to `preview`.
- `asserted_enterprise_lane_narrows_to_preview.json` — the enterprise-lane claim is asserted, not attested; narrows to `preview`.
- `private_export_withdrawn.json` — the export carries raw private bytes; the row is `withdrawn` with a banner.
- `widened_permission_withdrawn.json` — effective permissions widened beyond the declared manifest and policy cap; the row is `withdrawn` with a banner.

## How to verify

```bash
cargo test -p aureline-extensions stabilize_policy_pack
cargo run -q -p aureline-extensions --example dump_stable_policy_pack_governance_records -- validate
```

Materialized packets for every fixture validate against
[`schemas/extensions/stable_policy_pack_governance.schema.json`](../../../schemas/extensions/stable_policy_pack_governance.schema.json)
(checked with a Draft 2020-12 validator).
