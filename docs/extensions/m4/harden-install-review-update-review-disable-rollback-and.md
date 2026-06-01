# Harden install-review, update-review, disable/rollback, and revocation flows for extensions and policy packs

**Status:** Stable extension/policy-pack lifecycle-flow lane — implemented in `crates/aureline-extensions`.

## Goal

Promote the **lifecycle flows** an ecosystem row passes through — install review, update review, disable/rollback, and revocation — into the **stable line**, for both extensions and policy packs. Every claimed stable flow carries one canonical, checked-in flow truth: the deterministic resolver output and dependency tree, the effective permission inheritance (declared ∪ resolved-hard-dependency-transitive), the re-consent requirement raised on any effective-permission expansion, the lock/install plan for team or air-gapped rollout, the disable/rollback posture, and the revocation/propagation posture. The **stability qualification** that truth is allowed to claim is derived, not asserted: when the evidence can no longer back a `stable` flow claim, the visible tier is **automatically narrowed below Stable** (`beta`, `preview`, or `withdrawn`) with machine-readable reasons. Install review, update review, the disable/rollback panel, the revocation panel, the extension inspector, diagnostics, and support exports ingest this packet instead of inventing a generic "ready" badge.

## Design principles

1. **One vocabulary across all flows** — A single packet covers `install_review`, `update_review`, `disable`, `rollback`, and `revocation` for `extension` and `policy_pack` subjects, so install truth, runtime, mirror/offline import, disable/rollback, and revocation share one manifest, permission, lifecycle, and compatibility story.
2. **Deterministic resolver output before trust** — Each flow carries a `determinism_class` (`deterministic` / `nondeterministic` / `not_resolved`), an `install_scope_class` (`public_registry` / `approved_mirror` / `offline_bundle`), a resolution digest, and the resolved dependency tree (root, hard dependencies, optional integrations). A nondeterministic or not-yet-run resolution can never ride a stable install/update claim.
3. **Effective permission inheritance** — The effective set is `declared ∪ resolved-hard-dependency-transitive`, re-derived from its inputs at validation time so a stored packet can never hide a transitive permission. Optional-integration permissions are surfaced **separately** (what enabling them would add) and never folded in.
4. **Re-consent on expansion, not only on manifest change** — The effective set is diffed against the **prior installed** effective set. Whenever resolution **expands** it, re-consent is required (`triggered_by_permission_expansion`), independent of whether the top-level package manifest changed. An expansion without obtained re-consent narrows below Stable and raises a banner — while the expanded permissions stay surfaced.
5. **Exportable lock/install plans** — Each flow binds a lockfile ref and an install-plan ref with `supports_team_rollout` / `supports_air_gapped_rollout` flags, so a mirrored or offline rollout is reproducible. An unavailable plan narrows below Stable.
6. **Disable/rollback and revocation are first-class** — Disable/rollback carries a `rollback_state_class` (reversible target-pinned / reversible no-target / irreversible) and a rollback manifest ref; revocation carries a `revocation_state_class` and a `propagation_class` across the primary registry, approved mirrors, and offline bundles. An irreversible rollback or an unpropagated revocation withdraws the flow.
7. **Downgraded-flow banner** — A nondeterministic/unresolved resolution, an unresolved or conflicting hard dependency, an unconsented expansion, a missing lock/export plan, an irreversible rollback, an unpropagated revocation, or a quarantined trust tier raises a banner that names the reason a reviewer must see before the flow can be trusted.

## Record kinds

| Record kind | Purpose |
|---|---|
| `stable_lifecycle_flow_hardening_packet` | Top-level packet consumed by install review, update review, the disable/rollback panel, the revocation panel, the extension inspector, diagnostics, support export, docs/help, release packets, the CLI inspector, and mirror packets. |
| `stable_lifecycle_flow_identity` | Subject (extension/policy pack), flow class, pinned flow-contract version, source review/package refs, publisher trust tier, lifecycle state. |
| `stable_lifecycle_flow_deterministic_resolution` | Determinism class, install scope, resolution digest, resolver input, dependency-tree nodes, and derived node/hard/optional/unresolved/conflict counts. |
| `stable_lifecycle_flow_dependency_node` | Per-node kind (root / hard_dependency / optional_integration), target, version range, resolution state, contributed permissions. |
| `stable_lifecycle_flow_effective_permission_inheritance` | Declared / transitive / effective / optional / prior refs, the derived expansion diff, and the expansion class. |
| `stable_lifecycle_flow_reconsent_requirement` | Re-consent state, the permission-expansion and manifest-change triggers, and the consent record ref. |
| `stable_lifecycle_flow_lock_export_plan` | Lock/export state, lockfile and install-plan refs, team and air-gapped rollout flags. |
| `stable_lifecycle_flow_disable_rollback_posture` | Rollback state, target and manifest refs, disable reversibility, data retention. |
| `stable_lifecycle_flow_revocation_posture` | Revocation state, propagation across primary/mirror/offline sources, revocation and recovery-guidance refs. |
| `stable_lifecycle_flow_qualification_claim` | Claimed tier, effective tier after the posture is applied, support claim, narrowing reasons. |
| `stable_lifecycle_flow_downgraded_banner` | Whether a flow-review banner must display and why. |
| `stable_lifecycle_flow_inspection` | Compact boolean/count projection for CLI and inspector surfaces. |
| `stable_lifecycle_flow_hardening_support_export` | Metadata-safe support/partner/mirror export row. |

