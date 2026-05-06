# Workflow-bundle lifecycle visibility, successor guidance, and certification-sheet contract

This document freezes the cross-surface contract that keeps
workflow-bundle lifecycle truth visible after install, update,
drift, and evidence aging. The goal is that strong bundle claims
stay honest beyond the happy path: deprecation, successor
recommendations, certification expiry, mirror/offline changes,
removed artifact families, and remove/rollback paths remain
inspectable on every entry, update, and support surface.

The companion machine-readable artifacts live at:

- [`/schemas/workflow/bundle_certification_sheet.schema.json`](../../schemas/workflow/bundle_certification_sheet.schema.json)
  — boundary schema for the certification-sheet artifact exported
  alongside strong bundle claims.
- [`/artifacts/workflow/bundle_successor_rows.yaml`](../../artifacts/workflow/bundle_successor_rows.yaml)
  — seeded successor recommendation rows that MAY be published in a
  channel train alongside bundle sets.

The companion fixtures live under:

- [`/fixtures/workflow/bundle_lifecycle_cases/`](../../fixtures/workflow/bundle_lifecycle_cases/)

This contract is normative for:

- how lifecycle, successor, evidence age, and retest-needed posture
  stay visible across Start Center, gallery/update, bundle detail,
  export/support, and CLI/headless surfaces;
- the minimum certification-sheet fields required to substantiate
  strong claims without collapsing into decorative badge copy; and
- the rule that successor/sunset guidance MUST NOT hide drift history,
  rollback clarity, local overrides, or owner attribution.

Where it disagrees with the PRD, TAD, TDD, UI/UX spec, design-system
style guide, or milestone-document anchors quoted by upstream
contracts, those sources win and this contract plus its companion
artifacts update in the same change.

This contract mints **no** new bundle, lifecycle, drift, badge, or
support vocabulary. It re-exports — by reference, never by
redefinition — the closed sets frozen by the upstream contracts
listed in §3.

## Who reads this contract

- **Start Center, gallery/update, and bundle-detail surface authors**
  who must keep lifecycle and successor guidance visible without
  collapsing it into release-note-only copy.
- **Support-export and claim-publication authors** who must preserve
  lifecycle truth, owner attribution, and drift context in exported
  packets and published claim lanes.
- **Bundle authors and reviewers** who need one checklist for what a
  certification sheet MUST contain and what a successor recommendation
  MUST (and MUST NOT) imply.

## 1. Scope

In scope:

- A certification-sheet artifact boundary that carries the minimum
  fields required to substantiate strong claims.
- Successor recommendation visibility rules across entry, update, and
  support surfaces.
- Lifecycle visibility rules for:
  - deprecation and successor recommendation;
  - mirror/offline posture changes and promotion events;
  - certification expiry and retest-needed posture;
  - removed artifact families (missing artifacts) and drift-projected
    claim narrowing; and
  - remove/rollback affordance projection without erasing drift
    history or user overrides.

Out of scope:

- Recommendation ranking or “best bundle for you” ordering.
- Installer/update engine implementation.
- Registry transport, signature verification, and mirror promotion
  admission logic (owned by ecosystem/release contracts).

## 2. Non-negotiable invariants

1. **Lifecycle is never silent.** Every bundle projection MUST render
   lifecycle and successor posture even when the state is “healthy”:
   the conforming “healthy” representation is
   `lifecycle_state_class = active_supported` plus an explicit
   `successor_recommendation_class = no_successor`.
2. **Successor guidance never rewrites history.** A successor
   recommendation MUST NOT delete or hide:
   - drift rows;
   - local overrides;
   - applied-change lineage; or
   - rollback/remove review context.
3. **Successor guidance never widens claim authority.** A successor
   suggestion MUST NOT cause the current row to inherit stronger claim
   language (for example, a community successor does not permit
   certified wording on the predecessor).
4. **Certification never outlives proof.** Evidence freshness and
   badge downgrade rules are owned by the evidence-freshness contract;
   every surface MUST project the resulting `evidence_age_class`,
   `retest_needed_posture`, and any downgrade overlays rather than
   hiding the expiry.
5. **Local overrides can void certification.** When drift evaluation
   yields `claim_narrowing_class = breaks_certification_target_recall`
   or `narrows_certification_target_pending_retest`, surfaces MUST keep
   the drift cause visible alongside any certification-sheet or badge
   projection.

