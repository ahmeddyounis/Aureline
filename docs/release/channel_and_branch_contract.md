# Release channel, branch topology, artifact-family versioning, and side-by-side install contract

This document is the narrative companion to the channel-and-branch
contract Aureline freezes for its release lanes. It pins which
channels exist, which branch classes feed them, which release lanes
publish them, what versioning rules apply per artifact family, what
side-by-side installs look like, what downgrade posture each channel
carries, and how a failed update resolves into a named last-known-good
repair path.

Companion artifacts:

- [`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml)
  — machine-readable channel-and-branch matrix binding one
  `channel_row` per `channel_class`, the closed branch-posture
  register, the patch-and-backport lane register, the freeze-posture
  admission matrix, and the side-by-side admission matrix.
- [`/artifacts/release/artifact_family_versioning.yaml`](../../artifacts/release/artifact_family_versioning.yaml)
  — machine-readable per-artifact-family versioning matrix binding
  one `artifact_family_versioning_row` per shipped family
  (desktop shell, CLI, SDK, remote agent, managed service contract,
  extension API surface, extension SDK, marketplace metadata schema,
  admin policy bundle, docs / schema / reference packs, sidecars,
  supply-chain proofs, support runbook, release-evidence packet).
- [`/fixtures/release/channel_cases/`](../../fixtures/release/channel_cases)
  — seed `channel_case_record` fixtures for compatible side-by-side
  combinations, the explicitly-forbidden two-of-same-channel
  combination, portable coexistence with no machine-global mutation,
  downgrade posture exercised through admin or managed action,
  emergency freeze posture refusing promotion, and the last-known-good
  repair path after a failed update.

Cross-linked artifacts already in the repository:

- [`/docs/adr/0017-release-posture-artifact-families-and-promotion-gates.md`](../adr/0017-release-posture-artifact-families-and-promotion-gates.md)
  — governing release-posture ADR. The channel vocabulary, the
  rollback atom, the same-change-set bundles, the waiver / late-proof
  policy, and the advisory / revocation scope policy are owned there;
  this document references them, it does not re-decide them.
- [`/artifacts/release/artifact_family_map.yaml`](../../artifacts/release/artifact_family_map.yaml)
  and
  [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
  — release posture per artifact family and promotion-gate map
  composed by channel rows here.
- [`/artifacts/release/pipeline_lane_rules.yaml`](../../artifacts/release/pipeline_lane_rules.yaml)
  — pipeline lane rules (trust domains, credential boundaries,
  publishing rights). Every channel row binds one lane.
- [`/docs/release/install_topology_plan.md`](./install_topology_plan.md)
  and
  [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml)
  — install-topology and side-by-side / portable / managed posture.
  Every channel row references one or more install-profile cards.
- [`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml)
  — state-root, update-marker, recent-item, file-association, and
  protocol-handler ownership rows the side-by-side admission matrix
  cites by stable id.
- [`/docs/release/ring_progression_policy.md`](./ring_progression_policy.md)
  and
  [`/artifacts/release/ring_matrix.yaml`](../../artifacts/release/ring_matrix.yaml)
  — validation widening rings; orthogonal to the deployment-exposure
  ring vocabulary on install-profile cards. Channel rows admit
  rollout rings drawn from the install-topology matrix.
- [`/docs/compat/upgrade_order_contract.md`](../compat/upgrade_order_contract.md),
  [`/schemas/compat/mixed_version_envelope.schema.json`](../../schemas/compat/mixed_version_envelope.schema.json),
  and
  [`/artifacts/compat/skew_windows.yaml`](../../artifacts/compat/skew_windows.yaml)
  — mixed-version negotiation envelope, supported skew windows, and
  upgrade / rollback / downgrade order. Every artifact-family
  versioning row binds one skew window.
- [`/schemas/build/exact_build_identity.schema.json`](../../schemas/build/exact_build_identity.schema.json)
  — exact-build identity record. The channel vocabulary in this
  document mirrors the schema's `release_channel_class` values; no
  channel value is invented here.
