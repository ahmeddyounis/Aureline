# Boundary Wording: Hosted/Local/Self-hosted/Commercial Honesty

This document is the contract for the boundary-wording catalog. The catalog is the
single source of truth that governs how Aureline talks about its hosting and
commercial boundary across settings, onboarding, marketplace, help/About, release
notes, and account/upgrade prompts. Those surfaces resolve their boundary facts —
which term to use, what the term actually maps to, the identity/network/data/export/
rollback implications, the local/open alternatives that remain, and the
compatibility/support metadata the claim is anchored to — through this catalog rather
than maintaining parallel, drifting boundary prose.

It is the boundary-honesty projection of the frozen
[content-wording matrix](freeze_the_m5_content_design_controlled_vocabulary_content_ops_and_commercial_boundary_wording_matrix.md).
The controlled hosting/edition vocabulary is owned by the
[deployment-profile register](../../../artifacts/governance/deployment_profiles.yaml)
and the [controlled glossary](../../../artifacts/copy/controlled_glossary.yaml); this
catalog materializes the proof that those identities survive into user-facing wording.

- Record kind: `m5_boundary_wording_catalog`
- Schema: [`schemas/content/m5-boundary-wording.schema.json`](../../../schemas/content/m5-boundary-wording.schema.json)
- Canonical support export: [`artifacts/content/m5-boundary-wording-proof/support_export.json`](../../../artifacts/content/m5-boundary-wording-proof/support_export.json)
- Summary artifact: [`artifacts/content/m5-boundary-wording-proof/m5_boundary_wording.md`](../../../artifacts/content/m5-boundary-wording-proof/m5_boundary_wording.md)
- Fixtures: [`fixtures/content/m5-boundary-wording/`](../../../fixtures/content/m5-boundary-wording/)
- Copy-parity lint: [`scripts/content/m5-copy-parity-lint/`](../../../scripts/content/m5-copy-parity-lint/)
- Producer: `aureline_shell::content::boundary_wording::current_boundary_wording_catalog_export`
- Headless emitter: `aureline_shell_m5_boundary_wording`

## Boundary-wording entries

A `BoundaryWordingEntry` is a typed honesty packet for one boundary claim rendered on
one surface. Each entry carries a stable, locale-neutral `entry_id`, a `concept_id`
(the parity key shared by every surface that renders the same boundary concept), the
controlled `term`, the `surface`, the `claim_kind`, the `actual_boundary_posture` the
claim maps to, the identity/network/data/export/rollback `implications`, the
`alternative_paths` it discloses, and the `source_ref` its wording came from.

The seven `BoundaryTerm`s are the closed vocabulary the lane governs: `hosted`,
`managed`, `premium`, `self_hosted`, `local_only`, `byok`, and `trial`. They map to one
of six `ActualBoundaryPosture`s: `local_independent`, `self_hostable`, `byok`,
`managed_optional`, `managed_required`, and `commercial_paid`.

`render_boundary_explanation` reconstructs a deterministic line — text, term, actual
posture, surface, claim kind, the five implication postures, the disclosed
alternatives, and the support metadata — so any surface can explain the boundary with
one controlled vocabulary.

## No boundary overstatement

Each term claims a level of local independence; each actual posture provides one. A
claim is denied (`boundary_overstates_actual_posture`) when its term claims **more**
local independence than the posture provides — so a surface can never label a managed
or paid capability "Local only" or "Self-hosted". This keeps hosted/open/self-hosted/
commercial language from drifting above the real product boundary.

## No false vendor dependence

When the product contract keeps the core workflow local-capable
(`core_workflow_remains_local`), a managed or paid claim must disclose the local /
BYOK / self-hosted alternative that remains. A claim that hides every local/open
alternative is denied (`implies_vendor_dependence_when_core_local`). Boundary wording
can never pressure users off a valid local or open path, and managed/paid
introductions always keep an export and rollback route
(`managed_or_paid_missing_export_or_rollback`).

## Upgrade and account honesty