## 3. Upstream contracts (re-exported by reference)

This contract composes with:

- [`/docs/workflow/workflow_bundle_object_model.md`](./workflow_bundle_object_model.md)
  and [`/schemas/workflow/bundle_manifest.schema.json`](../../schemas/workflow/bundle_manifest.schema.json)
  for `lifecycle_state_class`, `successor_recommendation_class`,
  `evidence_age_class`, `retest_needed_posture`, `support_class`, and
  the `certification_sheet_refs[]` linkage.
- [`/docs/ux/start_center_bundle_surfaces.md`](../ux/start_center_bundle_surfaces.md)
  and the bundle card/detail schemas for entry, gallery/update, export
  summary, and detail-page projection rules.
- [`/docs/workflow/bundle_drift_and_removal_contract.md`](./bundle_drift_and_removal_contract.md)
  and [`/schemas/workflow/bundle_drift_row.schema.json`](../../schemas/workflow/bundle_drift_row.schema.json)
  for drift visibility, removed artifact families, and claim narrowing
  semantics.
- [`/docs/workflow/bundle_evidence_freshness_contract.md`](./bundle_evidence_freshness_contract.md)
  and [`/artifacts/workflow/bundle_badge_freshness_rows.yaml`](../../artifacts/workflow/bundle_badge_freshness_rows.yaml)
  for evidence-age evaluation, badge downgrade, and publish-gate rules.

## 4. Certification sheet artifact (minimum fields)

The certification sheet is the reviewer-facing, exportable artifact
that substantiates a strong claim (“Certified”, benchmark-backed
compatibility, replacement-grade workflow coverage) by binding the
bundle identity to its evidence, platform/toolchain matrix, and owner.

The sheet record shape is frozen by:

- [`/schemas/workflow/bundle_certification_sheet.schema.json`](../../schemas/workflow/bundle_certification_sheet.schema.json)

### 4.1 Required identity and attribution

Every sheet MUST carry:

