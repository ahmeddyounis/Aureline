# Backup-restore, failover, and key-rotation drill baseline (beta)

Reviewer-facing landing page for the first set of bounded drills that back
enterprise and managed assurances during beta. The baseline rehearses
backup-restore, failover, and key-rotation on each claimed managed /
enterprise row family across connected, mirror-only, offline, and
enterprise-managed beta profiles, and feeds claim downgrades back into the
product when drill evidence goes stale or breaks.

The canonical record kind is
`security_enterprise_drill_baseline_page_record`. The schema lives at
[`/schemas/security/enterprise_drill_baseline.schema.json`](../../../schemas/security/enterprise_drill_baseline.schema.json).
The beta module lives at
[`/crates/aureline-auth/src/enterprise_drill_baseline/mod.rs`](../../../crates/aureline-auth/src/enterprise_drill_baseline/mod.rs)
and the shell / headless consumer at
[`/crates/aureline-shell/src/enterprise_drill_baseline/mod.rs`](../../../crates/aureline-shell/src/enterprise_drill_baseline/mod.rs).
The seeded drill packets live as JSON under
[`/artifacts/security/m3/backup_restore_failover_drills/`](../../../artifacts/security/m3/backup_restore_failover_drills/).
The matrix axes live at
[`/artifacts/security/m3/backup_restore_failover_drills/enterprise_drill_baseline_matrix.yaml`](../../../artifacts/security/m3/backup_restore_failover_drills/enterprise_drill_baseline_matrix.yaml).

## Why this baseline exists

Enterprise and managed assurances ("we can restore the policy bundle if the
mirror is lost", "we can fail over to the declared secondary tenant", "we can
rotate the customer-managed wrap key without losing the claim") are not real
until the product has rehearsed them and recorded the evidence. This page is
the auditable record kind that makes those rehearsals real for the first set
of claimed beta row families.

## Row families covered

The first baseline rehearses every drill kind on three claimed managed /
enterprise row families:

- **`managed_policy_distribution`** — Managed policy distribution claims
  (policy packs, admin policy push, signed mirror). Pairs with the policy-pack
  and admin-audit-export beta pages.
- **`managed_credential_handle`** — Managed credential / vault handle claims
  (secret broker handles, keychain custody, BYOK wrap keys). Pairs with the
  secret-broker and secret-repair beta pages.
- **`enterprise_identity_session`** — Enterprise identity session claims
  (managed OIDC sessions, passkey step-up, tenant binding continuity). Pairs
  with the OIDC and passkey beta pages.

## Drill kinds

| Drill kind | What it rehearses |
| --- | --- |
| `backup_restore` | Restore the row family from its last signed snapshot, mirror, or air-gapped courier and replay it into the active claim. |
| `failover` | Fail the row family over from its primary path to its declared secondary path without widening sibling-lane authority. |
| `key_rotation` | Rotate the row family's signing or wrapping key material and confirm the rotated material drives the claim. |

Each row family carries one current drill packet per drill kind for nine
seeded packets in total. The validator's
`drill_kind_coverage_missing_for_family` defect fails closed if any family
loses a required drill kind.

## Protected states covered

- **Coverage.** Every claimed managed / enterprise row family has at least one
  current drill packet for backup-restore, failover, and key rotation. The
  validator's `drill_kind_coverage_missing_for_family` defect fails closed if
  any drill kind is dropped on any row family.
- **Claim downgrades feed off drills.** A drill packet's `evidence_freshness`
  of `stale_beyond_window` or `missing` must declare a non-`no_impact`
  `claim_impact_if_stale`. The validator's `stale_evidence_without_downgrade`
  defect refuses any drill that lets a claim stay on its previous authority
  after the drill evidence breaks. Conversely the
  `fresh_evidence_with_unexpected_downgrade` defect refuses a `fresh` drill
  that pretends a downgrade is in effect.
- **No silent widening on a sibling lane.** Every drill packet declares
  `sibling_lanes_unwidened`. The validator's `drill_sibling_lane_widened`
  defect refuses any drill that widens authority on a sibling lane.
- **No undeclared public fallback.** Every drill packet declares
  `no_public_endpoint_fallback` and `raw_private_material_excluded`. The
  validator's `hidden_public_endpoint_fallback` and
  `raw_private_material_exposed` defects refuse drills that allow undeclared
  fallback or expose raw private material.
- **Local editing preserved.** Every drill packet declares
  `local_editing_preserved`. The validator's
  `drill_local_editing_not_preserved` defect refuses drills that would block
  local-only work during the rehearsal.
- **Outcome shape.** A drill packet's outcome must match its drill kind. The
  validator's `outcome_does_not_match_drill_kind` defect refuses a
  backup-restore drill that records a failover outcome and vice versa.

## How enterprise packets and support playbooks consume the baseline

Enterprise packets and support playbooks consume the same drill packets the
product surfaces — without private interpretation. The support-export wrapper
preserves the typed defect vocabulary verbatim, excludes raw private material,
and re-asserts the four packet-level invariants
(`raw_private_material_excluded`, `no_public_endpoint_fallback_invariant`,
`local_editing_preserved_invariant`, `sibling_lanes_unwidened_invariant`).
Reviewers replay the same JSON to verify a claim, and the support flow exports
the same JSON to make a finding portable.

## How to inspect

The drill baseline is inspectable from the headless inspector binary that any
admin, support, or reviewer surface can run:

```sh
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- page
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- drill-packets
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- summary
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- support-export
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- defects
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- validate
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- drill-backup-restore
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- drill-failover
cargo run -q -p aureline-shell --bin aureline_shell_enterprise_drill_baseline -- drill-key-rotation
```

The `validate` subcommand exits non-zero when any defect is present and prints
the typed defects to stderr.

## Reviewer fixtures

The same JSON the binary emits is checked in under
[`/fixtures/security/m3/drill_inputs/`](../../../fixtures/security/m3/drill_inputs/)
so reviewers can replay the baseline without running cargo.

## Out of scope

This baseline does not add M5 / M6 commercial control-plane breadth. It only
covers the first set of claimed beta row families and the three drill kinds
the exit-gate calls out. Additional row families and additional drills (for
example credential reissuance under quarantine, or region-pair failover during
key rotation) land in subsequent waves.