- [`/docs/security/emergency_action_model.md`](../security/emergency_action_model.md)
  — authority for hard-freeze, cold-chain, and publication-paused
  postures. The freeze-posture admission matrix cites this model.

Normative sources this contract projects from:

- `.t2/docs/Aureline_PRD.md` §5.20 (release rhythm and channel
  discipline), §9.9 (mixed-version compatibility, negotiation, and
  upgrade posture), §9.12 (enterprise deployment hooks).
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §25.9
  (install, portable-mode, and fleet-rollout architecture), §26.5
  (distributed compatibility and version-skew policy), §27.8 through
  §27.9 (release widening and stable-facing claim movement),
  Appendix BA (platform installation matrix and fleet rollout rings).
- `.t2/docs/Aureline_Technical_Design_Document.md` §11.2.8 (release
  widening and skew evidence).
- `.t2/docs/Aureline_Milestones_Document.md` §6.18 (install and update
  behaviour as product truth), §8.12 (release widening and stable-
  facing claim movement), §12.1.6 (LTS and backport posture).

## Why publish this now

ADR 0017 froze the release posture, the artifact-family map, the
rollback atom, the same-change-set bundles, the waiver policy, the
advisory / revocation policy, and the promotion-gate map. The install-
topology plan and the state-root map froze the install profile, the
side-by-side rules, and per-channel ownership of state roots, update
markers, recent-item registration, file-association handlers, and
protocol-handler schemes. The mixed-version envelope and skew windows
froze the per-boundary capability negotiation contract.

What was still implicit was the **channel-and-branch contract** that
ties those pieces together at promotion time:

- which **branch classes** feed each channel and which postures those
  branches carry (release branches MUST be stabilisation lanes, not
  feature-invention lanes);
- which **patch / backport** lanes are admitted between channels and
  which change classes they may carry;
- what **versioning rule** applies per artifact family (semver, semver
  with capability negotiation, schema epoch, content-addressed,
  content-addressed with signed index, calendar-versioned LTS train,
  or coordinated-artifact-set versioned);
- what **side-by-side install** rules apply per channel pair, with
  handler and update-marker ownership pinned mechanically rather than
  through generic state-sharing language;
- what **downgrade posture** each channel carries, and how a failed
  update resolves into a named **last-known-good repair path**;
- and what **emergency freeze posture** is admissible per channel so a
  freeze does not silently widen.

Left implicit, every release lane would re-invent this vocabulary.
Stable and Preview would silently fight over state roots, update
markers, file associations, protocol handlers, or recent-work
registration. A backport could become a feature-invention vehicle
because the backport lane never declared its forbidden change classes.
A signed-bundle-only emergency would burn into Preview because the
freeze posture admitted no closed channel set. A failed update on
managed-fleet stable would have no agreed-upon repair path. Freezing
the contract now — before any release lane lands — ends those failure
modes.

This is a **pre-implementation plan**. No release lane, packager, or
fleet tooling is implemented at this revision. Every row in the
companion matrices is tagged `seeded` / `proposed`; rows are not
deleted, they are superseded by an ADR / RFC recorded in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).

## Scope

Frozen at this revision:

- One closed `channel_class_vocabulary` mirroring
  `schemas/build/exact_build_identity.schema.json`'s
  `release_channel_class` plus the two portable-mode rows the
  install-topology matrix admits. `dev_local` is included so the
  matrix can refuse promotion mechanically; it is not a publication
  channel.
- A closed `branch_class_vocabulary` and `branch_posture_register`
  pinning that **release branches are stabilisation lanes**, not
  feature-invention lanes.
- A closed patch / backport lane register pinning admissible and
  forbidden change classes per lane (so a hotfix backport cannot
  silently widen the marketed claim).
- A closed `freeze_posture_class_vocabulary` and freeze-posture
  admission matrix pinning which channels remain admissible under
  `normal_open`, `soft_freeze_release_lane_only`,
  `hard_freeze_security_only`, `cold_chain_mirror_only`, and
  `publication_paused`.
