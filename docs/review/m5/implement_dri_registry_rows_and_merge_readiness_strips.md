# M5 DRI-registry rows and merge-readiness strips

Two reusable M5 protected-path governance components — the **DRI-registry row** and the
**merge-readiness strip** — so a user can tell *who* is accountable for a governed service or path (the
primary and backup DRI role aliases, the escalation alias, the support forum, and the benchmark or
compatibility owner where relevant), *how the owner was resolved* (a CODEOWNERS or provider team
assignment, a manifest or registry declaration, or only an advisory heuristic guessed from the last
interacting team), *how fresh* that registry entry is, and — for the merge gate — whether a change is a
local estimate or provider-authoritative, which queue or branch it targets, how many blockers remain,
what the required next action is, and how to export the readiness packet, before they hand off,
escalate, or merge a governed change.

- Implementation: `crates/aureline-review/src/implement_dri_registry_rows_and_merge_readiness_strips_with_primary_backup_role_aliases_support_or_escalation_path_queue_or_branch_target_truth_blocker_counts_export_packet_actions_and_no_silent_mergeability_widening`
- Boundary schema: [`schemas/ui/m5-dri-registry-merge-readiness-controls.schema.json`](../../../schemas/ui/m5-dri-registry-merge-readiness-controls.schema.json)
- Checked support export: `artifacts/release/m5-dri-registry-merge-readiness-controls-proof/support_export.json`
- Narrowed fixtures: `fixtures/ui/m5-dri-registry-merge-readiness-controls/`

This lane *implements* two of the families frozen by the
[protected-path governance component matrix](freeze_the_m5_protected_path_governance_component_matrix.md).
It reuses that matrix's frozen governance-state vocabulary — `advisory`, `authoritative`, `covered`,
`backup_missing`, `waived`, `expired`, `stale`, `provider_authoritative`, `local_estimate` — verbatim
rather than minting a drifted lexicon. The states this lane owns — `provider_authoritative`,
`local_estimate`, and `stale` — always render under their frozen tokens. Honest states the frozen
lexicon does not name (`ci_only`, `not_evaluated_here`) carry **no** governance token and never borrow
another state's label.

## Derived resolvers

The module derives every honesty axis from an honest input, so a component can never *assert* a posture
it did not earn:

- `resolve_authority_locus(source)` maps an authority-locus source to the exact locus posture
  (`provider_authoritative`, `local_estimate`, `ci_only`, `not_evaluated_here`, or
  `stale_relative_to_head`) and, where the frozen lexicon names one, to the governance-state token it
  renders under. Both the DRI-registry row and the merge-readiness strip call it, so their
  local-versus-provider parity is one truth. A `provider_authoritative_state` or
  `provider_reported_state` source is provider-authoritative; a `local_heuristic_estimate` source is a
  local estimate; a `ci_reported_only` source is CI-only; a `not_evaluated_here` source was not
  evaluated here; and a `stale_against_base_head` source is stale relative to base/head. A local
  estimate or a CI-only signal can never claim provider-authoritative, and a not-evaluated-here gate can
  never claim it was evaluated. **This is the AC pinning merge-readiness honesty: a change never widens
  from a local estimate to provider mergeability without provider confirmation.**
- `resolve_owner_source(signal)` maps an owner-source signal to the exact owner-source posture
  (`codeowners_authoritative`, `registry_declared`, `advisory_heuristic`, or `unresolved`). Only a
  CODEOWNERS rule or a provider team assignment is authoritative; an owner guessed from the last
  interacting team is an advisory heuristic and can never read as an authoritative owner, so owner and
  escalation truth stay aligned wherever a governed change is listed.

## Components

- **DRI-registry row** — names its service/path identity, primary and backup DRI role aliases,
  escalation alias, support forum, benchmark or compatibility owner where relevant, escalation-path
  continuity, owner source, and registry freshness. `OpenSupportForum`, `InspectOwnerSource`, and
  `ReviewEscalationPath` are always offered, so the support forum, the owner source, and the escalation
  path stay inspectable before a user hands off a governed change.
- **Merge-readiness strip** — names its local-estimate-versus-provider-authoritative state, queue/branch
  target, blocker count, required next action, export-packet action, and mergeability parity.
  `OpenBlockerList`, `InspectMergeTarget`, and `ExportReadinessPacket` are always offered, so the
  blocker list, the merge target, and the export packet stay reachable before a user trusts the merge.

## Acceptance criteria

- **Review/release surfaces keep owner and escalation truth aligned without guessing from the last
  interacting team.** The DRI rows alone cover all six owner-source signals and all four owner-source
  postures, so an advisory owner is always separable from an authoritative one. An advisory-heuristic
  row that claims an authoritative owner, or that omits its advisory note, fails validation, and any
  role alias carrying a personal contact detail (an `@` handle) fails validation.
- **A change never appears `mergeable here` when it is only locally reviewable or blocked by
  provider-authoritative controls.** A strip may claim `mergeable_here` only when its locus is
  provider-authoritative *and* no blocker remains; a local-estimate, CI-only, stale, or blocked strip
  that claims mergeable-here fails validation, as does a not-evaluated-here strip that claims it was
  evaluated.

## Coverage and invariants

The controls packet's validator enforces the honesty invariants directly:

- The union of both vectors covers every authority-locus source and posture; the DRI rows cover every
  owner-source signal and posture, every support-forum kind, every escalation-continuity state, and
  every registry-freshness state; the merge-readiness strips cover every merge-target kind and every
  required-next-action kind.
- Every component's four hard invariants (`hides_owner_or_escalation_identity` /
  `hides_target_or_blocker_count`, `lets_advisory_owner_read_as_authoritative` /
  `lets_local_estimate_read_as_provider_mergeable`, `guesses_owner_from_last_interacting_team` /
  `widens_local_estimate_to_provider_mergeability`, and `invents_alternate_state_label`) must be
  `false`.
- Raw CODEOWNERS bodies, raw provider payloads, raw ruleset definitions, personal contact details,
  credentials, and secrets stay outside the export boundary; every owner, escalation, forum, and gate
  reference is carried only as an opaque, export-safe role alias or reference.

## Regenerating artifacts

The checked support export, Markdown summary, and narrowed fixtures are emitted by the gated generator
test:

```
GEN_DRI_REGISTRY_MERGE_READINESS_CONTROLS_ARTIFACTS=1 cargo test -p aureline-review \
  implement_dri_registry_rows_and_merge_readiness_strips_with_primary_backup_role_aliases_support_or_escalation_path_queue_or_branch_target_truth_blocker_counts_export_packet_actions_and_no_silent_mergeability_widening::tests::generate_artifacts \
  -- --exact --ignored
```