An upgrade, account, or help surface that introduces a managed or paid capability must
disclose the local/BYOK/self-hosted alternatives where the product contract says they
exist (`upgrade_surface_missing_alternative_disclosure`). Every available alternative
is anchored to a compatibility/support metadata `reference_ref`, not prose-only
marketing.

## Machine-anchored narrowing and widening

A claim that narrows or widens a boundary references the underlying
compatibility/support metadata through `support_metadata_ref`
(`narrowing_widening_missing_support_metadata`). A `states_boundary` claim may carry
the ref too — and shared concepts do, so parity holds — but a `narrows_boundary` or
`widens_boundary` claim must.

## Cross-surface copy parity

Every surface that renders the same `concept_id` must agree on the boundary term, the
support metadata, the five implication postures, the local-capability posture, and the
disclosed alternative availability. `lint_parity` reports one `ParityFinding` per
drift — `term_drift`, `support_metadata_drift`, `implication_posture_drift`,
`local_capability_posture_drift`, or `alternative_availability_drift` — and validation
fails with `parity_drift` if any are present. The claim act (`claim_kind`) and the
human prose (`canonical_text`) may differ per surface; the **boundary facts** may not.
`shared_concept_ids` must each span at least `SHARED_CONCEPT_MIN_SURFACES` (3) surfaces.

The same rules run as a standalone Python gate
([`scripts/content/m5-copy-parity-lint/check_copy_parity.py`](../../../scripts/content/m5-copy-parity-lint/check_copy_parity.py)),
so release/docs/help/UI review can fail on parity or boundary-honesty drift even when
the underlying feature code still works.

## Locale neutrality

Machine-facing identity stays locale-neutral while human prose localizes around it.
Entry ids, concept ids, support refs, alternative refs, and source refs are lowercase
ascii (`[a-z0-9_.]`); only `canonical_text`, implication `disclosure`, and alternative
`disclosure` prose localize. The localized overlay fixture rewrites every prose field
into a pseudo-localized form while keeping every id, term, posture, and ref
byte-for-byte identical — proving a translation can never fork a concept id or a
support ref into a different boundary claim.

## Validation invariants

`BoundaryWordingCatalog::validate` enforces, among others:

- record kind, schema version, and identity are present;
- the seven closed inventories match the canonical token lists;
- entry ids, concept ids, support refs, and source refs are unique where required and
  locale-neutral;
- every term, surface, claim kind, actual posture, implication dimension, implication
  posture, and alternative path is represented;
- narrowing/widening claims reference support metadata;
- every entry explains all five implication dimensions;
- no claim overstates the actual posture and none implies vendor dependence when the
  core stays local;
- managed/paid introductions keep export and rollback and disclose alternatives on
  upgrade/account/help surfaces;
- each shared concept spans at least three surfaces and no concept drifts across
  surfaces;
- the trust-review and parity-projection invariants all hold;
- the export carries no raw boundary material.

## Acceptance mapping

| Acceptance clause | Resolved by |
|---|---|
| Claimed M5 surfaces explain hosted/local/self-hosted/commercial boundaries with one controlled vocabulary without overstating the actual product boundary. | `BoundaryTerm`/`ActualBoundaryPosture`, `render_boundary_explanation`, and the `boundary_overstates_actual_posture` invariant. |
| Upgrade prompts and account/help surfaces disclose local/BYOK/self-hosted alternatives where the contract says they exist. | `AlternativePathDisclosure`, `must_disclose_alternatives`, and the `upgrade_surface_missing_alternative_disclosure` / `implies_vendor_dependence_when_core_local` invariants. |
| Release/docs/help/UI review can fail when copy parity or boundary honesty drifts even if the feature code still works. | `lint_parity`, the `parity_drift` invariant, and the standalone `check_copy_parity.py` gate. |
| Any surface that narrows or widens a boundary references the underlying compatibility/support metadata. | `support_metadata_ref`, `BoundaryClaimKind::requires_support_metadata`, and the `narrowing_widening_missing_support_metadata` invariant. |

## Fixtures

The fixtures are valid, export-safe catalog packets minted from the same seed builder
as the canonical export by `aureline_shell_m5_boundary_wording`. See
[the fixtures README](../../../fixtures/content/m5-boundary-wording/README.md).
