# Feature-flag, experiment, and Labs policy

This document is the **normative** policy for experiments, feature
flags, Labs inventory rows, and rollout controls in this repository.
At the current milestone Aureline does **not** ship a runtime flag
service, so the authoritative control surface is the pair of
machine-readable registers:

- [`/artifacts/governance/experiments_register.yaml`](../../artifacts/governance/experiments_register.yaml)
  — canonical register for every control row, including hidden
  developer toggles and rollout rows.
- [`/artifacts/governance/labs_register.yaml`](../../artifacts/governance/labs_register.yaml)
  — contributor-visible projection for prototype / Labs / preview
  items that should be inspectable without reading hidden-toggle rows.

Companion sources:

- [`/docs/adr/0008-settings-definition-and-effective-configuration-resolver.md`](../adr/0008-settings-definition-and-effective-configuration-resolver.md)
  — control-stack fields (`source_label`, `expires_at`,
  `offline_fallback`, `control_authority`, `narrowing_ceiling_active`)
  every settings-facing experiment must respect.
- [`/docs/adr/0011-capability-lifecycle-and-dependency-markers.md`](../adr/0011-capability-lifecycle-and-dependency-markers.md)
  — lifecycle-state vocabulary and dependency-marker rules.
- [`.t2/docs/Aureline_Technical_Design_Document.md`](../../.t2/docs/Aureline_Technical_Design_Document.md)
  Appendix CH — lifecycle expectations for Labs / Preview / Beta /
  Stable / Deprecated / DisabledByPolicy.
- [`.t2/docs/Aureline_UI_UX_Spec_Document.md`](../../.t2/docs/Aureline_UI_UX_Spec_Document.md)
  §18.25 — explainability requirements for flags, experiments, policy
  disables, kill switches, and schema-governance controls.

If this document and a register disagree, this document wins and the
register is updated in the same change.

## Purpose

The policy exists to prevent three failure modes:

1. Experimental behavior silently becoming product policy.
2. Saved artifacts or docs depending on unnamed flags.
3. Hidden developer toggles turning into durable dependency debt with
   no owner, expiry, or rollback path.

The current repository is pre-implementation, so the policy is about
**control truth**, not about a hosted flag backend. The current
prototype set and benchmark modes are still control rows and must be
governed the same way a future product-facing preview would be
governed.

## Row classes

The canonical register distinguishes four row kinds:

- `experiment` — a named non-stable capability or prototype path.
- `feature_flag` — a toggle or ceiling that can be on/off and must
  name its disable path.
- `benchmark_mode` — a named measurement mode whose output posture
  differs materially from the default path.
- `rollout` — a row whose primary job is to constrain cohort/ring and
  rollback posture.

Visibility is separate from lifecycle. A row may be:

- `contributor_visible_labs` — visible in the Labs projection today.
- `hidden_developer_toggle` — documented only in the canonical
  register; not shown in the Labs projection.
- `control_stack_reserved` — reserved because schemas and fixtures
  already depend on the vocabulary even though the runtime does not
  yet ship it.
- `ci_rollout_only` — visible as governance metadata for CI/release
  lanes, not as a contributor-facing Labs item.

## Required fields

Every experiment-register row must carry, at minimum:

- `owner_dri`
- `public_label`
- `lifecycle_state`
- `default_posture`
- `support_note`
- either `review_by` or `expires_on`
- `provider_chain`
- `embedded_default_behavior`
- `local_resolution_requirement`
- `offline_resolution_posture`
- `remote_fetch_failure_may_block_startup`
- `policy_override_posture`
- `kill_switch`
- `rollback_path`
- `artifact_dependencies`

These are not optional narrative aids. They are the control contract
future tooling, docs, support, and release evidence consume.

## Control-stack rules

### 1. Local-first resolution

- A row that can affect protected editing, save, or startup paths must
  resolve locally from embedded defaults plus any cached or signed
  local material. Remote fetch failure may not block startup for these
  rows.
- `offline_resolution_posture` must use the ADR-0008 vocabulary:
  `authoritative_local`, `last_known_good_signed`, `cache_only`, or
  `unavailable_offline`.
- A row that depends on optional remote or managed input still records
  the local behavior first. `Unavailable` is not enough; the fallback
  path must be named.

### 2. Provider-chain disclosure

- `provider_chain` names the actual resolution order for the row. At
  this milestone that is usually `embedded_default` plus a local CLI
  or CI input; the reserved settings-control row additionally names
  `signed_local_admin_bundle` and `optional_managed_override`.
- `embedded_default_behavior` must say what happens when no later
  provider contributes. Example: `inactive_until_invoked`,
  `defaults_to_smoke_subset`, or `preserve_last_known_local_value`.
