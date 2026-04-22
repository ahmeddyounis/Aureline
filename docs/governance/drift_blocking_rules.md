# Drift-blocking rules

This document defines how Aureline blocks or narrows public/support-
facing truth when product surfaces, docs, CLI/headless output, support
exports, release packets, or public-proof packets drift away from their
canonical owner artifacts.

Companion artifacts:

- [`/artifacts/governance/source_of_truth_map.yaml`](../../artifacts/governance/source_of_truth_map.yaml)
  — canonical owner-routing map.
- [`/artifacts/governance/public_truth_parity_matrix.yaml`](../../artifacts/governance/public_truth_parity_matrix.yaml)
  — channel-by-channel claim-row projection rules.
- [`/docs/governance/claim_manifest_contract.md`](./claim_manifest_contract.md)
  — canonical claim-row publication contract.
- [`/docs/docs/help_about_service_health_routes.md`](../docs/help_about_service_health_routes.md)
  — canonical route and destination-descriptor contract.

## Severity classes

| Severity class | Trigger | Required remediation | Promotion impact |
|---|---|---|---|
| `release_blocking_overclaim` | A downstream surface is broader than its canonical owner row, hides a downgrade reason, or presents stale proof as current truth. | Narrow or remove the claim in the same change that fixes the owner row or projection. | Blocks merge on protected public-truth changes and blocks release or proof promotion. |
| `same_change_blocker` | A canonical owner artifact changed but one or more required projections, known-limit notes, or support-window disclosures did not change with it. | Update the missing projections in the same change set; do not defer to follow-up cleanup. | Blocks merge. |
| `time_boxed_truth_defect` | Surfaces disagree, but the visible posture is already conservative and no broader public claim escaped. | File and route a truth-drift defect immediately; resolve before the next channel move or within 24 hours, whichever is sooner. | Blocks widening and blocks the next channel move while open. |
| `seed_gap_review_required` | A seeded or internal-only workflow lacks a full projection path but does not make an external claim. | Keep the row internal, experimental, or explicitly narrowed; record the gap in the owning review packet or dashboard. | Does not block internal review, but blocks public or support-language widening. |

## Named mismatch categories

Use these categories when filing or reviewing a drift defect:

| Category | Severity class | Example |
|---|---|---|
| `owner_row_missing` | `same_change_blocker` | A new Help/About row renders lifecycle or channel truth that is not backed by a canonical owner artifact. |
| `projection_broader_than_owner` | `release_blocking_overclaim` | CLI/help says `stable` while the capability or claim row is `preview` or `limited`. |
| `policy_disabled_hidden` | `release_blocking_overclaim` | A policy-disabled row disappears or appears available without the explicit blocked reason and fallback path. |
| `support_window_mismatch` | `same_change_blocker` | Release notes or docs omit the claim row's current support-window state or replacement route. |
| `known_limit_missing` | `same_change_blocker` | A public or support-facing row drops required caveat refs from the claim row. |
| `compatibility_or_skew_alias_drift` | `release_blocking_overclaim` | A surface invents a local compatibility or skew label instead of using the canonical row or skew case. |
| `proof_packet_out_of_sync` | `time_boxed_truth_defect` or `release_blocking_overclaim` | Shiproom dashboard or proof packet still cites outdated claim posture after the owner row narrowed. |

## Mandatory same-change-set cases

The following changes are not allowed to land as staggered follow-up
work. They require one change set.

### Claim-row changes

When any of these change on a `claim_row`:

- canonical copy,
- `effective_claim_posture`,
- `support_window_state`,
- `support_window_ref`,
- `known_limit_refs`,
- `exclusion_note_refs`,
- `compatibility_row_refs`,
- `version_skew_register_refs`,

the same change set must update every affected projection:

- `artifacts/governance/public_truth_parity_matrix.yaml`
- docs/help or release-note surfaces that quote the row
- support/export copy or packet references
- public-proof or evaluation packet templates when the row requires
  those channels

### Destination-descriptor changes

When any of these change on a destination descriptor:

- `display_source_version`
- `running_build_identity_ref`
- `version_match_state`
- `support_class`
- `client_scopes`
- `freshness_class`
- `availability_state`
- `offline_behavior`
- `preferred_route_class`
- `browser_handoff_reason`

the same change set must update:

- [`artifacts/docs/destination_descriptor_seed.yaml`](../../artifacts/docs/destination_descriptor_seed.yaml)
- [`docs/docs/help_about_service_health_routes.md`](../docs/help_about_service_health_routes.md)
- any claim-row or support/export copy that quotes the changed route
  posture or version/freshness meaning

### Experiment, Labs, and policy-disable changes

When any of these change on an experiment or control row:

- `lifecycle_state`
- `public_label`
- `support_note`
- `review_by` or `expires_on`
- `policy_override_posture`
- `kill_switch`
- `rollback_path`

the same change set must update:

- [`artifacts/governance/experiments_register.yaml`](../../artifacts/governance/experiments_register.yaml)
- [`artifacts/governance/labs_register.yaml`](../../artifacts/governance/labs_register.yaml)
  when the row is contributor-visible
- [`docs/governance/feature_flag_policy.md`](./feature_flag_policy.md)
  when vocabulary or policy meaning changed
- any dependent docs/help, claim-manifest, or support/export row that
  presents the row publicly

### Install, channel, and provenance changes

When install mode, install channel, updater owner, release channel, or
provenance semantics change, the same change set must update:

- [`artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
- [`docs/release/install_topology_plan.md`](../release/install_topology_plan.md)
- [`schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  or
  [`docs/build/exact_build_identity_model.md`](../build/exact_build_identity_model.md)
  when the exact-build meaning changed
- support/export or claim-manifest surfaces that expose the affected
  install, channel, or provenance truth

### Compatibility, skew, support-window, and caveat changes

When a compatibility row, skew case, support-window state, or required
known-limit note changes, the same change set must update:

- [`artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
- [`artifacts/compat/version_skew_register.yaml`](../../artifacts/compat/version_skew_register.yaml)
- [`artifacts/governance/claim_manifest_seed.yaml`](../../artifacts/governance/claim_manifest_seed.yaml)
- any release-note, migration, support/export, or public-proof surface
  that quotes the changed row

## Conservative resolution rule

If two surfaces disagree and the mismatch is unresolved, the more
conservative posture wins:

- narrower support class over broader support class
- limited or experimental over claim-bearing
- policy-disabled over silently omitted
- stale or degraded over ready
- explicit replacement-grade over legacy availability

No release, docs, support, or proof workflow may pick the more generous
interpretation while the canonical owner row still disagrees.

## Audit checklist

1. Start from the user-visible row, but classify it using
   [`source_of_truth_map.yaml`](../../artifacts/governance/source_of_truth_map.yaml).
2. Resolve the canonical owner artifact and inspect that row first.
3. Check whether the downstream surface is broader, narrower, or simply
   missing compared with the owner row.
4. Follow evidence joins:
   exact-build identity, compatibility row, skew case, route
   descriptor, or contract packet.
5. Follow caveat joins:
   `known_limit_refs`, `exclusion_note_refs`, migration notes, or
   replacement refs.
6. Assign the mismatch category and severity class from this document.
7. If the category is a same-change-set case, block merge until every
   required companion artifact is updated together.
8. If the category is time-boxed, record the owner and due date on the
   tracking issue and keep the public posture narrowed until closed.
