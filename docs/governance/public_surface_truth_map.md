# Public-surface truth map

This document is the narrative companion to
[`/artifacts/governance/source_of_truth_map.yaml`](../../artifacts/governance/source_of_truth_map.yaml).
It exists so product UI, docs, CLI/headless output, support exports,
release packets, and public-proof packets can answer one question
mechanically: which artifact owns this truth row?

Related contracts:

- [`/docs/governance/claim_manifest_contract.md`](./claim_manifest_contract.md)
  — canonical claim-row publication contract.
- [`/docs/docs/help_about_service_health_routes.md`](../docs/help_about_service_health_routes.md)
  — canonical route and destination-descriptor contract.
- [`/docs/adr/0017-release-posture-artifact-families-and-promotion-gates.md`](../adr/0017-release-posture-artifact-families-and-promotion-gates.md)
  and
  [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
  — release-governance decision and stage-specific promotion gates that
  say when public-truth drift is release-blocking rather than merely a
  docs defect.
- [`/docs/governance/drift_blocking_rules.md`](./drift_blocking_rules.md)
  — severity classes, same-change-set rules, and audit procedure for
  truth drift.
- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — control-graph discovery home for this map.

## Scope

This map does not replace the owning contracts. It names the canonical
owner artifact for public/support-facing truth classes that are already
seeded elsewhere in the repository, then records which downstream
surfaces are allowed to project that truth.

If a downstream surface disagrees with the canonical owner artifact, the
canonical owner artifact wins. The downstream surface must narrow,
defer, or remove its wording; it may not invent a broader story.

## Canonical owner map

| Truth class | Canonical owner artifact | Row identity or tuple | What downstream surfaces project |
|---|---|---|---|
| Capability lifecycle | `schemas/governance/capability_lifecycle.schema.json` | `capability_lifecycle_row_id`, `dependency_marker_id` | settings, install review, docs/help, support exports, claim rows |
| Policy-disabled state | `artifacts/governance/experiments_register.yaml` | `id` | Labs projection, docs/help, claim rows, support/export caveats |
| Experiment and Labs state | `artifacts/governance/experiments_register.yaml` | `id` | `artifacts/governance/labs_register.yaml`, docs/help, release/support caveats |
| Service-contract state | `artifacts/governance/stable_surface_inventory.yaml` | `surface_id` | docs/help, compatibility reviews, migration notes, support/export visibility |
| Install mode | `artifacts/release/install_topology_matrix.yaml` | `(install_mode_class, channel_class, platform_class, architecture_class)` | About/help, diagnostics, support bundles, rollout and shiproom review |
| Build or release channel | `schemas/build/exact_build_identity.schema.json` | `exact_build_identity_ref` | About/help, docs version-match, release packets, support bundles, public proof |
| Provenance | `schemas/build/exact_build_identity.schema.json` | `exact_build_identity_ref` | About/provenance, release packets, support bundles, public-proof packets |
| Version skew | `artifacts/compat/version_skew_register.yaml` | `register_id`, `skew_case_id` | claim rows, compatibility reports, migration notes, support/export posture |
| Compatibility | `artifacts/compat/qualification_matrix_seed.yaml` | `row_id` | claim rows, certified-archetype packets, release evidence, support/export posture |
| Support-window truth | `schemas/governance/claim_manifest.schema.json` | `claim_row_id` | docs, release notes, CLI/help, support exports, evaluation and proof packets |
| Known-limit and exclusion notes | `schemas/governance/claim_manifest.schema.json` | `claim_row_id` plus `known_limit_refs` / `exclusion_note_refs` | docs/help, release packets, support exports, proof packets, migration notes |

## Owner-routing rules

1. Lifecycle, freshness, client-scope, and dependency-marker posture
   route through the capability-lifecycle contract before any claim row,
   Help/About badge, or support export renders them.
2. Policy disables, kill switches, public labels, expiry/review dates,
   and fallback behavior route through the experiment register; the
   Labs register is a projection, not a second owner.
3. Surface maturity, downgrade behavior, versioning, and service-level
   contract posture route through the stable-surface inventory.
4. Install mode, updater owner, durable-state-root separation, rollout
   ring, and unattended deployment posture route through the install
   topology matrix.
5. Build channel, artifact family, and provenance route through the
   exact-build identity record. Version-only fallback is not allowed.
6. Compatibility and skew posture route through the qualification
   matrix and the version-skew register before public copy is widened.
7. Support-window state, public claim posture, known limits, and
   exclusion notes route through the claim manifest. Release notes,
   docs/help, CLI/help, support exports, and public-proof packets are
   projections of that row, not alternate owners.
8. Release-facing widening additionally obeys the release-posture ADR
   and promotion-gate map: if a downstream surface widens beyond the
   canonical owner row, shiproom treats that as a promotion blocker even
   when the binary itself is already built.

## Audit checklist

Use this checklist whenever a reviewer, support engineer, docs owner, or
shiproom packet needs to trace a user-visible truth row:

1. Identify the row and classify it with
   [`source_of_truth_map.yaml`](../../artifacts/governance/source_of_truth_map.yaml).
2. Resolve the canonical owner artifact and the row identity rule for
   that truth class.
3. Read the owner row first. Do not start from the downstream surface.
4. Follow the owner row's evidence joins:
   exact-build ref, compatibility row, version-skew register,
   destination descriptor, or contract packet as applicable.
5. If the row is claim-bearing, resolve the matching `claim_row_id` and
   inspect `effective_claim_posture`, `support_window_state`,
   `known_limit_refs`, and `exclusion_note_refs`.
6. If a caveat exists, open the cited known-limit or exclusion note
   before deciding whether the downstream wording is truthful.
7. Compare every downstream projection against the canonical owner row.
   A downstream surface may narrow; it may not widen.
8. Classify any mismatch with
   [`drift_blocking_rules.md`](./drift_blocking_rules.md) and route the
   remediation through the required same-change-set group.

## Current workflow bindings

The current seed is intentionally tied into four existing workflows:

- Claim publication uses the claim manifest and parity matrix.
- Help/About/service-health routing uses the destination-descriptor
  contract and worked seeds.
- Signoff review checks the map and drift rules through the shared M0
  validator.
- Shiproom public-proof review cites the map so benchmark and
  certification proof stays attached to the same owner artifacts.
- Stable-facing release review also cites the release-posture ADR and
  promotion-gate map so docs/help, known-limit, advisory, and
  release-note drift can block promotion on the same footing as stale
  build or support evidence.
