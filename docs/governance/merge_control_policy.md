# Merge-control, branch-protection, and public-surface change policy

This policy defines the repository-local merge floor for protected
subsystems, launch-critical surfaces, public interfaces, and
release-bearing artifacts. It turns ownership and public-surface change
control into merge evidence rather than relying only on repository-host
defaults.

Companion artifacts:

- [`/artifacts/governance/protected_merge_classes.yaml`](../../artifacts/governance/protected_merge_classes.yaml)
  - machine-readable merge-control classes, approver roles, required
  packets, and minimum checks.
- [`/artifacts/governance/public_surface_change_controls.yaml`](../../artifacts/governance/public_surface_change_controls.yaml)
  - public-surface families and the compatibility, migration, ADR, docs,
  and release-note obligations they trigger.
- [`/artifacts/governance/branch_protection_seed.yaml`](../../artifacts/governance/branch_protection_seed.yaml)
  - branch, tag, freeze-window, bypass, and emergency reconstruction
  expectations.
- [`/CODEOWNERS`](../../CODEOWNERS),
  [`/artifacts/governance/ownership_matrix.yaml`](../../artifacts/governance/ownership_matrix.yaml),
  and
  [`/artifacts/governance/package_inventory.yaml`](../../artifacts/governance/package_inventory.yaml)
  - ownership and protected-path sources.
- [`/artifacts/governance/stable_surface_inventory.yaml`](../../artifacts/governance/stable_surface_inventory.yaml)
  and
  [`/artifacts/governance/compatibility_surfaces.yaml`](../../artifacts/governance/compatibility_surfaces.yaml)
  - stable and compatibility-bearing public-surface inventories.
- [`/artifacts/governance/mandatory_review_artifacts.yaml`](../../artifacts/governance/mandatory_review_artifacts.yaml)
  - packet classes and minimum contents for ADRs, RFCs, verification
  packets, compatibility reports, benchmark reports, and waivers.
- [`/artifacts/governance/signing_quorum.yaml`](../../artifacts/governance/signing_quorum.yaml)
  and [`/docs/security/high_risk_control_quorum.md`](../security/high_risk_control_quorum.md)
  - high-risk release, policy, freeze, revocation, and break-glass
  approval controls.

Normative source anchors this policy projects from:

- `.t2/docs/Aureline_PRD.md` repository rules for protected paths,
  branch and tag protections, migration notes, and compatibility impact.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` review
  workspace, provider branch-protection, ownership-signal, and public
  interface compatibility rules.
- `.t2/docs/Aureline_Technical_Design_Document.md` public interface
  governance, release publication, and emergency action rules.
- `.t2/docs/Aureline_Milestones_Document.md` public-interface
  versioning, maintainer coverage, release-line, and emergency-response
  planning rules.

If this policy disagrees with the source documents above, the source
documents win and this policy plus the companion YAML files update in
the same change.

## 1. Scope

This policy applies to merges into protected repository refs, release or
patch refs, public-surface files, protected-path code, and release-bearing
control artifacts.

It governs:

- ordinary code and docs changes;
- protected crates and protected governance lanes;
- launch-critical UI, architecture, and runtime contract surfaces;
- stable or beta public interfaces and compatibility-bearing surfaces;
- release, policy, emergency, signing, trust, and claim-bearing
  artifacts; and
- branch, tag, freeze-window, and emergency reconstruction rules.

It does not implement every repository-host ruleset or bot integration.
The companion artifacts are the source of truth future host-specific
rulesets must project from.

## 2. Class Resolution

Every change resolves to exactly one effective merge-control class for
gate purposes. When more than one class matches, use the strongest class
in this order:

1. `release_bearing_artifact_change`
2. `public_or_stable_interface_change`
3. `launch_critical_surface_change`
4. `protected_path_change`
5. `ordinary_change`

The YAML catalog defines the machine-readable version of this order. The
classification inputs are:

- `CODEOWNERS` for enforced owner routing;
- `package_inventory.yaml` for protected crates and production
  dependency posture;
- `ownership_matrix.yaml` for DRI, backup, waiver, and governance-lane
  ownership;
- subsystem contract cards for launch-critical subsystem ownership and
  evidence refs;
- stable and compatibility-surface inventories for public-interface
  posture; and
- release, security, signing, branch, and control-artifact rows for
  release-bearing authority.

If a change cannot be classified because a path is missing from those
sources, the change is treated as `protected_path_change` until the
missing owner or surface row lands. Missing classification is a merge
blocker for release-bearing refs.

## 3. Protection Tiers

| Merge-control class | Typical scope | Required approvers | Required gate artifacts | Minimum checks |
|---|---|---|---|---|
| `ordinary_change` | Unprotected docs, fixtures, prototypes, or local-only maintenance | CODEOWNERS-matched reviewer when the repository host requests one | Change description and ordinary review evidence | Build or contract checks that already apply to the touched path |
| `protected_path_change` | Protected crates, protected package edges, ownership, dependency rules, governance packets | Non-author owner-lane reviewer from CODEOWNERS or the ownership matrix; backup owner or active backup waiver must be visible | Protected dependency result, ownership or topology update when routing changes, waiver or exception packet when required | Contract-artifact validation, protected-dependency validation, relevant build or smoke lane |
| `launch_critical_surface_change` | Shell, editor, trust, accessibility, command, navigation, restore, remote, or architecture contract surfaces that affect launch-critical behavior | Protected-path approver plus affected subsystem owner; design or accessibility reviewer when the changed surface is user-visible | Subsystem contract-card linkage, design or verification packet when behavior changes, benchmark or accessibility evidence when the surface owns those bars | Protected-path checks plus frozen-surface validation when monitored paths are touched |
| `public_or_stable_interface_change` | Stable schemas, CLI JSON, WIT or SDK surfaces, service APIs, policy files, docs-pack schemas, support or evidence packets, public contract rows | Owner-lane reviewer plus compatibility reviewer; docs-public-truth reviewer when user-visible docs or claims change | Compatibility report, migration note when consumers must act, ADR or RFC for breaking or meaning-changing changes, public-truth update when quoted externally | Protected-path checks plus contract/schema drift validation and any surface-specific conformance check |
| `release_bearing_artifact_change` | Release packets, channel or branch contracts, signing or quorum controls, emergency action paths, claim manifests, release evidence, advisory or revocation records | Release/evidence owner plus the quorum profile required by `signing_quorum.yaml`; security reviewer for trust or emergency paths | Release evidence packet, quorum action id, emergency or break-glass audit row when used, rollback or reconstruction note when applicable | Release qualification, clean-room or provenance lane when relevant, contract validation, no stale proof for affected public claims |

The current repository may carry an active backup-owner waiver while it
is still a seed-stage repository. That waiver does not relax stable,
release, or public-claim readiness. A protected change can land as a
seed under the waiver, but the waiver must remain visible in the
affected packet or review evidence.

## 4. Required Evidence Before Merge

Protected changes do not become mergeable because a repository host says
checks are green. The reviewer must be able to see the evidence class
that makes the change reviewable:

- **Owner evidence:** CODEOWNERS match plus ownership-matrix owner or
  lane row. If the owner row lacks a backup, the active waiver must be
  cited.
- **Contract evidence:** subsystem card, stable-surface row,
  compatibility-surface row, schema-family row, or control-artifact row.
- **Packet evidence:** ADR, RFC, design packet, verification packet,
  compatibility report, benchmark report, waiver, exception packet, or
  release evidence packet as required by the class.
- **CI evidence:** status checks or locally produced reports that match
  the minimum check refs for the class.
- **Public-truth evidence:** docs, help, release notes, support
  exports, claim rows, migration notes, and known-limits updates when
  a public statement or consumer-facing behavior changes.

Missing evidence is a merge block on protected refs. On non-protected
draft refs it must be represented as a visible follow-up state, not a
green merge result.

## 5. Public-surface Changes

The public-surface control matrix is authoritative for surface families
and required artifacts. The default rules are:

- Stable schemas, CLI JSON, WIT worlds, SDK APIs, managed-service APIs,
  policy files, docs-pack schemas, and support or evidence packet
  schemas require a compatibility report for any consumer-visible
  semantic change.
- Breaking, removing, renaming, narrowing, or reinterpreting a public
  field requires an ADR or RFC link, migration guidance, and release or
  docs visibility before merge into a protected or release-bearing ref.
- Additive changes still require the relevant inventory row and docs
  touchpoint to remain current. Additive does not mean undocumented.
- Claim-bearing docs must bind to current evidence or explicitly narrow
  the claim. A docs-only edit can still be a public-surface change when
  it changes support, compatibility, release, security, or performance
  truth.
- Human-readable CLI output is not the stable automation contract; CLI
  JSON and schema versions are.

If a public-surface row is missing, the change must add the inventory row
in the same change or stay classified as protected and unmergeable for
release-bearing refs.

## 6. Branch And Tag Protection

The branch-protection seed defines host-neutral expectations for:

- `main` as protected mainline;
- release train refs;
- patch and backport refs;
- emergency or hotfix isolation refs; and
- release, provenance, advisory, and policy tags.

The common floor is:

- direct pushes to protected refs are forbidden for background agents,
  broad automation, and unreviewed local work;
- required status checks must pass or be represented by an approved,
  scoped waiver;
- required owner review must resolve through CODEOWNERS and the
  ownership matrix, not a chat-only approval;
- force pushes and tag moves are forbidden on release-bearing refs
  except through a signed revocation or supersedence path;
- bypass actors, when admitted, are named by role and control action,
  not by "admin can override"; and
- every bypass or emergency action records reconstruction obligations.

Release branches are stabilisation lanes. They may carry correction,
security, release-note, compatibility, and evidence changes. They do
not admit late feature invention unless a release council decision row
explicitly narrows or rebaselines the scope.

## 7. Emergency And Freeze Rules

Emergency repairs divide into two classes:

- **Ordinary emergency repair:** a bounded correction that restores the
  supported behavior without changing public compatibility, docs truth,
  release evidence, signing posture, or trust boundaries. It may use the
  patch or hotfix lane but still needs owner review and rerun evidence
  for affected protected paths.
- **Compatibility, docs, release, or trust-affecting emergency:** a
  correction that changes a public surface, support promise, advisory,
  release packet, policy bundle, channel state, revocation, or trust
  root. It must reopen the corresponding compatibility report, public
  truth, release evidence, quorum action, emergency-action record, or
  break-glass audit path.

Break-glass is containment only. It may freeze, disable, pause, revoke,
or publish a bounded emergency packet when the quorum cannot wait. It
may not promote stable or LTS, widen a claim, permanently change signer
rosters, or approve an ordinary protected-path change.

After any emergency bypass, the follow-up reconstruction must:

- bind the emergency action or break-glass row to the final reviewed
  change;
- land the missed owner review, compatibility report, migration note,
  release evidence, or public-truth update;
- back-merge or forward-port the correction into the appropriate
  protected line; and
- close with the retrospective quorum required by
  `signing_quorum.yaml`.

## 8. Linkage Rules

Merge-control artifacts do not replace existing sources. They compose
them:

- CODEOWNERS answers who the repository host can request.
- The ownership matrix answers who owns the lane, who backs them up, and
  which waiver is active.
- The package inventory answers whether a crate or package edge is
  protected.
- Subsystem contract cards answer which launch-critical contracts,
  budgets, failure modes, and proof packets are affected.
- Stable and compatibility-surface inventories answer which public
  surface row owns the contract.
- Mandatory review artifacts answer which packet class makes the change
  reviewable.
- Signing quorum answers which high-risk release or emergency action
  may proceed.
- Branch-protection seed answers which ref and tag rules must be
  projected into repository-host enforcement.

A future bot or ruleset should project from these files. It should not
invent a second vocabulary for protected changes, emergency bypass, or
public-surface obligations.

