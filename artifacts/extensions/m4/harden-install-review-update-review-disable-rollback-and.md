# Artifact: Harden install-review, update-review, disable/rollback, and revocation flows for extensions and policy packs

**Task:** Promote the lifecycle flows an ecosystem row passes through — install review, update review, disable/rollback, and revocation — into the stable line for both extensions and policy packs: show deterministic resolver output and the dependency tree, the effective permission inheritance, the optional integrations pulled in, the re-consent required on any effective-permission expansion, the exportable lock/install plans for team or air-gapped rollout, the disable/rollback posture, and the revocation/propagation posture; and derive the stability qualification with automatic narrowing below Stable.
**Status:** Implemented
**Verification class:** Conformance / interoperability suite + Security / privacy review + Docs validation + Release evidence review

## Summary

This lane binds, into one validated packet, the flow identity (subject extension/policy pack, flow class, pinned flow-contract version, publisher trust tier, lifecycle state), the deterministic resolver output (determinism class, install scope, resolution digest, and the resolved dependency tree), the effective permission inheritance (declared ∪ resolved-hard-dependency-transitive, with the optional-integration set surfaced separately and the expansion diff against the prior installed set), the re-consent requirement (raised whenever resolution expands the effective set, not only when the top-level manifest changes), the lock/install plan (with team and air-gapped rollout flags), the disable/rollback posture, and the revocation/propagation posture — and derives the stability qualification it may claim. A `stable` flow claim is only allowed when the row pins the published flow-contract version, is evidence-backed, keeps its publisher trust tier out of quarantine, carries a deterministic resolution with every hard dependency resolved, obtains re-consent on any effective-permission expansion, exposes an exportable lock/install plan, and satisfies its flow-specific posture (installable for install/update, reversible for disable/rollback, fully propagated for revocation). When any condition fails the visible tier is automatically narrowed below Stable (to `beta`, `preview`, or `withdrawn`) with machine-readable reasons. The checked-in packet is canonical: install review, update review, the disable/rollback panel, the revocation panel, the extension inspector, diagnostics, and support exports ingest it instead of inventing a generic "ready" badge.

## What changed

- New Rust module: `crates/aureline-extensions/src/harden_install_review_update_review_disable_rollback_and/mod.rs` (+ `tests.rs`)
- Re-exported from `crates/aureline-extensions/src/lib.rs`
- New schema: `schemas/extensions/stable_lifecycle_flow_hardening.schema.json`
- New fixtures: `fixtures/extensions/m4/harden-install-review-update-review-disable-rollback-and/`
  - `verified_publisher_public_install_stable.json`
  - `mirrored_update_reconsent_obtained_stable.json`
  - `offline_policy_pack_install_stable.json`
  - `permission_expansion_without_reconsent_narrows_to_preview.json`
  - `nondeterministic_resolution_narrows_to_preview.json`
  - `unresolved_hard_dependency_withdrawn.json`
  - `policy_pack_revocation_propagated_stable.json`
  - `rollback_irreversible_withdrawn.json`
  - `catalog_asserted_install_narrows_to_preview.json`
- New dump example: `crates/aureline-extensions/examples/dump_stable_lifecycle_flow_records.rs`
- New docs: `docs/extensions/m4/harden-install-review-update-review-disable-rollback-and.md`

## Acceptance criteria

- [x] The checked-in implementation, fixtures, and proof packet are current and self-describing (schema, fixtures, and docs reference one another) rather than ad hoc notes. (`stable_lifecycle_flow_hardening.schema.json`, fixtures dir, this packet)
- [x] Any surface still lacking stable qualification is automatically narrowed below Stable, with machine-readable reasons, instead of inheriting an adjacent green row. (`permission_expansion_without_reconsent_narrows_to_preview.json`, `nondeterministic_resolution_narrows_to_preview.json`, `unresolved_hard_dependency_withdrawn.json`, `rollback_irreversible_withdrawn.json`, `catalog_asserted_install_narrows_to_preview.json`)
- [x] Users and admins can inspect permissions (top-level and effective-after-resolution), compatibility/resolution posture, activation cost class (dependency tree + lock/export), lifecycle label, publisher provenance, and rollback/revocation state for the touched ecosystem row. (`stable_lifecycle_flow_inspection`, `stable_lifecycle_flow_effective_permission_inheritance`, `stable_lifecycle_flow_disable_rollback_posture`, `stable_lifecycle_flow_revocation_posture`)
- [x] Install/update review shows deterministic resolver output, dependency tree, effective permission inheritance, optional integrations pulled in, and lock/export artifacts for team or air-gapped rollout. (`stable_lifecycle_flow_deterministic_resolution`, `stable_lifecycle_flow_lock_export_plan`, `verified_publisher_public_install_stable.json`, `offline_policy_pack_install_stable.json`)
- [x] Re-consent is required whenever dependency resolution expands the effective permission set, not only when the top-level package manifest changes. (`reconsent.triggered_by_permission_expansion` re-derived from the prior-vs-effective diff; `mirrored_update_reconsent_obtained_stable.json`, `permission_expansion_without_reconsent_narrows_to_preview.json`)
- [x] Install-review fixtures prove deterministic dependency resolution, effective-permission expansion warnings, and exportable lock/install plans across public, mirrored, and offline installs. (`verified_publisher_public_install_stable.json` (public), `mirrored_update_reconsent_obtained_stable.json` (mirror), `offline_policy_pack_install_stable.json` (offline))

## Guardrails honored

- No ambient/implicit privilege expansion: an effective-permission expansion without obtained re-consent is flagged, narrows below Stable, and raises a banner — while still surfacing the expanded permission (`expansion_without_reconsent_narrows_but_still_surfaces_the_permission`). The effective set is re-derived from declared ∪ transitive at validation time, so a stored packet cannot hide a transitive permission (`stored_effective_permission_set_cannot_hide_a_transitive_permission`).
- No nondeterministic install truth: a nondeterministic or not-yet-run resolution narrows below Stable (`nondeterministic_resolution_narrows_to_preview.json`, `not_resolved_resolution_withdraws_the_flow`).
- No catalog-only trust: a `catalog_asserted_only` basis can never back a stable claim (`catalog_asserted_install_narrows_to_preview.json`, `no_catalog_only_stable_claim`).
- Unbounded/irreversible/unpropagated actions are surfaced and blocked: an unresolved or conflicting hard dependency, an irreversible rollback, or an unpropagated revocation withdraws the flow (`unresolved_hard_dependency_withdrawn.json`, `rollback_irreversible_withdrawn.json`, `revocation_not_propagated_withdraws_the_flow`).
- A narrower stable claim is published rather than papered over: the effective tier, downgrade flag, reasons, and banner are re-derived from the posture at validation time, so the packet cannot drift.

## How to verify

```bash
cargo test -p aureline-extensions harden_install_review
cargo run -q -p aureline-extensions --example dump_stable_lifecycle_flow_records -- validate
```

Materialized packets for every fixture validate against `schemas/extensions/stable_lifecycle_flow_hardening.schema.json` (checked with a Draft 2020-12 validator).

## Risks / follow-ups

- The packet models flow truth as evidence-bearing records; wiring it to live install/update/disable/rollback/revocation controllers (so the resolver, consent, and revocation refs point at real subsystems) lands as those controllers stabilize.
- The lock/install-plan and resolution-digest refs are opaque ids here; binding them to the concrete lockfile and digest formats is a follow-up once those formats are pinned on the stable line.