## Closed vocabularies

### Subjects
`extension`, `policy_pack`

### Flows
`install_review`, `update_review`, `disable`, `rollback`, `revocation`

### Install scopes
`public_registry`, `approved_mirror`, `offline_bundle`

### Resolver determinism
`deterministic`, `nondeterministic`, `not_resolved`

### Re-consent states
`not_required`, `required_obtained`, `required_pending`, `required_missing`

### Rollback states
`not_applicable`, `reversible_target_pinned`, `reversible_no_target`, `irreversible`

### Revocation states / propagation
`none`, `advisory`, `emergency_disabled`, `quarantined`, `revoked` / `not_applicable`, `propagated_all_sources`, `partial`, `not_propagated`

### Stability tiers / claim basis
`stable`, `beta`, `preview`, `withdrawn` / `evidence_backed`, `catalog_asserted_only`

## Automatic narrowing

A `stable` claim narrows to:

- **`withdrawn`** — `lifecycle_not_installable` (install/update), `resolver_not_resolved`, `unresolved_hard_dependency`, `version_conflict_dependency`, `rollback_irreversible`, `revocation_not_propagated`, `revocation_state_missing`.
- **`preview`** — `flow_contract_version_not_published`, `catalog_only_trust_not_evidence_backed`, `trust_tier_quarantined`, `resolver_nondeterministic`, `permission_expansion_without_reconsent`, `lock_export_unavailable`, `rollback_target_missing`, `attribution_incomplete`.
- **`beta`** — `reconsent_pending`, `team_rollout_unsupported`, `air_gapped_rollout_unsupported`.

Several reasons are flow-aware: the installable/rollout checks apply only to install-shaped flows, the rollback checks only to disable/rollback, and the revocation checks only to the revocation flow.

## Schema and fixtures

- Schema: [`schemas/extensions/stable_lifecycle_flow_hardening.schema.json`](../../../schemas/extensions/stable_lifecycle_flow_hardening.schema.json)
- Fixtures: `fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/`
  - `verified_publisher_public_install_stable.json` — a verified extension installed from the public registry with a resolved hard dependency; holds Stable.
  - `mirrored_update_reconsent_obtained_stable.json` — an update from an approved mirror whose dependency resolution expands the effective set; re-consent is obtained, so it holds Stable.
  - `offline_policy_pack_install_stable.json` — a self-contained policy pack installed from an offline bundle; holds Stable.
  - `permission_expansion_without_reconsent_narrows_to_preview.json` — an update whose resolution expands the effective set with no re-consent; narrows to `preview` with a banner, expanded permission still surfaced.
  - `nondeterministic_resolution_narrows_to_preview.json` — an install over an unpinned, nondeterministic resolution; narrows to `preview` with a banner.
  - `unresolved_hard_dependency_withdrawn.json` — an install whose required driver cannot be resolved; `withdrawn`, banner raised, driver permission excluded.
  - `policy_pack_revocation_propagated_stable.json` — a policy pack revoked and propagated across all sources; holds Stable.
  - `rollback_irreversible_withdrawn.json` — a rollback that cannot be reversed; `withdrawn` with a banner.
  - `catalog_asserted_install_narrows_to_preview.json` — a stable claim on catalog assertion alone; narrows to `preview`.

## How to verify

```bash
cargo test -p aureline-extensions harden_install_review
cargo run -q -p aureline-extensions --example dump_stable_lifecycle_flow_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_lifecycle_flow_hardening.schema.json` (checked with a Draft 2020-12 validator).
