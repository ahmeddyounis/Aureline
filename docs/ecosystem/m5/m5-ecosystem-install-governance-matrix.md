# M5 ecosystem install-governance matrix

This document describes the canonical packet that freezes the **M5 ecosystem
install-governance matrix** — one row per marketed M5 artifact family — and that
automatically narrows or fails promotion on any underqualified family before
publication. It is the user-facing companion to the governed artifact at
`artifacts/ecosystem/m5/m5-ecosystem-install-governance-matrix.json` and the typed
model in the `aureline-ecosystem` crate
(`freeze_the_m5_ecosystem_install_lifecycle_state_and_activation_budget_matrix`).

Where the
[`compatibility scorecard`](../m4/stabilize-ecosystem-compatibility-scorecards-reference-workspace.md)
speaks for individual imported extensions and bridges, this packet is the canonical
truth for the new M5 artifact *families* — framework packs, docs packs, local-model
packs, recipe packs, templates, and bridge-backed or side-loaded packages — so they
are never treated as undifferentiated catalog rows.

## What this packet covers

The packet carries one row for every claimed M5 artifact family:

1. **`first_party_framework_pack`** — first-party framework pack.
2. **`docs_pack`** — documentation pack.
3. **`local_model_pack`** — local-model pack.
4. **`signed_recipe_pack`** — signed recipe pack.
5. **`template_artifact`** — template artifact.
6. **`bridge_backed_package`** — bridge-backed package.
7. **`side_loaded_package`** — side-loaded package.
8. **`mirrored_registry_variant`** — mirrored/private-registry variant.

Each row answers, for its family:

- **Where did it come from?** A `runtime_origin` of `first_party_signed`,
  `partner_signed`, `community_signed`, `bridge_runtime`, `local_model_runtime`, or
  `unsigned_side_loaded`, plus a `declared_support_class` the family claims for
  itself.
- **Is it compatible with the target?** A `compatibility_label` of `compatible`,
  `degraded_compatible`, `compatibility_bridge_required`, or `unsupported_on_target`.
- **What does it do to permissions?** A `permission_manifest_state` of `unchanged`,
  `reduced`, `additive_disclosed`, `expanded_unreviewed`, or `not_applicable`.
- **What does it cost to activate?** An `activation_budget_band` of
  `healthy_under_budget`, `approaching_ceiling`, `over_budget`, `budget_unknown`, or
  `not_applicable`.
- **Where is it in its lifecycle?** A `lifecycle_state` of `available`, `installed`,
  `update_available`, `disabled`, `quarantined`, `rolled_back`, or `retired`.
- **How fresh is the evidence?** An `evidence_freshness` of `current`, `stale`,
  `expired`, or `unknown`.
- **Can the install be undone?** A `rollback_posture` of `reversible_verified`,
  `reversible_unverified`, `compensating_only`, `irreversible`, or `not_applicable`.
- **What is backing it?** A `provenance_ref`, a `permission_manifest_ref`, an
  `activation_budget_ref`, a `compatibility_ref` (downgrade story), a `rollback_ref`,
  and a `support_export_ref` that binds the row into the registry, Help/About,
  support exports, and release surfaces.
- **What does the gate publish?** A `published_support_class`, a
  `promotion_decision`, and the headline `narrowing_reasons` that explain it.

## The governance gate narrows automatically

The support class a family may publish is **not** copied from
`declared_support_class`. It is recomputed and the `published_support_class`,
`promotion_decision`, and `narrowing_reasons` fields must equal that recomputation or
validation fails. The gate lowers the published support class to the weakest of:

- the **capability floor** — the family's own `declared_support_class`;
- the **runtime-origin ceiling** — `first_party_signed`/`partner_signed` permit
  `fully_supported`, `bridge_runtime`/`local_model_runtime` cap at
  `best_effort_supported`, and `community_signed`/`unsigned_side_loaded` cap at
  `community_supported`, so a side-loaded or community artifact can never inherit
  first-party trust;
- the **freshness ceiling** — `current` permits `fully_supported`, `stale`/`unknown`
  cap at `best_effort_supported`, and `expired` caps at `community_supported`;
- the **permission ceiling** — `unchanged`/`reduced`/`not_applicable` permit
  `fully_supported`, `additive_disclosed` caps at `best_effort_supported`, and
  `expanded_unreviewed` caps at `community_supported`;
