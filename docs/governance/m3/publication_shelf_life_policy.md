# M3 publication shelf-life policy

This policy is the M3-scoped extension of the cross-milestone evidence
freshness policy (`docs/governance/evidence_freshness_policy.md`). It
applies to every public-proof artifact named in the M3 public-proof
index (`artifacts/milestones/m3/public_proof_index.md`) and to every
review packet that cites that index.

Companion control artifacts:

- Public-proof index (canonical): `artifacts/milestones/m3/public_proof_index.md`
- Review-packet template: `artifacts/milestones/m3/review_packet_template.md`
- Evidence freshness policy (cross-milestone):
  `docs/governance/evidence_freshness_policy.md`
- Freshness SLOs (cross-milestone proof-class ceilings):
  `artifacts/governance/evidence_freshness_slos.yaml`
- Rerun-trigger catalog (cross-milestone trigger ids):
  `artifacts/governance/evidence_rerun_triggers.yaml`
- Claim manifest (M3 beta-bearing rows):
  `artifacts/release/m3/claim_manifest.json`
- Validator: `ci/check_m3_public_proof_index.py`

## Purpose

The M3 beta train publishes packets that read as current truth on docs,
Help/About, service-health, release notes, support exports, and partner
evaluation kits. This policy fixes the shelf-life of those packets so
they cannot stay green by inertia: each M3 public-proof family has an
explicit time window, an explicit rerun-trigger set, and an explicit
automatic downgrade behavior that downstream surfaces honor without
re-reading prose.

The policy is normative for the M3 beta channel. Stricter channel
caps (preview, hotfix, LTS) layered on top of this policy still apply;
this policy MUST NOT widen what a stricter cross-milestone class allows.

## Scope

This policy governs:

- every row in `artifacts/milestones/m3/public_proof_index.md`;
- every review packet that cites
  `artifacts/milestones/m3/review_packet_template.md`;
- every checked-in current-output ref the index names; and
- every downstream consumer surface that quotes a public-proof row by
  id (docs site, Help/About, service-health, release notes, support
  exports, partner evaluation kits, marketplace discovery copy).

This policy does NOT govern:

- internal scratch packets that are never claim-bearing;
- packet families that ship under a separate published policy (security
  advisory, transport governance — those rows fall under their own
  freshness ceilings); or
- evidence captured for retrospectives that does not project a current
  support claim.

## Freshness windows per claim family

Each M3 public-proof claim family inherits one cross-milestone proof
class. The window below is the maximum claim-bearing age for the M3
beta channel; the index row may declare a stricter window, but it
MUST NOT widen the window beyond the value here.

| Claim family | Proof class | Max `stale_after` | Stale-propagation profile |
|---|---|---|---|
| `boundary_truth` | `docs_claim_truth_proof` | `P14D` | `docs_truth_stale` |
| `exact_build_identity` | `docs_claim_truth_proof` | `P14D` | `claim_narrow_and_hold` |
| `benchmark_publication` | `benchmark_publication_proof` | `P14D` | `claim_narrow_and_hold` |
| `docs_freshness` | `docs_claim_truth_proof` | `P14D` | `docs_truth_stale` |
| `version_skew_truth` | `compatibility_report_proof` | `P14D` | `compatibility_retest_pending` |
| `launch_wedge` | `compatibility_report_proof` | `P14D` | `compatibility_retest_pending` |

The proof class and stale-propagation profile are stable ids from
`artifacts/governance/evidence_freshness_slos.yaml`. Downstream surfaces
read those ids; they do not interpret the table prose.

## Rerun-trigger ids per claim family

A packet expires immediately when any of its named rerun triggers
fires, even when the time window is still open. The M3 trigger sets
below are the minimum; review packets MAY name additional triggers,
but they MUST name at least the trigger ids listed here.

