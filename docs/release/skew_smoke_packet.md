# Skew-smoke packet: side-by-side channel coexistence, state-schema migration, and helper/agent attach against skewed clients

This packet freezes one shared contract for Aureline's skew and drift
states so release, migration, helper/agent, and support work can inspect
the same record instead of inventing per-surface terminology. Each seed
case is a versioned decision object describing which surface the skew
is on, which explicit skew state applies, which compatibility row and
version-skew-register case it maps to, which support-packet family it
routes through, and which promotion decision it justifies.

If this packet, the
[`skew_cases/`](../../fixtures/release/skew_cases/) fixture corpus,
and the [`skew_examples/`](../../artifacts/release/skew_examples/)
reviewer examples disagree with
[`artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml),
[`artifacts/compat/version_skew_register.yaml`](../../artifacts/compat/version_skew_register.yaml),
[`schemas/release/compatibility_row.schema.json`](../../schemas/release/compatibility_row.schema.json),
[`docs/release/install_topology_plan.md`](./install_topology_plan.md),
[`docs/release/compatibility_report_template.md`](./compatibility_report_template.md),
or [`docs/adr/0017-release-posture-artifact-families-and-promotion-gates.md`](../adr/0017-release-posture-artifact-families-and-promotion-gates.md),
those governing sources win and this packet plus its companion
artifacts update in the same change.

## Companion artifacts

- [`/fixtures/release/skew_cases/`](../../fixtures/release/skew_cases/)
  — seed cases covering side-by-side install, state-schema migration
  (old-to-new and new-to-old directions), helper/agent attach against a
  skewed client, rollback to a prior channel/build, and an unknown
  state that routes to probe rather than silently widening.
- [`/artifacts/release/skew_examples/`](../../artifacts/release/skew_examples/)
  — reviewer-facing examples keyed by the same `skew_case_id` values so
  compatibility reports, support exports, and shiproom reviewers can
  read one paragraph per surface without re-describing skew.
- [`/artifacts/compat/qualification_matrix_seed.yaml`](../../artifacts/compat/qualification_matrix_seed.yaml)
  and
  [`/artifacts/compat/version_skew_register.yaml`](../../artifacts/compat/version_skew_register.yaml)
  — the qualification rows and the per-combination skew cases every
  smoke case cites by stable id instead of free-text aliasing.
- [`/schemas/release/compatibility_row.schema.json`](../../schemas/release/compatibility_row.schema.json)
  — compatibility-row record every smoke case projects into when a
  release or certified-archetype report is assembled.
- [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
  and
  [`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml)
  — install-profile cards and per-channel state-root separation rules
  the side-by-side and rollback cases cite.