- A closed `side_by_side_admission_class_vocabulary` and side-by-side
  admission matrix pinning per-pair isolation rules for state roots,
  update markers, recent-item registration, file associations, and
  protocol handlers, plus the explicit `forbidden_two_of_same_channel`
  case so two installs of the same channel on one host are mechanically
  refused.
- A closed `downgrade_posture_class_vocabulary` and a
  `last_known_good_repair_path_class_vocabulary` so downgrade and
  failed-update recovery are never accidental capabilities.
- A closed `versioning_model_class_vocabulary` (semver, semver with
  capability negotiation, schema epoch with additive minor, content-
  addressed, content-addressed with signed index, calendar-versioned
  LTS train, coordinated-artifact-set versioned) with explicit
  semantics and one `artifact_family_versioning_row` per shipped
  family.

Out of scope until a superseding decision row opens:

- The release pipeline itself (packagers, signers, mirror publishers,
  fleet-console adapters). The channel rows reserve `pipeline_lane_ref`
  slots; the lane bytes are owned by
  [`/artifacts/release/pipeline_lane_rules.yaml`](../../artifacts/release/pipeline_lane_rules.yaml).
- Final signing, notarisation, or transparency-log workflows; the
  exact-build identity schema pins the signing-class and signing-
  material-state vocabulary every channel row reads.
- Marketplace metadata bytes, policy-bundle DSL, or extension-host
  binary bytes. The versioning matrix freezes the row shape and the
  versioning model; the bytes are later lanes.
- Cohort sizes, percentage rollouts, soak time values, or calendar
  cadence beyond the published support window. Those are
  benchmark-council / release-council authority recorded on
  decision rows.

## Channel-and-branch contract

### Channel vocabulary

Closed set. Mirrors `schemas/build/exact_build_identity.schema.json`'s
`release_channel_class` plus the two portable-mode rows from the
install-topology matrix.

| Channel | Branch class | Pipeline lane | Default audience | Narrowing semantics |
|---|---|---|---|---|
| `dev_local` | `dev_local_ref` | `contributor_pr_lane` | local builder only | non-promotable; no public claim |
| `nightly` | `nightly_train_ref` | `nightly_release_lane` | engineering plus QE / perf | no public support claim; promotes only into `preview` |
| `preview` | `preview_train_ref` | `preview_release_lane` | gated pre-release | renders the `pre_release_unverified` docs version-match state |
| `beta` | `beta_train_ref` | `beta_release_lane` | partner-facing pre-release | published workarounds, no hidden trust or rollback gap |
| `stable` | `stable_train_ref` | `stable_release_lane` | general availability | rolling support window |
| `lts` | `lts_train_ref` | `lts_release_lane` | enterprise long-term support | explicit calendar-bounded support window |
| `hotfix` | `hotfix_train_ref` | `emergency_hotfix_lane` | correction off stable / lts | single named correction scope; no widening |
| `portable_stable` | `portable_train_ref` | `stable_release_lane` | portable-mode stable | self-contained directory; no machine-global mutation |
| `portable_preview` | `portable_train_ref` | `preview_release_lane` | portable-mode preview | self-contained directory; no machine-global mutation |