- `certification_sheet_id`
- `bundle_id`, `bundle_revision`
- `bundle_class`, `bundle_source_class`, `bundle_status_class`
- `support_class`
- `owner_dri` (resolved by downstream tooling through
  [`artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml))

### 4.2 Required certification target

Every sheet MUST cite at least one of:

- an `archetype_ref` (archetype row id + revision), and/or
- a `reference_workspace_id`

so “what was certified” is explicit and auditable without reading
free-text copy.

### 4.3 Required freshness / lifecycle posture

Every sheet MUST carry:

- `evidence_age_class`
- `retest_needed_posture`
- `lifecycle_state_class`
- the `successor_recommendation` block (including `no_successor`)

so certification expiry and successor/sunset guidance remain visible on
the sheet itself.

### 4.4 Required platform/toolchain matrix and proof bindings

Every sheet MUST carry:

- a `platform_toolchain_matrix[]` describing the supported
  platform/toolchain envelope the certification asserts; and
- `evidence_link_refs[]` binding the sheet to the underlying proof
  packets (scoreboard, benchmark, compatibility report, docs version
  match, etc.) by ref, never by raw payload.

## 5. Successor recommendation rows (seeded)

The workflow-bundle manifest’s lifecycle block is the primary source of
truth for successor recommendations. A channel train MAY additionally
publish successor recommendation rows to support cross-cutting surfaces
that need a stable, reviewable mapping table.

The seeded successor mapping lives at:

- [`/artifacts/workflow/bundle_successor_rows.yaml`](../../artifacts/workflow/bundle_successor_rows.yaml)

Rules (frozen):

1. Every successor row MUST carry a `decision_row_ref` so successor
   guidance is auditable and cannot silently widen claim authority.
2. A successor row MUST NOT contradict a manifest’s explicit successor
   block; it may only:
   - restate the manifest successor mapping for tooling convenience, or
   - supply a mapping only when the manifest is unavailable or pinned to
     an older revision that lacks the successor block (offline restore
     or expired registry path).
3. Successor rows are guidance, not mutation: they MUST NOT imply that
   an installed bundle has updated, and MUST NOT hide drift history.

## 6. Lifecycle visibility rules (frozen)

This section pins the minimum **visibility** obligations for lifecycle
events. Detailed record shapes and downgrade/resolve behavior are owned
by the upstream contracts; this section is the “must still be visible”
checklist.

### 6.1 Deprecation and successor recommendation

When a bundle is deprecated, archived, or demoted, every surface that
renders the bundle MUST keep the deprecation posture and successor
guidance visible.

Rules (frozen):

1. Deprecation is visible when either:
   - `bundle_status_class = deprecated_or_archived`, or
   - `lifecycle_state_class ∈ {soft_deprecated_with_successor,
     hard_deprecated_remove_recommended, removed_unavailable,
     rollback_recommended}`.
2. When `successor_recommendation_class ∈ {successor_recommended_*}`,
   Start Center cards, detail pages, gallery/update rows, export-facing
   summaries, and CLI/headless listings MUST render a typed successor
   affordance (for example “View successor”) that points at the
   successor bundle id + revision — not free-text copy.
3. Successor guidance MUST preserve owner attribution and MUST NOT
   widen the predecessor’s claim authority. A successor mapping never
   implies the predecessor is certified/current.

### 6.2 Mirror/offline posture changes and mirror promotion

Mirror/offline posture is part of lifecycle truth: a bundle that is now
mirror-only or offline-only MUST remain visibly different from a
live-origin row.

Rules (frozen):

1. Every projection MUST render the manifest’s
   `mirror_or_offline_packaging_posture` chip/state (or an explicit
   unknown posture when offline).
2. A mirror promotion or packaging posture change MUST NOT be expressed
   only in release notes. If the effective posture changes across
   installs/updates, the next bundle projection MUST surface the new
   posture and keep the before/after discoverable via drift/audit
   surfaces (per the drift/removal contract provenance rules).

### 6.3 Certification expiry, evidence age, and retest-needed posture

Certification expiry is a lifecycle event. Surfaces MUST prefer visible
truth over “clean” presentation.

Rules (frozen):

1. When `evidence_age_class ∈ {aging_within_window, stale_past_window,
   age_unknown}`, Start Center cards, detail pages, gallery/update rows,
   export summaries, and support exports MUST render the evidence-age
   state (chip/overlay) and MUST surface `retest_needed_posture` when it
   is not `not_required`.
2. When evidence is stale, the certification sheet MAY still be
   referenced and exportable, but it MUST carry the stale posture and
   MUST NOT present certified wording as if current.
3. Claim-publication gates MAY block publishing certified wording (per
   the evidence-freshness contract), but user-scope and support surfaces
   MUST still show the lifecycle state, the stale reason class, and the
   owner attribution.

### 6.4 Removed artifact families and drift-projected narrowing

Removed or missing bundle-owned artifacts MUST remain visible as drift;
they are lifecycle truth, not a hidden implementation detail.

Rules (frozen):

1. A missing extension, docs pack, preset, recipe, template reference,
   or evidence link MAY surface as one or more drift rows with
   `drift_state_class = missing_artifact`.
2. When drift evaluation yields
   `claim_narrowing_class ∈ {narrows_certification_target_pending_retest,
   breaks_certification_target_recall}`, entry and support surfaces MUST
   keep the narrowing visible and MUST NOT present a clean Certified
   state.

### 6.5 Remove/rollback paths and drift/override preservation

Remove and rollback affordances are lifecycle events and MUST remain
auditable.

Rules (frozen):

1. When the manifest’s lifecycle block cites `removal_surface_kind` or
   `rollback_surface_kind`, surfaces that render lifecycle MUST keep
   those typed affordances discoverable (even when disabled by offline
   or policy constraints, using typed disabled reasons).
2. Removal/rollback guidance MUST NOT imply deletion of user-owned
   artifacts. Drift/removal review records preserve:
   - bundle-owned removable assets, and
   - retained local overrides (kept, inlined, or dropped with consent).
3. Successor guidance MUST NOT suppress local overrides or drift rows;
   the conforming experience is “successor suggested” **plus** “this
   workspace has overrides/drift” with both remaining inspectable.

## 7. Fixture cases (required coverage)

The fixture corpus under
[`/fixtures/workflow/bundle_lifecycle_cases/`](../../fixtures/workflow/bundle_lifecycle_cases/)
must cover:

- stale certification / retest-needed visibility;
- successor bundle available (deprecated with successor);
- mirror-only or offline packaging posture with visible lifecycle;
- removed artifact family (missing extension) surfacing as drift;
- narrowed support class remaining visible; and
- local overrides that void certification without erasing history.
