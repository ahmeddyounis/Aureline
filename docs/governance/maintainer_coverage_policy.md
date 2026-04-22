# Maintainer-coverage policy

This document is the repo-local operating policy for human coverage on
protected code, docs, schemas, and release/security artifact families.
It turns the architecture and design durability rules into one
checked-in review floor.

Companion artifacts:

- [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml)
  — machine-readable primary/backup-owner registry and active waivers.
- [`/docs/governance/dri_map.md`](./dri_map.md) — narrative ownership,
  escalation, and blocker-aging rules.
- [`/artifacts/governance/control_artifact_index.yaml`](../../artifacts/governance/control_artifact_index.yaml)
  — canonical control-asset register for the covered docs and artifacts.
- [`/artifacts/governance/signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml)
  — action matrix for protected release, policy, emergency, and
  security-sensitive approvals.
- [`/artifacts/governance/upstream_health_scorecard.yaml`](../../artifacts/governance/upstream_health_scorecard.yaml)
  — critical-upstream health scorecard keyed by dependency row id.
- [`/artifacts/governance/dependency_register.yaml`](../../artifacts/governance/dependency_register.yaml)
  — canonical dependency register whose critical rows must resolve into
  the upstream-health scorecard.
- [`/docs/release/release_evidence_packet_template.md`](../release/release_evidence_packet_template.md)
  — release packet template that cites maintainer coverage, quorum use,
  and any break-glass action.

Normative source anchors this policy projects from:

- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix AR
  and Appendix AS.
- `.t2/docs/Aureline_Technical_Design_Document.md` Section 10.5 and
  Appendix P.
- `.t2/docs/Aureline_Milestones_Document.md` Section 6.16.

## 1. Coverage principles

- No protected surface may rely on a single unreviewable human in the
  steady state.
- Human durability is a release-bearing control, not a courtesy. A
  path that lacks reviewer depth or a usable backup owner is not ready
  for beta/stable claims.
- Backup-owner waivers make coverage gaps visible; they do not turn
  those gaps into the normal operating model.
- Protected publish, freeze, revocation, disable, and kill-switch
  actions follow [`signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml),
  not ad hoc approval in chat or PR comments.
- Critical upstream rows on protected or release-critical paths require
  a companion row in
  [`upstream_health_scorecard.yaml`](../../artifacts/governance/upstream_health_scorecard.yaml).

## 2. Protected scope classes

This policy does not mint a second path inventory. The authoritative
path and lane sources remain the package inventory, ownership matrix,
and control-artifact index.

| Surface class | Canonical source | Steady-state named coverage floor | What counts as non-conforming |
|---|---|---|---|
| **Protected crate or subsystem** | `artifacts/governance/package_inventory.yaml` rows with `protected_path: true` plus the matching ownership-matrix package or lane row | named primary maintainer, named backup owner, and standing reviewer depth of at least two qualified humans | one named owner with no backup, no active waiver, or no second person capable of meaningful review |
| **Docs / public truth on a protected path** | `ownership_matrix.yaml#governance_lanes:docs_public_truth`, release docs, support docs, and claim-bearing templates named in the control-artifact index | named content owner, named backup owner, and a non-author reviewer who can check evidence-bearing claims | public truth that only one person can edit or verify |
| **Schema family** | protected contract families under `schemas/governance/`, `schemas/release/`, `schemas/security/`, and any public boundary schema named by the control-artifact index | named schema owner, named backup owner, and at least one downstream consumer reviewer in addition to the owner lane for semantic changes | schema change with only owner-lane eyes or no backup capable of owning migrations |
| **Repository-governance roots** | `CODEOWNERS`, ownership matrix, package inventory, control-artifact index, `CONTRIBUTING.md`, and `docs/repo/` truth surfaces | named governance owner, named backup owner, and two qualified reviewers for rule changes that affect protected routing or release authority | rules that can silently change who approves or owns protected work |
| **Release / policy / security-sensitive artifacts** | release packets, advisory/revocation records, emergency bundles, signing metadata, and the action rows in `signing_quorum.yaml` | named primary and backup operators, plus the quorum floor defined by `signing_quorum.yaml` for live actions | publish/freeze/revocation/disable authority resting with one human or an unaudited chat decision |

## 3. Standing reviewer depth versus per-change approval