`rc_candidate` is **not** a channel. It is a shiproom review-stage
label that an existing channel-bound build set may carry while moving
from `beta` → `stable`, from `stable` → `lts`, or through a `hotfix`
review. The `release_review_stage_register` in
[`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml)
pins which carrier channels admit `rc_candidate` and which forbid it.

### Branch posture: release branches are stabilisation lanes

The `branch_posture_register` pins one `branch_posture_class` per
branch class:

- `mainline_protected_ref` is the only branch class with
  `branch_posture_class` `feature_invention_protected_mainline`.
  Every feature lands here first; release-train branches take
  stabilisation snapshots from this ref. Release-train branches do
  not feed mainline.
- `nightly_train_ref`, `preview_train_ref`, `beta_train_ref`,
  `stable_train_ref`, `lts_train_ref`, `hotfix_train_ref`, and
  `portable_train_ref` all carry
  `stabilisation_no_feature_invention` (or
  `portable_packaging_stabilisation` for portable). New feature
  invention on a release train is non-conforming and opens a decision
  row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
- `patch_lane_ref` (forward-port) and `backport_lane_ref` (back-port)
  are explicit stabilisation lanes; the patch / backport lane register
  pins admissible change classes per lane:

| Lane | Direction | Source → Destination | Admissible change classes | Forbidden change classes |
|---|---|---|---|---|
| `patch_lane.preview_to_beta` | forward-port | `preview` → `beta` | bug fix, security fix, docs/help correction, migration-note clarification | new feature invention, public API widening, claim widening |
| `patch_lane.beta_to_stable` | forward-port | `beta` → `stable` | bug fix, security fix, docs/help correction, migration-note clarification | new feature invention, public API widening, claim widening |
| `backport_lane.stable_to_lts` | back-port | `stable` → `lts` | security fix, data-loss / trust correction, bug fix, docs/help correction | new feature invention, public API widening, schema widening, support-window widening |
| `backport_lane.stable_to_hotfix` | back-port | `stable` → `hotfix` | security fix, data-loss / trust correction | new feature invention, public API widening, claim widening |
| `backport_lane.lts_to_hotfix` | back-port | `lts` → `hotfix` | security fix, data-loss / trust correction | new feature invention, public API widening, support-window widening |

Each lane row also names the same-change-set bundles the change MUST
satisfy (typically `claim_docs_known_limit_bundle` plus, for hotfix
and LTS lanes, `advisory_and_revocation_bundle` and
`mirror_and_offline_bundle`).

### Forward and backward promotion

Forward promotion is the path a build takes from authoring channel to
the next more-seniority channel. Each `channel_row` carries an
explicit `forward_promotion_targets` set:

- `dev_local` → `none`
- `nightly` → `preview`
- `preview` → `beta`
- `beta` → `stable`
- `stable` → `lts`, `hotfix`
- `lts` → `hotfix`
- `hotfix` → `none`
- `portable_stable`, `portable_preview` → `none` (portable channels
  are packaging variants of their installed peer; they do not promote
  separately)

Backward promotion (downgrade) is governed by the
`downgrade_posture_class` on the `channel_row`:

| Channel | Downgrade posture | Last-known-good repair path |
|---|---|---|
| `dev_local` | `downgrade_unsupported` | `unsupported_no_in_product_repair` |
| `nightly` | `downgrade_supported_in_channel` | `in_channel_rollback_to_previous_build` |
| `preview` | `downgrade_best_effort_with_state_warning` | `in_channel_rollback_to_previous_build` |
| `beta` | `downgrade_best_effort_with_state_warning` | `in_channel_rollback_to_previous_build` |
| `stable` | `downgrade_supported_to_named_floor` | `in_channel_rollback_to_last_broad_cut` |
| `lts` | `downgrade_supported_to_lts_floor` | `rollback_to_lts_floor` |
| `hotfix` | `downgrade_requires_admin_or_managed_action` | `rollback_to_admin_pinned_target` |
| `portable_stable` | `downgrade_supported_in_channel` | `portable_extract_replace_self_contained` |
| `portable_preview` | `downgrade_supported_in_channel` | `portable_extract_replace_self_contained` |

Repair paths resolve mechanically: the `last_known_good_repair_path_class`
on the channel row is the named recovery flow a failed update must
land in. Free-text recovery prose on a release-evidence packet is
non-conforming.

### Emergency freeze posture

The `freeze_posture_admission_matrix` pins which channels remain
admissible under each freeze posture:

| Freeze posture | Authority | Channels admitted | Channels blocked |
|---|---|---|---|
| `normal_open` | shiproom | every channel | none |
| `soft_freeze_release_lane_only` | shiproom | every channel except `nightly` | `nightly` |
| `hard_freeze_security_only` | release council (per emergency-action model) | `dev_local`, `stable`, `lts`, `hotfix` | `nightly`, `preview`, `beta`, `portable_*` |
| `cold_chain_mirror_only` | release council | `dev_local`, `stable`, `lts`, `hotfix` (offline / mirror transports only) | `nightly`, `preview`, `beta`, `portable_*` |
| `publication_paused` | release council | `dev_local` only | every published channel |

A channel not listed under the active freeze posture's
`admitted_channel_refs` MUST refuse promotion. The matrix is the
authority; no release lane may negotiate a per-incident exception
without an emergency-action record.

## Artifact-family versioning rules

Each shipped artifact family resolves to one
`artifact_family_versioning_row` in
[`/artifacts/release/artifact_family_versioning.yaml`](../../artifacts/release/artifact_family_versioning.yaml)
binding it to:

- one `versioning_model_class` (semver, semver with capability
  negotiation, schema epoch with additive minor, content-addressed,
  content-addressed with signed index, calendar-versioned LTS train,
  coordinated-artifact-set versioned);
- one `compatibility_capability_source` (mixed-version envelope
  negotiation, schema-epoch admission table, signed-index capability
  table, coordinated-artifact-set no-negotiation, admin-policy pin);
- one `skew_window_declaration_ref` into
  [`/artifacts/compat/skew_windows.yaml`](../../artifacts/compat/skew_windows.yaml);
- one `downgrade_support_class` and one `backward_readability_class`;
- a `same_change_set_obligation` set drawn from the bundles in
  [`/artifacts/release/promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml);