- [`/artifacts/release/artifact_family_map.yaml`](../../artifacts/release/artifact_family_map.yaml)
  and
  [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
  — release-posture and promotion-gate vocabulary the promotion
  decision classes below bind to.
- [`/docs/state/migration_and_restore_playbook.md`](../state/migration_and_restore_playbook.md)
  and
  [`/schemas/state/restore_provenance.schema.json`](../../schemas/state/restore_provenance.schema.json)
  — migration-result labels and restore-provenance fields the state-
  schema cases compose with.
- [`/schemas/support/support_packet_index.schema.json`](../../schemas/support/support_packet_index.schema.json)
  — support-packet family registry the skew cases route through when
  an export is required.
- [`/schemas/support/recovery_action.schema.json`](../../schemas/support/recovery_action.schema.json)
  and
  [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md)
  — recovery-rung vocabulary cited when a skew state's repair path
  hands off to the recovery ladder.

## Normative sources projected here

- `.t2/docs/Aureline_PRD.md` §9.12 (enterprise deployment hooks),
  §10.22 (support export), and §10.23 (recovery ladder).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §8.10 (fault
  domain and supervisor), §25.9 (install, portable-mode, and fleet-
  rollout architecture), Appendix BA (platform installation matrix and
  fleet rollout rings), and §26 (remote attach and helper envelopes).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §6.12 (About, update, and
  diagnostics surfaces) and §22.20 (Support Center).
- `.t2/docs/Aureline_Milestones_Document.md` §6.18 (install and update
  behaviour), §3.20 (supportability), and §3.21 (evidence).

If this document disagrees with those sources, those sources win and
this packet plus the companion artifacts update in the same change.

## Shared header

```yaml
schema_version: 1
header_kind: evidence_packet_header
packet_family: skew_smoke_packet
packet_id: release.skew_smoke.seed
evidence_id: evidence.release.skew_smoke.packet
title: Skew-smoke packet covering side-by-side channel coexistence, state-schema migration, helper/agent attach, and rollback outcome labels
ownership:
  owner_dri: "@ahmeddyounis"
  evidence_owner: "@ahmeddyounis"
  backup_owner: null
  backup_waiver: single-maintainer-backup
coverage:
  requirement_ids:
    - REL-SKEW-001
    - REL-COMPAT-005
    - REL-SUPPORT-001
    - OPS-SUP-005
    - GOV-EVID-901
    - GOV-CORPUS-901
  claim_row_refs:
    - packet_row:skew_smoke.skew_state_honesty
    - packet_row:skew_smoke.surface_coverage
    - packet_row:skew_smoke.compatibility_report_linkage
    - packet_row:skew_smoke.version_skew_register_linkage
    - packet_row:skew_smoke.support_packet_linkage
    - packet_row:skew_smoke.promotion_decision_linkage
  covered_lanes:
    - release_evidence
    - support_export
    - docs_public_truth
    - governance_packets
result_status: seed_only
visibility_class: internal
freshness:
  captured_at: 2026-04-23T00:00:00Z
  stale_after: P30D
  freshness_class: warm_cached
  source_revision: skew_smoke_seed@1
  trigger_revision: skew_smoke_packet@1
environment:
  channel_context: not_applicable
  deployment_context:
    - not_applicable
  environment_summary: >
    Seed packet over the frozen qualification-matrix rows, the version-
    skew register cases, the compatibility-row boundary schema, the
    install-topology matrix, and the state-migration playbook already
    landed in this repository. No live updater, helper-attach runtime,
    or managed fleet is wired to this packet yet; claims are
    structural only.
artifact_links:
  supporting_evidence_ids:
    - evidence.release.skew_smoke.seed
    - evidence.compat.qualification_matrix_seed
    - evidence.compat.version_skew_register
    - evidence.release.install_topology_matrix
    - evidence.state.migration_and_restore_playbook
    - evidence.support.support_packet_index
  exact_build_identity_refs: []
  fixture_refs:
    - fixtures/release/skew_cases/
  archetype_refs: []
  source_anchor_refs:
    - docs/release/install_topology_plan.md
    - docs/release/compatibility_report_template.md
    - docs/adr/0017-release-posture-artifact-families-and-promotion-gates.md
    - docs/state/migration_and_restore_playbook.md
    - docs/support/recovery_ladder_packet.md
    - artifacts/compat/qualification_matrix_seed.yaml
    - artifacts/compat/version_skew_register.yaml
    - artifacts/release/install_topology_matrix.yaml
    - artifacts/release/state_root_map.yaml
    - artifacts/release/artifact_family_map.yaml
    - artifacts/release/promotion_gate_map.yaml
    - schemas/release/compatibility_row.schema.json
    - schemas/state/restore_provenance.schema.json
    - schemas/support/recovery_action.schema.json
    - schemas/support/support_packet_index.schema.json
  waiver_refs: []
  known_limit_refs: []
  migration_packet_refs: []
```

## Summary

This seed packet freezes:

- one closed `skew_state_class` vocabulary pinned to the five states
  the milestone requires: `compatible`, `degraded`, `blocked`,
  `repairable`, and `unknown_requires_probe` — each mutually
  exclusive, each carrying its own promotion and support rules;
- one closed `skew_surface_class` vocabulary covering the four surfaces
  skew must be inspectable on: `side_by_side_install`,
  `state_schema_migration`, `helper_agent_attach`, and
  `downgrade_upgrade_rollback`;
- one closed `outcome_label_class` vocabulary for upgrade, downgrade,
  rollback, side-by-side coexist, attach-success, attach-degraded, and
  attach-blocked outcomes so support, release, and diagnostics do not
  mint their own verbs for what happened to the user;
- one closed `blocked_vs_degraded_class` vocabulary that keeps the
  difference between a hard stop, a policy block, a capability-subset
  degradation, and a read-only degradation explicit rather than
  collapsing them into a single "didn't work" label;
- one closed `promotion_decision_class` vocabulary mapping each skew
  state to a shiproom outcome so the packet cannot quietly imply
  promotion when the state is blocked or unknown;
- one closed `support_packet_routing_class` vocabulary keyed to the
  support-packet family registry so an export always knows which
  family a skewed session belongs to;
- one `skew_smoke_case_record` shape every smoke case emits, with
  stable preconditions, surface, state, outcome label, compatibility-
  report linkage, version-skew-register linkage, support-packet-family
  linkage, promotion-decision linkage, preserved-state set, capability-
  impact set, and reviewer-facing explanation fields; and
- one seeded smoke case per required surface — side-by-side install,
  state migration in both schema directions, helper/agent attach
  against a skewed client, rollback to a prior channel or build, and
  an unknown-requires-probe case — so release/support tasks can
  reference one skew packet family when diagnosing drift instead of
  inventing per-surface terminology.

It does not claim a live updater, helper-attach runtime, fleet rollout
controller, or hosted shiproom is wired up. It claims only that the
decision object, the state and surface honesty rules, and the linkage
rules to compatibility reports, support packets, and promotion
decisions now exist in one reviewable form and reuse the frozen
compatibility, install-topology, state-migration, and support-packet
vocabularies already landed in this repository.

## Claim coverage

| Packet row | Requirement id(s) | Status | Visibility | Supporting evidence ids | Notes |
|---|---|---|---|---|---|
| `packet_row:skew_smoke.skew_state_honesty` | `REL-SKEW-001`, `REL-COMPAT-005` | `seed_only` | `internal` | `evidence.release.skew_smoke.seed` | Closed `skew_state_class` set distinguishes compatible, degraded, blocked, repairable, and unknown without collapsing them into one vague status. |
| `packet_row:skew_smoke.surface_coverage` | `REL-SKEW-001`, `GOV-CORPUS-901` | `seed_only` | `internal` | `evidence.release.skew_smoke.seed` | Every smoke case binds exactly one `skew_surface_class` and one stable case id. |
| `packet_row:skew_smoke.compatibility_report_linkage` | `REL-COMPAT-005`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.compat.qualification_matrix_seed` | Every smoke case cites a `compatibility_row_ref` resolving into `artifacts/compat/qualification_matrix_seed.yaml`. |
| `packet_row:skew_smoke.version_skew_register_linkage` | `REL-COMPAT-005`, `GOV-EVID-901` | `seed_only` | `internal` | `evidence.compat.version_skew_register` | Every smoke case cites a `version_skew_register_ref` resolving into `artifacts/compat/version_skew_register.yaml` so the `supported`/`best_effort`/`untested`/`unsupported` bucket is explicit. |
| `packet_row:skew_smoke.support_packet_linkage` | `REL-SUPPORT-001`, `OPS-SUP-005` | `seed_only` | `internal` | `evidence.support.support_packet_index` | Every smoke case names one or more support-packet families the session routes through when export is required. |
| `packet_row:skew_smoke.promotion_decision_linkage` | `REL-SKEW-001`, `GOV-CORPUS-901` | `seed_only` | `internal` | `evidence.release.skew_smoke.seed` | Every smoke case carries a `promotion_decision_class` so the packet never implies promotion when the skew state is `blocked` or `unknown_requires_probe`. |

## Skew-smoke case contract

Every row in
[`skew_cases/`](../../fixtures/release/skew_cases/) projects onto one
`skew_smoke_case_record` and freezes the following fields. Field names
and tokens MUST match the vocabularies below exactly; additive-minor
changes widen a vocabulary only after a decision row lands in
`artifacts/governance/decision_index.yaml`.

- `skew_case_id` — stable dotted id (for example
  `skew_case:release.side_by_side.stable_preview_coexist`). Ids are
  additive-only; repurposing is breaking.
- `surface_class` — one of the four tokens from the frozen
  `skew_surface_class` set below.
- `skew_state_class` — one of the five tokens from the frozen
  `skew_state_class` set below.
- `outcome_label_class` — one of the tokens from the frozen
  `outcome_label_class` set below.
- `blocked_vs_degraded_class` — one of the tokens from the frozen
  `blocked_vs_degraded_class` set below. `not_applicable_compatible`
  is the required value whenever `skew_state_class = compatible`.
- `preconditions` — one or more reviewable sentences naming the
  boundary-, schema-, or attach-level condition that MUST hold before
  the case applies. Each sentence SHOULD quote a boundary label from
  `artifacts/compat/qualification_matrix_seed.yaml`.
- `compatibility_row_ref` — stable `compat_row:*` id from
  `artifacts/compat/qualification_matrix_seed.yaml`.
- `version_skew_register_ref` — stable `skew_case:*` id from
  `artifacts/compat/version_skew_register.yaml`.
- `install_topology_refs` — optional list of `install_profile_card` or
  `state_root` refs the surface depends on; required when
  `surface_class = side_by_side_install` or
  `surface_class = downgrade_upgrade_rollback`.
- `state_migration_refs` — optional list of restore-provenance or
  migration-session refs; required when
  `surface_class = state_schema_migration`.
- `helper_agent_refs` — optional list of helper-family or agent-family
  refs; required when `surface_class = helper_agent_attach`.
- `support_packet_routing_classes` — one or more tokens from the
  frozen `support_packet_routing_class` set below. Every case MUST
  name at least one family so an export never silently drops into the
  generic lane.
- `promotion_decision_class` — one of the tokens from the frozen
  `promotion_decision_class` set below. Cases whose
  `skew_state_class` is `blocked` MUST carry `no_go`; cases whose
  `skew_state_class` is `unknown_requires_probe` MUST carry
  `pending_probe`.
- `preserved_state_classes` — state classes the surface MUST NOT
  mutate regardless of skew outcome. `user_authored_files` appears on
  every case by rule.
- `capability_impact_classes` — capabilities the skew state disables,
  narrows, or makes read-only while the case is the current posture.
- `recovery_rung_ref` — optional `recovery_action:*` id pinning the
  default recovery rung if the case needs local repair or escalation;
  required when `skew_state_class = repairable` or
  `skew_state_class = blocked` and a rung is known.
- `explanation_fields` — four reviewer-facing sentences (user-facing
  summary, compatibility-report summary, support summary, promotion
  summary) the Support Center, About/Help, release evidence, and
  shiproom MAY render verbatim. These are the stable strings that
  replace free-text drift descriptions.

### `skew_state_class` (frozen)

Five states, mutually exclusive. A row that cannot cite one of these
MUST NOT claim a skew posture at all.

| Token | Meaning |
|---|---|
| `compatible` | The skew combination falls inside the claimed window and no capability narrowing is implied. Corresponds to `supported` bucket entries in `version_skew_register.yaml`. |
| `degraded` | The combination stays usable but narrows an explicit capability subset (for example review-only attach, read-only cache, caveated archetype row). Corresponds to `best_effort` bucket entries; the case MUST state which capability subset is narrowed. |
| `blocked` | The combination is explicitly unsupported. The surface fails closed, quarantines, or refuses the action. Corresponds to `unsupported` bucket entries with `outside_window_posture` values of `fail_closed`, `explicitly_unsupported`, or `read_only` combined with a privileged write. |
| `repairable` | The combination is out-of-window but a deterministic repair path exists (for example additive schema migration that succeeds, checkpoint-backed rollback, or cache/index regeneration). The case MUST cite a `recovery_rung_ref`. |
| `unknown_requires_probe` | Diagnosis has not converged; the packet refuses to label the state before a probe lands. Corresponds to `untested` bucket entries. The case MUST carry `promotion_decision_class = pending_probe`. |

### `skew_surface_class` (frozen)

| Token | Meaning |
|---|---|
| `side_by_side_install` | Two channels (for example stable and preview) coexist on one host. Scope includes owning channel, state-root separation, update marker ownership, and side-by-side relation class from the install-topology matrix. |
| `state_schema_migration` | State, profile, or workspace artifact produced under one schema epoch is read or restored under another. Includes both old-to-new and new-to-old directions. |
| `helper_agent_attach` | A helper binary or remote agent attaches to a client whose contract version, WIT ABI, or capability set differs from the helper/agent's. |
| `downgrade_upgrade_rollback` | A user or operator moves between channels/builds via upgrade, downgrade, or rollback and the session inherits residual state from the prior build. |

### `outcome_label_class` (frozen)

| Token | Meaning |
|---|---|
| `side_by_side_coexist_ok` | Channels coexist; no cross-channel state merge implied. |
| `upgrade_applied_exact` | Destination build reads prior state with `exact` fidelity. |
| `upgrade_applied_compatible` | Destination build reads prior state with `compatible` fidelity; additive migration succeeded. |
| `upgrade_applied_layout_only` | Only layout and non-authoritative state carried; authored truth preserved. |
| `downgrade_applied_compatible` | Destination build reads prior-newer state with `compatible` fidelity within the additive window. |
| `downgrade_blocked_newer_schema` | Destination build cannot read newer-schema state; surface falls back to read-only or refuses. |
| `rollback_applied` | Rollback to prior channel/build succeeded; side-by-side state-root ownership preserved. |
| `attach_success_full` | Helper or agent attach negotiated the full capability intersection. |
| `attach_degraded_review_only` | Helper or agent attach succeeded in review/read mode only; write-capable paths disabled. |
| `attach_blocked_unsupported` | Helper or agent attach refused because contract or permission requirement is outside the window. |
| `probe_required` | Outcome unknown; the surface routes to a probe rather than publishing a verdict. |

### `blocked_vs_degraded_class` (frozen)

| Token | Meaning |
|---|---|
| `not_applicable_compatible` | Required value whenever `skew_state_class = compatible`. |
| `degraded_capability_subset` | Capability set narrows (for example review-only attach); user retains partial function. |
| `degraded_read_only` | Writes are disabled but reads still succeed. |
| `blocked_hard_stop` | The surface fails closed regardless of user consent. |
| `blocked_policy` | Managed policy forced a narrower posture even when local state would otherwise allow the action. |
| `unknown_requires_probe` | Degradation vs. block cannot be decided yet. |

### `promotion_decision_class` (frozen)

| Token | Meaning |
|---|---|
| `promote` | The skew state does not block promotion; the case appears in compatibility reports as a supported row. |
| `ship_narrowed_claim` | The skew state narrows the claim but does not block shipment; release evidence MUST cite the narrowing case id. |
| `no_go` | The skew state blocks RC/stable promotion regardless of binary build success. The packet routes the case to an advisory or revocation path. |
| `pending_probe` | Promotion is held pending a probe or re-run. The packet routes to a probe, not to a claim. |

Rule: a case whose `skew_state_class = blocked` MUST carry
`promotion_decision_class = no_go`; a case whose
`skew_state_class = unknown_requires_probe` MUST carry
`promotion_decision_class = pending_probe`. Violation is non-conforming.

### `support_packet_routing_class` (frozen)

| Token | Meaning |
|---|---|
| `exact_build_support` | Session exports route through the exact-build support-packet family in `schemas/support/support_packet_index.schema.json`. |
| `rollback_review` | Session exports route through the rollback-review family (debug retention and symbol sidecars). |
| `object_issue_handoff` | Session exports route through the object-issue handoff family when a specific object is in scope. |
| `compatibility_report` | Session contributes a compatibility-row record to the next compatibility report. |
| `advisory_or_revocation` | Session routes to the advisory or revocation lane described in the release-posture ADR. |

## Mapping rules

The skew-smoke packet is not a standalone artifact. It MUST compose
with the three release-governance families already frozen in this
repository:

1. **Compatibility reports** — every smoke case MUST project onto a
   `compatibility_row` record (see
   `schemas/release/compatibility_row.schema.json`) by quoting the
   `compatibility_row_ref`, the `version_skew_register_ref`, and the
   `skew_state_class`. A compatibility report that advertises support
   for a combination not covered by a smoke case MUST land a new
   smoke case before the report is published.
2. **Support packets** — every smoke case MUST name at least one
   `support_packet_routing_class` so exports route through a known
   family instead of a generic lane. Cases whose skew state is
   `blocked` or `repairable` MUST additionally cite a recovery rung
   (`recovery_action:*`) so local repair or escalation is never
   described as free-text troubleshooting advice.
3. **Promotion decisions** — every smoke case MUST carry a
   `promotion_decision_class` so shiproom and release evidence cannot
   implicitly promote an unknown or blocked combination. Cases whose
   decision class is `ship_narrowed_claim` MUST cite the narrowing
   text in `explanation_fields.promotion_summary`; cases whose
   decision class is `no_go` MUST cite the advisory/revocation routing
   in the same field.

Rule: a release, support, or docs/help surface MUST cite a skew case
by its stable `skew_case_id` rather than by free-text drift advice. A
compatibility report, release-evidence packet, or support bundle that
describes skew in narrative without naming a case id is non-conforming.

## Seeded smoke cases

The seed corpus in
[`skew_cases/`](../../fixtures/release/skew_cases/) covers the four
scenarios the milestone requires plus one `unknown_requires_probe`
case that proves the packet does not silently widen. Every case row
binds 1:1 to a `skew_smoke_case_record` and names its default surface,
state, outcome label, compatibility-row ref, version-skew-register
ref, support-packet routing class, and promotion decision.

| Case id | Surface | Skew state | Outcome label | Promotion decision |
|---|---|---|---|---|
| `skew_case:release.side_by_side.stable_preview_coexist` | `side_by_side_install` | `compatible` | `side_by_side_coexist_ok` | `promote` |
| `skew_case:release.state_schema.old_to_new_additive` | `state_schema_migration` | `repairable` | `upgrade_applied_compatible` | `ship_narrowed_claim` |
| `skew_case:release.state_schema.new_to_old_blocked` | `state_schema_migration` | `blocked` | `downgrade_blocked_newer_schema` | `no_go` |
| `skew_case:release.helper_agent_attach.skewed_client_degraded` | `helper_agent_attach` | `degraded` | `attach_degraded_review_only` | `ship_narrowed_claim` |
| `skew_case:release.rollback.prior_channel_build_compatible` | `downgrade_upgrade_rollback` | `compatible` | `rollback_applied` | `promote` |
| `skew_case:release.helper_agent_attach.unknown_probe_required` | `helper_agent_attach` | `unknown_requires_probe` | `probe_required` | `pending_probe` |

Every case also names the compatibility-row ref it is the default
smoke case for, the version-skew-register case it composes with, the
support-packet family routing, and the preserved/impacted state and
capability classes so compatibility reports, support exports, and
shiproom reviewers render the rung without re-describing skew.

## Reviewer examples

The
[`skew_examples/`](../../artifacts/release/skew_examples/) directory
seeds one reviewer-facing example per smoke case. Examples are
deliberately shaped so review can read five things in one paragraph:
which surface the skew is on, which skew state applies, what the user
experiences (the outcome label), which compatibility row and support
packet route the case, and what promotion decision it justifies.
Examples quote the case's `skew_case_id` and the stable tokens from
the closed vocabularies above; they are not prose substitutes for the
packet.

## What this seed does not promise

- No live updater, helper-attach runtime, managed fleet controller, or
  hosted shiproom is wired to this packet. The decision object, the
  state honesty rules, and the mapping rules are reviewable objects
  only.
- No new JSON schema lands alongside this packet. Smoke cases project
  onto already-frozen contracts: `compatibility_row.schema.json`,
  `restore_provenance.schema.json`, `recovery_action.schema.json`, and
  `support_packet_index.schema.json`. Adding a new smoke case row is
  additive-minor provided it reuses the frozen `skew_state_class`,
  `skew_surface_class`, `outcome_label_class`,
  `blocked_vs_degraded_class`, `promotion_decision_class`, and
  `support_packet_routing_class` vocabularies. Repurposing any of those
  tokens is breaking and requires a new decision row in
  `artifacts/governance/decision_index.yaml`.
- No full mixed-version test matrix, helper/remote-agent runtime, or
  future remote-agent combination is committed by this packet.
  Additional combinations land as new smoke cases with their own case
  ids, not by widening the existing ones.
- No numeric strike-budget, freshness window, or adjacent-window
  number is committed. Windows remain the tokens already pinned in
  `artifacts/compat/version_skew_register.yaml`; the smoke packet
  quotes them rather than minting new ones.
