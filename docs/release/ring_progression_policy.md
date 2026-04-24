# Ring progression, soak windows, rollback stops, and evidence resets

This policy defines Aureline's validation widening ladder. It answers
which ring a lane may occupy, what evidence is required before the lane
widens, how much normal workload must soak before the next widening, and
when the lane must hold or reset to a narrower ring.

Companion artifacts:

- [`/artifacts/release/ring_matrix.yaml`](../../artifacts/release/ring_matrix.yaml)
- [`/schemas/release/ring_history_packet.schema.json`](../../schemas/release/ring_history_packet.schema.json)
- [`/docs/release/qualification_cadence.md`](./qualification_cadence.md)
- [`/docs/release/shiproom_runbook.md`](./shiproom_runbook.md)
- [`/docs/release/release_evidence_packet_template.md`](./release_evidence_packet_template.md)
- [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
- [`/artifacts/release/evidence_ownership_map.yaml`](../../artifacts/release/evidence_ownership_map.yaml)
- [`/docs/release/install_topology_plan.md`](./install_topology_plan.md)
- [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
- [`/docs/release/channel_and_branch_contract.md`](./channel_and_branch_contract.md)
- [`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml)
- [`/artifacts/release/artifact_family_versioning.yaml`](../../artifacts/release/artifact_family_versioning.yaml)

Normative sources:

- `.t2/docs/Aureline_Milestones_Document.md` section 8.12 and section
  12.1.6
- `.t2/docs/Aureline_Technical_Design_Document.md` section 11.2.8
- `.t2/docs/Aureline_PRD.md` section 5.20
- `.t2/docs/Aureline_Technical_Architecture_Document.md` section 27.8
  through section 27.9

## Model

- Validation widening rings are not the same thing as the install
  topology rollout rings frozen in
  [`/docs/release/install_topology_plan.md`](./install_topology_plan.md)
  and
  [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml).
  Validation rings answer "how much evidence may justify a wider claim
  right now?"; rollout rings answer "where is the build deployed?".
- The five validation rings below refine the PRD's dogfood model into
  one release-control ladder. `broad_internal_dogfood` is an internal
  evidence lane between core-team canary and design-partner preview; it
  does not mint a public channel.
- `stable_candidate_or_ga` is a widening lane over stable-facing build
  sets. It does not introduce a new exact-build channel value; it is the
  final evidence lane before a stable or GA claim stays live.
- Lanes widen in order. A lane does not skip a ring unless shiproom
  explicitly narrows scope first and the ring-history packet captures why
  the skipped ring's evidence was not required for that narrower scope.
- Default reset rules define a reset floor. If the lane is already
  narrower than the floor, it holds its current ring until refreshed
  evidence lands; there is no automatic re-widening.

## Widening Contract

### Core-team canary

- Enter or widen only when owner coverage is current for the affected
  lane, crash or incident visibility is present, an explicit rollback
  path is named, and issue capture is exact-build aware.
- Minimum normal-workload soak: at least one core-team workday across
  the claimed matrix rows, not only synthetic smoke.
- Default rollback-stop condition: any crash, data-loss, trust, or
  protected-path regression on the widened lane.
- Hold or reset when rollback evidence is stale, crash visibility is
  missing, or a reset-trigger family is active without refreshed
  evidence.

### Broad internal dogfood

- Enter or widen only when the current protected-function snapshot is
  fresh, install or update viability is current for the widened profile,
  and internal docs or support wording for the wedge is current.
- Minimum normal-workload soak: at least one multi-team internal
  workweek across the claimed profiles.
- Default rollback-stop condition: repeated protected-path regression,
  update failure, or missing evidence refresh on a live internal lane.
- Hold or reset when install-topology truth, support-export truth, or
  current owner coverage drifts.

### Design-partner preview

- Enter or widen only when the current compatibility row is fresh, the
  support or export path is viable, known limits are published, and a
  named incident path exists for the partner lane.
- Minimum normal-workload soak: at least one design-partner workweek
  across the claimed archetype or deployment-profile rows.
- Default rollback-stop condition: partner-blocking trust, recovery, or
  compatibility failure.
- Hold or reset when partner repro data cannot be tied back to current
  claim rows, when mixed-version or rollback evidence ages out, or when
  public-truth narrowing has not caught up to the lane.

### Public preview or beta

- Enter or widen only when matrix rows are current, mixed-version and
  rollback drills are current where claimed, docs or migration or
  support language matches the current claim rows, and no unreviewed
  red-risk item remains on the widened lane.
- Minimum normal-workload soak: at least seven calendar days and one
  public-preview observation window with normal update or rollback usage
  where claimed.
- Default rollback-stop condition: stale packet on a live claim,
  unresolved high-severity regression, contradictory public truth, or a
  missing ring-history update.
- Hold or reset when any required packet is stale, when docs/help and
  support disagree about the widened claim, or when a reset-trigger
  family stays open.

### Stable candidate or GA

- Enter or widen only when ORR is green, rehearsals are current, the
  release-evidence packet is complete, the ring-history packet captures
  the exact evidence snapshot, and no release-bearing blocker remains on
  the widened profile or provider lane.
- Minimum normal-workload soak: at least fourteen calendar days and one
  full RC or release-center observation window across the widened
  profile or provider lane.
- Default rollback-stop condition: any failed rehearsal, stale matrix or
  claim row, missing ring-history evidence snapshot, or claim that
  outruns the current evidence.
- Hold or reset when shiproom is below `go`, when any required stable-
  facing proof is stale, or when a reset-trigger family remains active.

## Default Evidence-Reset Families

| Change family | Default reset floor | Evidence that must be refreshed before widening again | Canonical sources |
|---|---|---|---|
| `version_skew_behavior` | `design_partner_preview` | current compatibility row, version-skew register row, mixed-version and rollback rehearsal, updated claim rows and known-limit refs | `artifacts/compat/version_skew_register.yaml`, `docs/release/compatibility_report_template.md`, `artifacts/release/evidence_ownership_map.yaml` |
| `provider_mutation_authority` | `core_team_canary` | current approval-ticket contract, current provider-authority or browser-handoff proof, security posture review, updated claim rows and disclosure surfaces | `schemas/integration/approval_ticket.schema.json`, `docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`, `artifacts/release/promotion_gate_map.yaml` |
| `install_topology` | `broad_internal_dogfood` | install-topology and state-root rows, continuity-drill result, support-bundle install diagnostics, rollback-target recheck | `docs/release/install_topology_plan.md`, `artifacts/release/install_topology_matrix.yaml`, `artifacts/support/deployment_drill_catalog_seed.yaml` |
| `schema_migration` | `core_team_canary` | restore-provenance or migration-session evidence, rollback review, compare/export path, updated migration notes or known limits | `docs/state/migration_and_restore_playbook.md`, `schemas/state/restore_provenance.schema.json`, `schemas/migration/migration_session.schema.json` |
| `protected_dependency_posture` | `broad_internal_dogfood` | dependency-policy or package-inventory update, dependency-marker or claim parity refresh, support caveats, and any waiver closure | `docs/repo/dependency_rules.md`, `artifacts/governance/package_inventory.yaml`, `docs/adr/0011-capability-lifecycle-and-dependency-markers.md` |

Rules:

1. Reset applies to the affected lane, not automatically to every other
   lane in the train.
2. A reset trigger narrows evidence posture first and rollout narrative
   second. Release notes may summarize the reset, but they do not
   replace the evidence refresh.
3. Support, docs, and rollback owners may require a narrower reset floor
   than the default when the changed scope is broader than the default
   source set suggests.

## Ring-History Packet

The ring-history packet is the stable record of why a lane widened,
held, reset, or stopped for rollback.

- The packet shape is frozen in
  [`/schemas/release/ring_history_packet.schema.json`](../../schemas/release/ring_history_packet.schema.json).
- A public-preview, beta, or stable-facing lane should keep one current
  ring-history packet. Stable candidate or GA widening is
  non-conforming unless that packet is current.
- Every widening, hold, reset, or rollback-stop decision appends a
  transition row. The transition row keeps stable ids, not narrative
  release-note prose.
- Every widening or reset transition must preserve at least the
  `evidence_id` values, the affected `claim_row_refs`, the matching
  `release_evidence_packet_ref`, the observed soak summary, and any
  waiver or known-limit refs that narrowed the decision.
- Shiproom, ORR, support, and docs reviews may summarize the ring
  history, but they must point back to the same packet id and transition
  ids rather than re-state the story in a separate dialect.

## Operating Rules

- Ring occupancy is evidence, not marketing. A lane does not widen
  faster than release, docs, support, and rollback owners can explain.
- Stable-facing widening preserves the exact evidence snapshot that
  justified the decision. Free-text release notes are not enough.
- If the ring-history packet, release-evidence packet, claim rows, and
  known-limit refs disagree, the lane holds at the narrowest truthful
  posture until the discrepancy is corrected.