| Claim family | Required rerun-trigger ids |
|---|---|
| `boundary_truth` | `claim_row_or_channel_binding_changed`, `schema_or_packet_header_contract_changed`, `deployment_topology_or_boundary_changed`, `docs_truth_contract_or_pack_revision_changed` |
| `exact_build_identity` | `exact_build_identity_chain_changed`, `claim_row_or_channel_binding_changed`, `schema_or_packet_header_contract_changed` |
| `benchmark_publication` | `reference_hardware_image_changed`, `corpus_or_fixture_revision_changed`, `protected_metrics_or_fitness_catalog_changed`, `exact_build_identity_chain_changed` |
| `docs_freshness` | `docs_truth_contract_or_pack_revision_changed`, `claim_row_or_channel_binding_changed`, `schema_or_packet_header_contract_changed`, `interface_or_version_skew_window_changed` |
| `version_skew_truth` | `interface_or_version_skew_window_changed`, `schema_or_packet_header_contract_changed`, `claim_row_or_channel_binding_changed`, `support_window_or_release_family_changed` |
| `launch_wedge` | `reference_hardware_image_changed`, `corpus_or_fixture_revision_changed`, `claim_row_or_channel_binding_changed`, `support_window_or_release_family_changed` |

All trigger ids are from
`artifacts/governance/evidence_rerun_triggers.yaml`. The validator
fails closed when a public-proof row names a trigger id outside that
catalog.

## Automatic downgrade behavior

The validator derives one downgrade matrix from this policy and writes
it into the public-proof validation capture so docs, Help/About,
service-health, release notes, and support exports can downgrade stale
rows without reviewer interpretation.

Rules:

1. A packet older than `stale_after` for its claim family is treated as
   expired; its stale-propagation profile drives the lane status floor,
   the claim posture cap, the qualification badge, and the release
   posture cap.
2. A packet with a fired rerun trigger is treated as expired
   immediately, even when its time window remains open.
3. Missing freshness metadata is treated as expired for scorecard,
   signoff, claim, and promotion use; downstream surfaces MUST render
   the row as `evidence_stale` and MUST NOT render any current support
   wording until the packet is refreshed.
4. A stricter packet-local `stale_after` always wins over the proof
   class ceiling; a packet MAY choose `P0D` for seed-only or
   self-captured public-proof rows.
5. The downgrade is not advisory: docs/help/service-health copy MUST
   render the downgraded posture, claim-bearing release packets MUST
   pass through `hold_for_refresh` (or `no_go` when the profile names
   it), and signoff MUST NOT accept milestone or release close on an
   expired required public-proof row.

## Review cadence

- Routine refresh: each public-proof row SHOULD be refreshed at least
  once per beta release-candidate cycle, and ALWAYS before the
  release-candidate gate is opened.
- Trigger-driven refresh: any change source listed in the row's rerun
  triggers (hardware image, fixture set, fitness catalog, schema, etc.)
  forces a refresh in the same change set that introduces the change.
- Same-change-set rule: any update to the public-proof index, a claim
  family, a beta-surface row, or an archetype row MUST refresh the
  matching review packet in the same change set. The validator fails
  closed when these drift.

## Failure drill

To confirm the guardrail is live:

1. Temporarily move the captured-at timestamp of one row in
   `artifacts/milestones/m3/public_proof_index.md` to a value older than
   the stale-after window (or rename one current-output ref).
2. Re-run `python3 ci/check_m3_public_proof_index.py --repo-root .`;
   the validator MUST fail with an actionable error naming the expired
   row or missing artifact and the stale-propagation profile that would
   apply to downstream surfaces.
3. Restore the timestamp or ref and re-run; it MUST pass.

## Authoring rules

When a new M3 public-proof family lands, in the SAME change set:

1. Add the row to `artifacts/milestones/m3/public_proof_index.md` (both
   the prose tables and the canonical YAML block).
2. Pick one proof class id and one stale-propagation profile id from
   `artifacts/governance/evidence_freshness_slos.yaml`.
3. Name at least one rerun-trigger id from
   `artifacts/governance/evidence_rerun_triggers.yaml`.
4. Add or update the matching review packet under
   `artifacts/milestones/m3/review_packet_template.md`.
5. Run the validator and commit the regenerated capture.

A new row that ships without one of the above steps is treated as
non-existent for claim purposes; downstream surfaces MUST NOT quote it
until the row passes validation.