- the **activation ceiling** — `healthy_under_budget`/`not_applicable` permit
  `fully_supported`, `approaching_ceiling`/`budget_unknown` cap at
  `best_effort_supported`, and `over_budget` caps at `community_supported`;
- the **compatibility ceiling** — `compatible` permits `fully_supported`,
  `degraded_compatible`/`compatibility_bridge_required` cap at
  `best_effort_supported`, and `unsupported_on_target` caps at `unsupported`;
- the **rollback ceiling** — `reversible_verified`/`not_applicable` permit
  `fully_supported`, `reversible_unverified`/`compensating_only` cap at
  `best_effort_supported`, and `irreversible` caps at `community_supported`;
- the **lifecycle ceiling** — `available`/`installed`/`update_available`/`disabled`
  permit `fully_supported`, `retired` caps at `community_supported`, and
  `quarantined`/`rolled_back` cap at `unsupported`.

The `promotion_decision` then names the result: `promote` for a published
`fully_supported`, `narrow_to_best_effort`, `narrow_to_community`, or
`fail_promotion` for a withheld `unsupported` claim.

The `narrowing_reasons` are the seven canonical, spec-aligned release-control
triggers, each recomputed from the observed states:

- **`provenance_unverified`** — the runtime origin is `unsigned_side_loaded`.
- **`evidence_stale`** — freshness is `stale` or `expired`.
- **`permission_expansion_unreviewed`** — the permission manifest is
  `expanded_unreviewed`.
- **`activation_budget_exceeded`** — the activation band is `over_budget`.
- **`compatibility_unsupported`** — the compatibility label is
  `unsupported_on_target`.
- **`rollback_incomplete`** — the rollback posture is `reversible_unverified` or
  `irreversible`.
- **`quarantined`** — the lifecycle state is `quarantined` or `rolled_back`.

This is what lets release/registry/public-truth tooling **prove** that stale or
underqualified families narrow before publication: a family that is unverified,
stale, permission-expanded, over budget, unsupported, rollback-incomplete, or
quarantined simply cannot carry a `fully_supported` published claim, because the
recomputed gate decision overrides the stored one.

## Governance stays family-specific and provenance-bound

A signed first-party framework pack must never lend its trust to a side-loaded
package or a mirrored-registry variant. The packet enforces this several ways:

- Every claimed family must carry exactly one row (`MissingFamilyRow` /
  `DuplicateFamilyRow` otherwise), so no family inherits trust from an adjacent one,
  and a row may not cover a family outside the claimed set (`UnclaimedFamilyRow`).
- Every row must carry its own non-empty `provenance_ref`,
  `permission_manifest_ref`, `activation_budget_ref`, `compatibility_ref`,
  `rollback_ref`, and `support_export_ref`.

A promotable family — one that publishes `fully_supported` — must additionally be
genuinely clean: a fully-supporting runtime origin, current freshness, a
non-narrowing permission, activation, compatibility, rollback, and lifecycle state,
an all-supported capability floor, and no narrowing reason
(`PromotedFamilyNotClean` otherwise).

## How downstream surfaces consume it

`export_projection()` produces a redaction-safe row set with each family's runtime
origin, declared and published support class, compatibility, permission, activation,
lifecycle, freshness, and rollback states, decision, and narrowing-reason tokens,
plus `promotable_count`, `narrowed_count`, and `failed_promotion_count`. The
marketplace and registry, Help/About, `docs/migration`, support exports, and
release/public-truth surfaces should ingest this projection directly rather than
restating M5 ecosystem install/lifecycle/activation status by hand, so public and
internal claim surfaces use the same source-class, provenance, permission,
activation, downgrade, and rollback vocabulary as the underlying packet.

## Validation

`M5EcosystemGovernanceMatrix::validate()` reports every violation, including an
unsupported schema version or record kind, non-canonical closed vocabularies, empty
required fields, duplicate family ids, duplicate or missing family rows,
unclaimed-family rows, duplicate narrowing reasons, an overstated published support
class, a decision that disagrees with the gate, narrowing reasons that disagree with
the recomputed set, a promotable family that is not clean, and a summary block that
disagrees with the rows.