- one `stability_promise_class` and one `deprecation_window_class`.

Per-family summary at this revision:

| Artifact family | Versioning model | Stability promise | Capability source | Downgrade |
|---|---|---|---|---|
| Desktop shell binary | semver with capability negotiation | stable public API with capability negotiation | mixed-version envelope | supported |
| CLI binary | semver with capability negotiation | stable public API with capability negotiation | mixed-version envelope | supported |
| SDK library | semver | stable public API with two-minor deprecation window | mixed-version envelope | best-effort |
| Remote agent | semver with capability negotiation | stable public API with capability negotiation | mixed-version envelope | supported |
| Managed service contract | schema epoch with additive minor | stable public API with capability negotiation | schema-epoch admission table | requires admin or managed action |
| Extension API surface | semver with capability negotiation | stable public API with capability negotiation | mixed-version envelope | best-effort |
| Extension SDK library | semver | stable public API with two-minor deprecation window | mixed-version envelope | best-effort |
| Marketplace metadata schema | content-addressed with signed index | stable public API with capability negotiation | signed-index capability table | supported |
| Admin policy bundle | content-addressed with signed index | admin-only contract | admin-policy pin | requires admin or managed action |
| Docs pack payload | semver | stable public API | schema-epoch admission table | supported |
| Schema export payload | schema epoch with additive minor | stable public API with capability negotiation | schema-epoch admission table | supported |
| Reference pack | semver | stable public API | schema-epoch admission table | supported |
| Debug symbols / source map / crash symbols | content-addressed | internal-only | coordinated artifact set | requires coordinated artifact set |
| SBOM document | content-addressed | stable public API | coordinated artifact set | requires coordinated artifact set |
| Signed attestation | content-addressed with signed index | stable public API | signed-index capability table | requires coordinated artifact set |
| Source bundle | content-addressed | stable public API with one-LTS-window deprecation | coordinated artifact set | requires coordinated artifact set |
| Reproducibility pack | content-addressed | stable public API with one-LTS-window deprecation | coordinated artifact set | requires coordinated artifact set |
| Support runbook payload | coordinated-artifact-set versioned | admin-only contract | coordinated artifact set | requires coordinated artifact set |
| Release-evidence packet envelope | coordinated-artifact-set versioned | stable public API | coordinated artifact set | requires coordinated artifact set |

The matrix's `paired_artifact_family_refs` slot pins which families
move together. The marketplace metadata schema MUST stay aligned with
the extension API surface and the schema export payload; the admin
policy bundle MUST stay aligned with the managed service contract and
the extension API surface; the support runbook MUST move with the
release-evidence packet envelope. Mirrors and private registries MUST
preserve digests, signatures, compatibility metadata, and permission
manifests; weakening the signed index on the marketplace metadata
schema or the admin policy bundle is non-conforming per ADR 0012.

