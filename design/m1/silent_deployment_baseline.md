# Silent-deployment baseline (design packet)

This packet is the human-readable design record for the silent /
managed deployment behaviour the product publishes at the current
milestone. It is the second named consumer of the install-topology
truth seed at
[`/artifacts/install/state_root_matrix.yaml`](../../artifacts/install/state_root_matrix.yaml).

The packet does not introduce a new vocabulary. It is the design
narrative that reviewers read alongside the seeded rows and the
return-code seed at
[`/artifacts/release/silent_deployment_seed.yaml`](../../artifacts/release/silent_deployment_seed.yaml).
If the packet and the seed disagree, the seed wins and this packet is
updated in the same change.

## Why a baseline (not a depth claim)

The product ships a baseline silent / managed deployment story so that
operators, support, and release reviewers know exactly what is and is
not supported. The baseline is deliberately narrow:

- We document scriptable installs, updates, pins, rollbacks, and
  uninstalls with a stable return-code family and remediation
  pointers, drawn from
  [`/artifacts/release/silent_deployment_seed.yaml`](../../artifacts/release/silent_deployment_seed.yaml).
- We document partial managed-deployed behaviour: an admin-policy
  fleet image can deploy silently and can pin to a prior build, but we
  do not yet ship full fleet-wide silent rollout orchestration.
- We document portable swap: portable trees do not require silent
  deployment at all, because they are swapped at the file-system level
  rather than installed.

What we do **not** ship at the current milestone:

- **Full fleet-wide silent deployment with rollback orchestration.** The
  product does not own a fleet rollout service yet. The token
  `managed_silent_full` is deliberately absent from
  `silent_deployment_baseline_class_vocabulary`. Any row that publishes
  it is non-conforming and the validation lane fails closed.
- **A managed deployment portal or scheduling service.** Out of scope.
- **A rollback / pin service.** Managed-deployed rows publish
  `managed_pin_to_prior_build` as a revert path; the actual pin
  mechanism is owned by the operator's fleet tooling and is a
  policy-bundle artefact rather than a product-side service.

## Baseline classes (seeded, supported)

| Class | Where it applies | What the product publishes today |
| --- | --- | --- |
| `not_supported` | Reserved for profiles that explicitly do not allow silent deployment. | Carried so the seed can record a profile honestly without a fallback class. |
| `scriptable_baseline_with_exit_codes` | `per_user_installed`, `per_machine_installed`, `offline_bundle`, `side_by_side_preview` | Scriptable install / update / pin / rollback / uninstall / verify with the stable return-code family. Honest exit codes; no enterprise rollout orchestration. |
| `managed_silent_partial` | `managed_deployed` | Admin-policy fleet image can deploy silently; pin to prior build is supported via `managed_pin_to_prior_build` revert path. Fleet-wide orchestration is owned by the operator. |
| `portable_swap_no_silent_required` | `portable` | Portable trees are swapped at the file-system level; no silent installer is required. |

## What rows MUST publish (seed cross-reference)

Each install-truth profile row in
[`/artifacts/install/state_root_matrix.yaml`](../../artifacts/install/state_root_matrix.yaml)
publishes the following silent-deployment-relevant fields:

- `silent_deployment_baseline_class` — one of the supported baseline
  classes above. Never `managed_silent_full`.
- `revert_path_class` — one of `in_app_revert`,
  `package_manager_revert`, `portable_swap`,
  `managed_pin_to_prior_build`, or `unsupported`. `managed_deployed`
  rows MUST NOT publish `unsupported`.
- `updater_owner_class` — names the entity that owns the updater path
  for the profile. `none_portable` is the explicit class for portable
  trees.
- `durable_state_root_class_refs` — the state roots the operator can
  inspect / back up before any silent operation; resolves into
  [`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml).

## Return-code family (out of seed scope, in baseline scope)

Scriptable installs, updates, pins, rollbacks, uninstalls, and verify
operations resolve their results into the return-code family at
[`/artifacts/release/silent_deployment_seed.yaml`](../../artifacts/release/silent_deployment_seed.yaml).
The baseline packet does not redefine the family; it points operators
at the seed and at the stable CLI exit-code model in
`.t2/docs/Aureline_Technical_Architecture_Document.md` §B.2.

## File / protocol associations and side-by-side coexistence

Silent deployment honesty depends on per-channel file / protocol
ownership. Every seeded row publishes:

- `file_association_ownership_class` — one of `per_channel_namespaced`,
  `not_registered`, or `managed_policy_owned`. Portable rows are
  `not_registered`; managed rows are `managed_policy_owned`; user / admin
  installs are `per_channel_namespaced` so stable and preview never
  fight over default-handler claims.
- `protocol_handler_ownership_class` — same vocabulary.
- `side_by_side_relation_class` and `paired_channel_class` — together
  describe whether the row coexists with another channel. The lane
  forbids `side_by_side_preview` rows from publishing `none`.

## Failure drills the lane reproduces

Silent-deployment-relevant drills the validation lane reproduces under
`--force-drill`:

| Row | Drill | Expected check id |
| --- | --- | --- |
| `portable.portable_stable` | `portable_silent_baseline_widened_to_managed_silent_full` | `install_topology_truth.silent_deployment_baseline_managed_silent_full_blocked_in_baseline` |
| `managed_deployed.stable` | `managed_deployed_revert_path_dropped_to_unsupported` | `install_topology_truth.revert_path_class_unsupported_blocked_for_managed_deployed` |

The first drill is the canonical guard against widening the baseline
into enterprise depth the product cannot execute honestly. The second
drill is the canonical guard against silently dropping the revert path
from a managed profile.

## Out of scope (recorded so reviewers can route requests)

- Device-management productization.
- Enterprise deployment orchestration (fleet rollout, ring promotion
  services, scheduled-deployment windows).
- Rollback-service automation as a product surface.

Requests for any of the above open a decision row in
`artifacts/governance/decision_index.yaml`; they do not land here.