Two different floors apply:

- **Standing reviewer depth** is the number of distinct humans who can
  review and take over a protected surface at all. The minimum is `2`
  for every protected surface in steady state: primary plus backup.
- **Per-change approval** is the number of distinct humans who must
  actively review a specific change or action.

Per-change approval floor:

| Change class | Minimum per-change approval | Extra rule |
|---|---|---|
| **Ordinary protected code, docs, or artifact change** | author plus one non-author qualified reviewer | the reviewer must be able to own follow-up on the affected lane |
| **Cross-lane or public-contract change** | author plus two non-author qualified reviewers | one reviewer must come from the non-author owning lane or downstream consumer lane affected by the change |
| **Schema semantic change** | owner-lane reviewer plus downstream-consumer reviewer | breaking or meaning-changing schema edits still require the matching decision-row / schema-version updates |
| **Claim-bearing docs or release packet update** | content author plus evidence owner or release/evidence reviewer | author-only claim publication is forbidden |
| **Live publish, freeze, rollback, revocation, disable, or kill-switch action** | the quorum named in `signing_quorum.yaml` | break-glass rules are the only permitted exception path |

## 4. Backup-owner rules

Backup owners are not placeholders. A valid backup owner for a
protected surface must:

- be a named human, not a team alias or queue;
- be able to review non-trivially, not only rubber-stamp;
- have the runbook access and context needed to absorb the lane during
  maintainer outage;
- know the relevant publication, waiver, and escalation path for the
  surface they back up; and
- participate in a shadow review, succession review, or continuity drill
  at least quarterly and before any beta/stable claim on that surface.

Additional expectations by surface:

- **Docs / public truth** backups must be able to update known limits,
  release disclosures, and contribution guidance without waiting on the
  primary owner.
- **Schema** backups must understand the current `schema_version`,
  compatibility promises, and the downstream consumers that would need
  migration notes.
- **Release / policy / security** backups must follow the same action
  ids and audit trail required by
  [`signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml).

## 5. Current single-maintainer posture

The repository currently carries the
`single-maintainer-backup` waiver recorded in
[`ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml).
That waiver exists to make the gap visible while the project is still
in pre-implementation.

What the waiver does allow:

- draft and seed artifacts to land while the project is still building
  out its governance surface; and
- explicit recording of the current fragility instead of silently
  pretending backup coverage exists.

What the waiver does **not** allow:

- beta, stable, or LTS claims on a protected path as though reviewer
  depth were already real;
- sole-human custody of release-signing, trust-root, or emergency
  disable authority as an acceptable steady state; or
- informal "whoever is awake" emergency approval outside the audited
  break-glass path in `signing_quorum.yaml`.

Any packet or decision that touches a claimed protected lane while the
waiver is open must cite the waiver explicitly and say whether the claim
is blocked, narrowed, or seed-only.

## 6. Required linkage in the repository

Changes that affect human durability or supply-chain fragility must
update the linked assets in the same change:

- **New protected lane, crate, doc family, or schema family:** update
  the ownership matrix, DRI map, and whichever canonical artifact row in
  the control-artifact index names that surface.
- **New critical upstream or criticality change:** update both the
  dependency register and the upstream-health scorecard.
- **Change to a protected publish, freeze, revocation, disable, or
  kill-switch path:** update `signing_quorum.yaml` and the affected
  release/security docs so the approval rule is still explicit.
- **Release-evidence or claim-bearing packet change:** keep the packet
  template wired to this policy and cite any active waiver,
  break-glass action, or high-risk upstream row.
- **Contribution-process change:** keep `CONTRIBUTING.md` aligned with
  this policy so contributors can see the reviewer-depth and backup
  rules before opening the pull request.

## 7. Escalation and correction rules

- Missing backup coverage on a protected surface is a correction signal,
  not an acceptable steady-state posture.
- Repeated renewal of the same backup-owner waiver must open a tracked
  correction plan; waiver renewal alone is not closure.
- If reviewer depth falls below the steady-state floor on a protected
  path, the next release or stable-claim packet must either narrow scope
  or block the claim until coverage is restored.
- If critical upstream risk rises and no backup maintainer can absorb a
  fork/replace path, the escalation goes through both the dependency
  register and the release/security decision forums rather than being
  left to repo folklore.
