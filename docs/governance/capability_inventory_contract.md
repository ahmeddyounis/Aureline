# Capability-inventory contract

This document publishes Aureline's capability-inventory registry contract.
It exists so capability state is first-class across product UI, docs/help,
CLI/headless inspection, support exports, compatibility reports, and release
artifacts, and so no public-facing surface can claim a capability without an
explicit registry row stating its lifecycle posture, ownership, gating, and
visibility policy.

Machine-readable companions:

- [`/artifacts/governance/capability_inventory_seed.yaml`](../../artifacts/governance/capability_inventory_seed.yaml)
  — seeded capability-inventory entries spanning claimable, preview, sunset,
  policy-disabled, managed-only, and internal-only examples.
- [`/schemas/governance/capability_inventory_entry.schema.json`](../../schemas/governance/capability_inventory_entry.schema.json)
  — boundary schema for one capability inventory entry.

Related upstream contracts (re-exported, not re-minted):

- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  and
  [`/schemas/governance/capability_lifecycle.schema.json`](../../schemas/governance/capability_lifecycle.schema.json)
  — canonical lifecycle axes and dependency-marker vocabulary.
- [`/docs/governance/capability_axis_matrix.md`](./capability_axis_matrix.md)
  and
  [`/docs/governance/capability_lifecycle_projection_matrix.md`](./capability_lifecycle_projection_matrix.md)
  — cross-surface projection rules and forbidden badge collapses.
- [`/docs/governance/claim_manifest_contract.md`](./claim_manifest_contract.md),
  [`/artifacts/governance/public_truth_parity_matrix.yaml`](../../artifacts/governance/public_truth_parity_matrix.yaml),
  and
  [`/artifacts/governance/claim_propagation_rules.yaml`](../../artifacts/governance/claim_propagation_rules.yaml)
  — public-claim channels, downgrade propagation, and fail-closed claim routing.
- [`/artifacts/governance/experiments_register.yaml`](../../artifacts/governance/experiments_register.yaml)
  and
  [`/docs/governance/feature_flag_policy.md`](./feature_flag_policy.md)
  — canonical rollout/flag rows and disclosure expectations.

## Purpose

The capability lifecycle contract makes effective posture renderable, but it
does not by itself guarantee that every capability a surface can “claim”
exists in one place with an owner, a public label policy, a rollout gate, and
export visibility.

The capability inventory fills that gap:

- It is the **registry of record** for what counts as a named capability.
- It declares whether a capability is **claimable**, **visible-but-nonclaimable**,
  or **forbidden** on public-claim surfaces.
- It declares which lanes are allowed to author public statements about the
  capability (docs/help, release, support).
- It declares whether the capability may appear in **support exports** and
  **compatibility artifacts**.
- It binds capabilities that are gated (feature flag, experiment, rollout, or
  manual gate) to an explicit gate ref so surfaces cannot “hide behind” an
  implicit rollout state.

In short: if a capability is not in the inventory, it is not eligible for
public claim projection, and every surface fails closed rather than inventing
a private label set.

## Terms

- **Capability (inventory sense).** A named unit a surface can list, badge, or
  claim (setting, bundle, command, provider-linked feature, workspace feature,
  extension feature, SDK/API surface, docs capability, support capability).
- **Surface family.** A protected destination family the capability may be
  projected onto (settings row, bundle card, command palette entry, install
  review step, docs page, Help/About, diagnostics panel, support export row,
  policy explainer row, CLI help, compatibility report, release packet, release
  notes).
- **Claim lane.** A lane id (from `ownership_matrix.yaml`) allowed to author
  public statements about the capability. Claim lanes are *who may claim*; the
  parity matrix describes *where the claim propagates*.
- **Rollout gate.** The controlling flag/experiment/rollout/manual gate that
  must be disclosed when capability availability is not universal.

## Entry model (what every row must state)

Every capability inventory entry states, at minimum:

- `capability_id` and `capability_kind`
- `surface_families` the capability may project onto
- `lifecycle_state` (declared posture, using the canonical vocabulary)
- `owner_dri` and `owning_lane`
- `claim_lanes` allowed to author public statements about the capability
- `dependency_marker_refs` that must remain first-class disclosures when
  present in lifecycle projections