## Side-by-side install and state-sharing rules

Side-by-side installs are admissible only when every isolation row
holds. The
[`side_by_side_admission_matrix`](../../artifacts/release/channel_matrix.yaml)
pins one `pair_id` per admissible pair; each pair declares
`state_root_isolation_required`, `update_marker_isolation_required`,
`recent_item_registration_isolation_required`,
`file_association_isolation_required`, and
`protocol_handler_isolation_required` and cites the
[`state_root_map.yaml`](../../artifacts/release/state_root_map.yaml)
rows that implement the rule. Reviewers can resolve "may these two
channels coexist on this host?" in O(1) by reading the pair row.

### Compatible side-by-side combinations

- `pair.stable_and_preview` — default coexistence pair. Each channel
  keeps its own configuration root, recovery root, derived cache root,
  keychain entries, update marker (under the binary root), recent-item
  registration (channel-suffixed namespace), file-association
  candidacy (user- or admin-selectable; never last-writer-wins), and
  protocol-handler scheme (channel-suffixed, e.g.
  `aureline-stable://` and `aureline-preview://`).
- `pair.stable_and_beta` — same isolation rules as
  `pair.stable_and_preview`. Beta is reserved at this revision; the
  rule is pinned now so later beta-train work cannot inherit
  preview-channel state roots.
- `pair.stable_and_lts` — coexistence with channel-suffixed
  admin-policy and shared-data roots. The LTS update marker lives
  under the admin-policy root (because LTS rows MUST carry an admin or
  managed updater per the install-topology matrix); stable's lives
  under its binary root.
- `pair.preview_and_beta` — same isolation rules; pinned so later
  beta-train work cannot inherit preview state roots.
- `pair.installed_and_portable` — installed stable plus
  portable_stable. Portable mode MUST NOT register file associations,
  protocol handlers, or machine-global recent-item entries; the
  portable root is self-contained.
- `pair.three_channel_matrix` — stable + preview + portable_stable on
  one host. Composes the pairwise rules; no rule is relaxed when a
  third channel is added.
- `pair.managed_and_portable` — managed-fleet stable plus
  portable_stable. The managed install owns the admin-policy root;
  the portable install owns its self-contained directory.

### Forbidden combinations

- `pair.forbidden_two_stables` — two installs of the same channel on
  one host are non-conforming. The state-root map enforces this by
  pinning `owning_channels` to one channel per row; the matrix carries
  the explicit `forbidden_two_of_same_channel` admission class so the
  refusal is named rather than inferred.
- Any side-by-side combination that would cause stable to write a
  preview state root, or vice versa, is non-conforming. The
  `state_root_map.yaml`
  [`per_channel_separation_rules`](../../artifacts/release/state_root_map.yaml)
  block pins this as a schema-level rule.
- Any side-by-side combination that would cause one channel's
  installer to silently override another channel's default file
  association or protocol-handler scheme is non-conforming. Default
  handler selection is user- or admin-selectable; "last-writer-wins"
  is forbidden by schema.

### Handler and update-marker ownership pointers

