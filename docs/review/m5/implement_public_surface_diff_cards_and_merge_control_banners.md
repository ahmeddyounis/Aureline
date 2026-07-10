# M5 public-surface diff cards and merge-control banners

Two reusable M5 protected-path governance components — the **public-surface diff card** and the
**merge-control banner** — so a user can tell *which* public surface a change materially affects
(command, CLI flag, schema, manifest, SDK/WIT surface, token, message id, automation contract, or
compatibility claim), *how stable* that surface is, whether the change is breaking, compatible, a
deprecation, or a removal, whether the diff was machine-generated, provider-confirmed, a local
estimate, not evaluated here, or stale relative to base/head, and — for the merge gate — which required
checks and ruleset/branch-protection rules apply, what the current blocker is, what the bypass policy
is, and whether the gate is provider-confirmed rather than a local estimate, before they trust, merge,
or release a governed change.

- Implementation: `crates/aureline-review/src/implement_public_surface_diff_cards_and_merge_control_banners_with_surface_class_stability_label_schema_or_command_delta_disclosure_blocker_reason_bypass_policy_and_migration_note_continuity`
- Boundary schema: [`schemas/ui/m5-public-surface-diff-merge-control-controls.schema.json`](../../../schemas/ui/m5-public-surface-diff-merge-control-controls.schema.json)
- Checked support export: `artifacts/release/m5-public-surface-diff-merge-control-controls-proof/support_export.json`
- Narrowed fixtures: `fixtures/ui/m5-public-surface-diff-merge-control-controls/`

This lane *implements* two of the families frozen by the
[protected-path governance component matrix](freeze_the_m5_protected_path_governance_component_matrix.md).
It reuses that matrix's frozen governance-state vocabulary — `advisory`, `authoritative`, `covered`,
`backup_missing`, `waived`, `expired`, `stale`, `provider_authoritative`, `local_estimate` — verbatim
rather than minting a drifted lexicon. The states this lane owns — `provider_authoritative`,
`local_estimate`, and `stale` — always render under their frozen tokens. Honest states the frozen
lexicon does not name (`machine_generated_local`, `not_evaluated_here`) carry **no** governance token
and never borrow another state's label.

## Derived resolvers

The module derives every honesty axis from an honest input, so a component can never *assert* a posture
it did not earn:

- `resolve_confirmation_locus(source)` maps a confirmation-locus source to the exact locus posture
  (`provider_confirmed`, `machine_generated_local`, `local_estimate`, `not_evaluated_here`, or
  `stale_relative_to_head`) and, where the frozen lexicon names one, to the governance-state token it
  renders under. Both the public-surface diff card and the merge-control banner call it, so their
  local-versus-provider parity is one truth. A `provider_confirmed_gate` or `provider_reported_state`
  source is provider-confirmed; a `machine_generated_locally` source is machine-generated locally; a
  `local_heuristic_estimate` source is a local estimate; a `not_evaluated_here` source was not
  evaluated here; and a `stale_against_base_head` source is stale relative to base/head. A local
  estimate or a machine-generated local diff can never claim provider-confirmed, and a
  not-evaluated-here gate can never claim it was evaluated. **This is the AC pinning merge-control
  blocker honesty: a merge gate never widens from a local estimate to provider mergeability without
  provider confirmation.**
- `resolve_surface_change(source)` maps a surface-change source to the exact change posture
  (`breaking`, `compatible`, `deprecation`, or `removal`). A breaking or removing change requires an
  explicit migration note and evidence link on a stable surface; it can never collapse into generic
  `changed` language.

## Components

- **Public-surface diff card** — names its affected public surfaces (command, CLI flag, schema,
  manifest, SDK/WIT surface, token, message id, automation contract, or compatibility claim), its
  stability label, its schema-or-command delta disclosure, its breaking/compatible/deprecation/removal
  change, its machine-generated-versus-provider confirmation parity, its diff evidence, and its
  migration note where relevant. `OpenDiffEvidence`, `InspectSurfaceChange`, and `ReviewMigrationNote`
  are always offered, so the machine-generated diff, the surface change, and the migration note stay
  inspectable before a user trusts a public-surface change.
- **Merge-control banner** — names its current blocker, required checks, ruleset/branch-protection
  state, bypass policy, local-versus-provider mergeability parity, and export-packet parity.
  `InspectMergeGate`, `ReviewRequiredChecks`, and `ReviewBypassPolicy` are always offered, so the
  current gate, the required checks, and the bypass policy stay inspectable before a user trusts the
  merge.

## Acceptance criteria

- **A stable-contract change cannot hide inside ordinary review without an explicit public-surface diff
  card and migration/evidence links.** The diff cards alone cover all nine public-surface classes and
  all four change postures, so any materially affected public surface renders a card. A stable surface
  with a breaking, removing, or deprecating change that omits its migration note or migration-evidence
  reference fails validation.
- **Merge-control blockers name the current gate honestly and do not widen from local estimate to
  provider mergeability without confirmation.** A blocking banner that omits its blocker reason fails
  validation, and a local-estimate or machine-generated banner that claims provider-confirmed
  mergeability, or a not-evaluated-here banner that claims it was evaluated, fails validation.

## Coverage and invariants

The controls packet's validator enforces the honesty invariants directly:

- The union of both vectors covers every confirmation-locus source and posture; the diff cards cover
  every surface-change source and posture, every public-surface class, every stability class, and every
  diff-evidence kind; the merge-control banners cover every merge-blocker class, every bypass-policy
  class, and every protection state.
- Every component's four hard invariants (`hides_surface_class_or_stability` /
  `hides_blocker_reason_or_bypass_policy`, `lets_local_estimate_read_as_provider_confirmed` /
  `lets_local_estimate_read_as_provider_mergeable`,
  `lets_stable_breaking_change_hide_without_migration` /
  `names_generic_blocker_instead_of_current_gate`, and `invents_alternate_state_label`) must be
  `false`.
- Raw change generators, raw provider payloads, raw diff bodies, raw ruleset definitions, credentials,
  and secrets stay outside the export boundary; every surface, evidence, and gate reference is an opaque
  export-safe reference.

## Regenerating artifacts

The checked support export, Markdown summary, and narrowed fixtures are emitted by the gated generator
test:

```
GEN_PUBLIC_SURFACE_DIFF_MERGE_CONTROL_CONTROLS_ARTIFACTS=1 cargo test -p aureline-review \
  implement_public_surface_diff_cards_and_merge_control_banners_with_surface_class_stability_label_schema_or_command_delta_disclosure_blocker_reason_bypass_policy_and_migration_note_continuity::tests::generate_artifacts \
  -- --exact --ignored
```