- `rollout_gate` (or null) plus whether public disclosure is required
- `public_label` (or null) plus `public_label_policy`
- `public_claim_posture` (claimable / visible-but-nonclaimable / forbidden)
- `export_visibility` (public exportable / support-only / internal redacted)

The inventory does not compute effective posture. Surfaces still render their
badges from the capability lifecycle contract (declared vs effective state plus
live dependency markers). The inventory provides the cross-surface alignment
inputs that lifecycle alone does not carry (public-label eligibility, claim
posture, export visibility, and gate disclosure requirements).

## Propagation rules (normative)

### 1) Registration is required (fail closed)

1. A protected surface MUST NOT list, badge, or claim a capability unless it
   can resolve that capability to exactly one capability-inventory entry.
2. When an inventory entry is missing, the surface MUST fail closed:
   - treat the capability as nonclaimable,
   - avoid “stable-by-omission” wording,
   - and render an explicit inventory-gap cue (not a generic “unavailable”).

### 2) Inventory never widens lifecycle posture

Inventory is metadata, not an override:

- A surface MUST use the capability lifecycle contract to render effective
  posture and markers.
- A surface MUST NOT use inventory fields to widen above the lifecycle row’s
  effective posture (for example, claiming stable when a live marker narrows
  to preview).

### 3) Gate disclosure must be consistent

When `rollout_gate.public_disclosure_required = true`:

- Any surface that lists the capability MUST disclose that the capability is
  gated (in copy-safe terms) and MUST route to a repair hook or explainer.
- Release notes and docs MUST NOT present the capability as universally
  available without disclosing the gate.

### 4) Public-label policy prevents leakage

1. `public_label_policy = public_label_forbidden` means:
   - `public_label` is null,
   - public claim surfaces MUST NOT name the capability, and
   - any accidental appearance is treated as a disclosure violation.
2. `public_claim_posture = forbidden` is the hard stop:
   - public claim surfaces MUST NOT project it, and
   - export artifacts MUST NOT include it (`export_visibility` must be
     internal redacted).

### 5) Export visibility controls what appears in bundles/reports

- `export_visibility = public_exportable` may appear in compatibility reports
  and public-proof packets when the lifecycle posture and evidence are current.
- `export_visibility = support_export_only` may appear in support bundles and
  support-facing exports, but MUST NOT appear in public-proof or compatibility
  publication artifacts.
- `export_visibility = internal_redacted` MUST NOT appear in exports.

### 6) Alignment with public-claim projection

Capability inventory entries are a join input to claim-bearing work:

- A claim-bearing channel that names a capability MUST ensure the capability is
  `claimable` and has a public label policy that permits naming.
- When a capability transitions to `deprecated` or `retired`, the inventory
  update MUST be accompanied by claim narrowing on the channels that mention it
  (docs/help, release notes, and support templates). Silent marketing/support
  wording drift is non-conforming.

## Change triggers (when updates require review)

The following inventory changes are public-proof sensitive and require a
reviewable update in the same change set for every downstream surface family
that can project the capability:

- `public_claim_posture` changes (especially to/from `forbidden`)
- `export_visibility` changes
- `public_label_policy` changes
- adding/removing a `rollout_gate` or flipping `public_disclosure_required`
- lifecycle promotions/demotions that affect claim language (`labs`/`preview`/
  `beta`/`stable`/`deprecated`/`retired`)
- adding a new `dependency_marker_ref` that will narrow or gate a stable-facing
  capability projection

When a change affects a claim-bearing surface, the change must also update the
surface’s projection rules or copy so the public truth cannot remain greener
than the worst supporting lifecycle/evidence truth.

## Seed scope (today)

The seed file intentionally includes:

- launch-critical claimable capabilities (settings, bundles, workspace features),
- at least one policy-disabled/deprecated capability,
- at least one provider-linked capability with multiple dependency markers,
- at least one managed-only capability whose posture must remain explicit, and
- internal-only capabilities that are forbidden to leak into public claims.

This is enough to make downgrade and hidden-surface behavior explicit and to
give downstream tooling one registry to depend on as additional capabilities
are registered.