Every `channel_row` carries a `handler_ownership_pointer_class` that
resolves to one row family in
[`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml):

| Channel | Update-marker ownership | File-association registration | Protocol-handler scheme |
|---|---|---|---|
| `stable` | per-channel under binary root | user- or admin-selectable candidate handler | `aureline-stable://` (per-channel suffixed) |
| `preview` | per-channel under binary root | user- or admin-selectable candidate handler | `aureline-preview://` (per-channel suffixed) |
| `beta` | per-channel under binary root | user- or admin-selectable candidate handler | `aureline-beta://` (per-channel suffixed) |
| `lts` | per-channel under admin-policy root | admin-only default handler | `aureline-lts://` (per-channel suffixed) |
| `hotfix` | per-channel under binary root | (inherits from the channel being patched) | (inherits from the channel being patched) |
| `portable_stable` | per-channel under portable root | not registered | not registered |
| `portable_preview` | per-channel under portable root | not registered | not registered |
| `external_package_manager` rows | external-package-manager-owned | (per host policy) | (per host policy) |

A shared `aureline://` URL scheme resolves to the user- or admin-
selected default channel; installers MUST NOT silently claim the
shared scheme. Deep links that arrive at a wrong-target channel
(for example a link to `aureline-preview://…` opened on a host where
only stable is installed) MUST fail with a typed result code from the
silent-deployment return-code family rather than redirect to the wrong
channel.

### Acceptance criteria mapped to evidence

The acceptance criteria from the spec map to the matrices and the
state-root map as follows:

- *Release branches are clearly stabilisation / patching lanes, not
  feature-invention lanes.* — Pinned by the
  `branch_posture_register` and the patch / backport lane register in
  [`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml).
- *Versioning and compatibility promises can be stated mechanically
  for each artifact family.* — Pinned by
  [`/artifacts/release/artifact_family_versioning.yaml`](../../artifacts/release/artifact_family_versioning.yaml)
  with one `artifact_family_versioning_row` per shipped family.
- *Side-by-side preview / stable installs can be reasoned about
  without guessing which state is shared or isolated.* — Pinned by
  the `side_by_side_admission_matrix` in
  [`/artifacts/release/channel_matrix.yaml`](../../artifacts/release/channel_matrix.yaml)
  with explicit citations into
  [`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml).
- *Side-by-side preview / stable behaviour includes handler and
  update-marker ownership, not only generic state-sharing language.* —
  Pinned by the `handler_ownership_pointer_class` slot on every
  `channel_row`, the explicit isolation columns on every
  `side_by_side_admission_matrix` pair, and the per-channel rows in
  the `update_markers`, `recent_item_registrations`,
  `file_associations`, and `protocol_handlers` blocks of
  [`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml).

## Linkage into other control artifacts

- **Release evidence.** Every release-evidence packet MUST cite the
  `channel_row` for the shipped channel, the
  `artifact_family_versioning_row` for every shipped artifact family,
  and the relevant `side_by_side_admission_matrix` pair when a
  side-by-side install is part of the claimed line.
- **Promotion gates.** The
  [`promotion_gate_map.yaml`](../../artifacts/release/promotion_gate_map.yaml)
  candidate stages compose with the channel rows here; a candidate
  stage admitted on a `channel_row` MUST resolve to one of the
  candidate stage families on the gate map.
- **Pipeline lanes.** Every `channel_row.pipeline_lane_ref` resolves
  to a row in
  [`/artifacts/release/pipeline_lane_rules.yaml`](../../artifacts/release/pipeline_lane_rules.yaml).
  The lane rules pin trust domain, credential boundary, publishing
  rights, and the same-change-set re-materialization requirement; the
  channel matrix pins which channel that lane publishes onto.
- **Mixed-version envelope.** Every
  `artifact_family_versioning_row.skew_window_declaration_ref`
  resolves to a row in
  [`/artifacts/compat/skew_windows.yaml`](../../artifacts/compat/skew_windows.yaml).
  Compatibility reports and qualification packets read the same
  envelope; no per-family compatibility identity is invented.
- **Install-topology.** Every `channel_row.install_topology_card_refs`
  entry resolves to a card in
  [`/artifacts/release/install_topology_matrix.yaml`](../../artifacts/release/install_topology_matrix.yaml).
  Side-by-side installs read both the install-topology matrix (per-
  card behaviour) and the channel matrix (per-pair admission rules).
- **State-root map.** Every
  `channel_row.state_root_map_owning_channel_refs` entry resolves to a
  row in
  [`/artifacts/release/state_root_map.yaml`](../../artifacts/release/state_root_map.yaml).
  Side-by-side admission rows cite the same refs so handler /
  marker / state isolation is verifiable in O(1).
- **Emergency action.** The freeze posture admission matrix cites
  [`/docs/security/emergency_action_model.md`](../security/emergency_action_model.md)
  as authority for `hard_freeze_security_only`,
  `cold_chain_mirror_only`, and `publication_paused`.
- **Validation rings.** The
  [`ring_progression_policy.md`](./ring_progression_policy.md)
  validation widening ladder is orthogonal: validation rings widen
  evidence; the channel matrix governs publication. A release-
  evidence packet may cite both, but tooling must not substitute one
  for the other.

## Failure modes prevented

1. *Stable and Preview silently fight over a state root, update
   marker, file association, protocol-handler scheme, or recent-work
   registration.* — Refused at the `state_root_map.yaml`
   `per_channel_separation_rules` schema boundary; cited explicitly on
   every `side_by_side_admission_matrix` pair.
2. *Two installs of the same channel on one host.* — Refused by
   `pair.forbidden_two_stables` and the matching state-root-map rule
   that pins `owning_channels` to one channel per row.
3. *A backport silently widens the marketed claim or invents a new
   public API.* — Refused by the `forbidden_change_classes` set on
   every patch / backport lane row.
4. *Feature invention on a release-train branch.* — Refused by
   `branch_posture_register`'s `feature_invention_admitted: false`
   on every release-train ref.
5. *A failed update with no agreed-upon recovery flow.* — Refused by
   the `last_known_good_repair_path_class` slot on every
   `channel_row`; the recovery flow is named, not narrated.
6. *A Preview build promoted during a hard freeze.* — Refused by
   `freeze_posture_admission_matrix`'s `blocked_channel_refs` for the
   active posture.
7. *Marketing the marketplace metadata schema or the admin policy
   bundle without a signed index.* — Refused by the
   `marketplace_metadata_signed_index` and
   `admin_policy_no_silent_widening` invariants on
   [`/artifacts/release/artifact_family_versioning.yaml`](../../artifacts/release/artifact_family_versioning.yaml).
8. *A managed service contract changing its wire shape without a
   schema-epoch bump or without admin-or-managed downgrade authority.* —
   Refused by the
   `versioning.managed_service_contract` row's
   `versioning_model_class: schema_epoch_with_additive_minor` and
   `downgrade_support_class: downgrade_requires_admin_or_managed_action`.
9. *A release-evidence packet that does not move with its paired
   support-runbook payload.* — Refused by the
   `paired_artifact_family_refs` slot on
   `versioning.release_evidence_packet_envelope` and
   `versioning.support_runbook_payload`.
10. *A portable install registering a file association, protocol
    handler, or machine-global recent-item entry.* — Refused by
    `portable_no_machine_global_handlers` (channel matrix) and the
    `portable_no_machine_global_mutation` rule (install-topology
    matrix).

## Change control

- Adding a `channel_class`, `branch_class`, `branch_posture_class`,
  `versioning_model_class`, `compatibility_capability_source`,
  `downgrade_posture_class`, `downgrade_support_class`,
  `backward_readability_class`, `stability_promise_class`,
  `deprecation_window_class`, `freeze_posture_class`,
  `side_by_side_admission_class`, `handler_ownership_pointer_class`,
  `last_known_good_repair_path_class`, `coordinated_artifact_set_class`,
  or `identifier_form_class` value is additive-minor. Adding an entry
  requires bumping the matrix `schema_version`, extending the matrix
  with the new row, and updating this document in the same change.
- Repurposing an existing vocabulary value is breaking. Repurposing
  requires a new decision row in
  [`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and the concurrence of the release council.
- Adding a new patch or backport lane requires extending the
  patch-and-backport-lane register with a row that names admissible
  and forbidden change classes; admission of `new_feature_invention`
  on any release-train lane is non-conforming.
- Adding a new artifact family, or changing the versioning model on
  an existing family, requires the
  `exact_build_and_publication_bundle` and
  `claim_docs_known_limit_bundle` same-change-set bundles to update
  in the same change.

## Status

Contract is **seeded**. Every row in the channel matrix and the
artifact-family versioning matrix is tagged `seeded` / `proposed`.
Rows are not deleted; they are superseded by a follow-on ADR / RFC
recorded in
[`/artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml).