- A row may not imply a remote provider it does not actually have.

### 3. Policy overrides and kill switches

- `policy_override_posture = not_allowed` means the row may be changed
  only by its local/default provider chain.
- `policy_override_posture = disable_only` means policy may turn the
  row off or narrow it, but may not widen it.
- `policy_override_posture = narrowing_allowed` is reserved for rows
  like the ADR-0008 settings experiment-rollout layer, where policy
  may cap or narrow behavior but never silently widen trust, egress,
  write scope, or startup dependencies.
- Every row must name a `kill_switch.source_kind`, a `source_ref`, and
  a visible fallback behavior. "Just stop using it" is not a valid
  kill-switch policy.

Policy disables and emergency kill switches must surface as explicit
state, not silent disappearance. When a row is disabled:

- the lifecycle state becomes `disabled_by_policy` on the affected
  projection,
- the disable source or safety reason remains inspectable,
- the fallback path remains named,
- dependent artifacts remain inspectable even if they are no longer
  re-generated by default.

## Lifecycle actions

### Promote

Promote a row only when all of the following are true:

- the row has a current owner, lifecycle state, and review or expiry
  date,
- the fallback and rollback path are already documented,
- artifact dependencies are named explicitly,
- the Labs projection is updated if the row is meant to be visible.

Promotion from `labs` to `preview`, `beta`, or `stable` requires the
same label change everywhere the row is surfaced: canonical register,
Labs register, docs, and any dependent artifacts or screenshots.

### Renew

Renew a row when the experiment is still justified but not ready for
promotion or removal. Renewal must:

- move `review_by` or `expires_on`,
- keep the same row identity,
- update `support_note` if the known limits changed,
- leave artifact dependencies intact unless they were actually
  cleared.

### Demote

Demote when the experiment needs a narrower audience or weaker claim.
Typical cases:

- `preview` back to `labs` because evidence no longer supports broad
  testing,
- contributor-visible Labs row removed from the Labs projection but
  kept in the canonical register,
- rollout row narrowed from a broader ring to a smaller one.

Demotion is a visible control event. The old label must not silently
remain on artifacts or docs after the row narrows.

### Disable

Disable when a policy ceiling or kill switch has fired, or when a row
must be held off while keeping its identity and artifact lineage
inspectable. A disabled row must still expose:

- source of disable,
- fallback behavior,
- rollback path,
- dependent artifacts that were produced while it was enabled.

### Remove

Remove a row only after all of the following are true:

- no current artifact or docs dependency remains,
- the replacement path or retirement note is recorded,
- the Labs projection is removed if present,
- the row is either retired in place or superseded by a new named row.

Do not delete a row simply because its implementation moved.

## Artifact dependency rules

- Any saved artifact, fixture corpus, workflow seed, schema contract,
  or doc projection that depends on a non-stable row must be named in
  `artifact_dependencies`.
- A row whose default change would alter contributor-visible behavior
  must declare `default_shift_change_log_posture`.
- Hidden developer toggles may use `docs_update_required`; editing-
  critical or settings-default rows should use
  `release_note_required` even before a public release channel exists,
  because those shifts materially change behavior and migration risk.

At the current milestone, explicit dependency marking matters most for:

- prototype-generated artifacts under `artifacts/`,
- fixtures and schemas that reserve future control-stack vocabulary,
- benchmark-lab dashboard seed files,
- contributor-facing README or prototype pages that describe a mode as
  supported.

## Labs inventory rules

- A row appears in `labs_register.yaml` only when it is intentionally
  contributor-visible and has a stable `public_label`, support note,
  review/expiry, feedback path, dependency summary, kill-switch
  source, and offline posture.
- Hidden developer toggles never appear in the Labs projection.
- The Labs projection may point at a prototype, a side-channel smoke
  path, or a benchmark mode. "Labs" here means visible experimental
  inventory, not end-user general availability.

At the current milestone the repository docs and YAML registers are
the visible inventory. A later in-product Labs surface must project
from these rows, not invent a parallel inventory.

## Current seeded families

The current register intentionally covers three groups:

- the current provisional prototypes and side-channel smoke paths
  (`buffer`, `vfs`, `large_file`, `graph`, `reactive_state`,
  `text_stack`).
- Optional benchmark modes and rollout boundaries
  (`smoke_subset`, `reference_capture` rollout).
- Hidden developer toggles that materially affect reviewable evidence
  (`regression_demo`, `skip_build`, `verify_seed_only`) plus the
  reserved settings experiment-rollout control-stack binding.

That split is deliberate: contributor-visible experiments belong in
the Labs projection; hidden developer toggles remain in the canonical
register only.
