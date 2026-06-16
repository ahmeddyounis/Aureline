# Freezing the open/local boundary, repository compliance, third-party imports, and maintainer/signer durability

This document is the human-readable companion to the canonical boundary-and-upstream durability matrix checked in at `artifacts/governance/m5-boundary-and-upstream-durability.json` and described by the schema at `schemas/governance/m5-boundary-and-upstream-durability.schema.json`.

## Purpose

Aureline's local/open core must stay inspectable and materially useful, third-party and generated code must stay attributable, and the release/signing/registry/security lanes must never depend on one irreplaceable human. Those facts were previously spread across the open-versus-paid audit, the signing-quorum policy, the third-party-import manifest, and the critical-upstream scorecard. This matrix is the one place that **freezes** them per asset lane and narrows a lane the moment any of them thins out — so a compliance gap, an authority gap, or a continuity gap is visible before it becomes a shiproom surprise, not buried in a private runbook.

For every claimed asset lane it binds:

- the **asset-lane axis** — `core_desktop_client_platform`, `sdk_schema_contract`, `docs_migration_pack`, `marketplace_protocol`, `managed_service`, or `restricted_brand_asset` — to its open/local `boundary_posture` and `support_class`, with a `must_remain_open` flag marking the lanes whose ordinary local usefulness may never be blurred by commercial or managed value;
- the **review/control axis** — `compliance_controls`, one binding per applicable control dimension (contribution provenance, file-level licensing, third-party imports, generated-code attribution, SBOM/notices, signer coverage, registry emergency action, security response, critical-upstream ownership) — each carrying its own satisfied/unsatisfied/not-applicable `state`;
- the **emergency authority** — the primary and backup owners, the signer quorum (`required` vs `available` distinct humans), and the registry-emergency and security-response owners;
- the **continuity rules** — backup coverage, the single-point-of-failure posture, and the owned critical upstreams with their risk class and fork/replace plan;
- the proof packet that grounds the row, the optional waiver that holds a gap provisionally, and the owner sign-off.

## The open-baseline guardrail and the no-widening ceiling

The spine of the matrix is that managed value may never blur the must-remain-open baseline, and a narrowed lane may never publish wider than it declares:

- A `must_remain_open` lane may only carry an open-baseline posture (`open_local_core` or `open_local_with_managed_optional`). If it drifts to a managed or restricted posture, the lane must narrow on `boundary_baseline_violated` — the matrix refuses to record the drift silently.
- A row's `effective_label` may never rank above its `declared_label`.
- A row is **durable** only when every control is satisfied (or not applicable), the signer quorum is met, the authority owners are present, there is no unmitigated single point of failure, backup coverage is in place, the critical upstreams are owned, the proof packet is fresh, and the owner has signed. Anything short of that narrows the lane and names the reason.

## No single global flag

The matrix never collapses durability into one green/red flag. The `summary` keeps per-state counts (`state_durable`, `state_narrowed_boundary_drift`, `state_narrowed_compliance_gap`, `state_narrowed_authority_gap`, `state_narrowed_continuity_gap`, `state_narrowed_stale`, `state_withdrawn`) and per-axis counts (controls, packets, active reasons). Because each axis narrows independently, a stale signing-pipeline proof narrows that lane on the stale axis while the core, SDK, and docs lanes stay durable. A reviewer reads exactly which lane thinned out, on which axis.

## Automatic narrowing and the publication gate

Each `active_reasons` entry is drawn from a closed vocabulary and watched by a `rule`:

- An **inherited** narrowing — a lane whose `declared_label` already sits below the cutline (Beta/Preview/Withdrawn), or a gap held by an unexpired waiver — does not itself hold promotion. The open-versus-paid audit and the maintainer-coverage waiver already gate it upstream; the stop rules only watch labels at or above the cutline.
- A **durability-layer** failure — an open-baseline violation, an unsatisfied compliance control, an unmet signer quorum, a missing authority owner, a single point of failure, missing backup coverage, an unowned critical upstream, stale or missing proof, a missing sign-off, or a lapsed waiver — on a lane whose `declared_label` is still at or above the cutline narrows the lane and holds promotion through a `rule`, recorded in `publication.decision` with the firing `blocking_rule_ids` and the offending `blocking_row_ids`.

This is the acceptance behavior the source docs require: any marketed/support-class lane narrows automatically when boundary, compliance, authority, continuity, or proof evidence goes stale or missing, while inherited and waived narrowings stay gated upstream.

## Reuse

The matrix is the single source release packets, docs/boundary manifests, repository-compliance scans, and shiproom gates read instead of minting per-surface boundary copy. The typed consumer exposes `reuse_projection()` — a copy-safe per-lane view of the boundary posture, effective label, durability state, active reasons, and reuse destinations — so every consuming surface reconstructs the current posture from the same machine-readable source. Each row also lists its `reuse_destinations` explicitly.

## Canonical sources

- **Matrix JSON**: `artifacts/governance/m5-boundary-and-upstream-durability.json`
- **Schema**: `schemas/governance/m5-boundary-and-upstream-durability.schema.json`
- **Fixtures**: `fixtures/governance/m5-boundary-and-upstream-durability/`
- **Validation capture**: `artifacts/governance/captures/m5-boundary-and-upstream-durability_validation_capture.json`
- **Regenerator**: `tools/regenerate_m5_boundary_and_upstream_durability.py`
- **Typed consumer**: `crates/aureline-governance/src/m5_boundary_and_upstream_durability/mod.rs`
- **Open-versus-paid boundary audit**: `artifacts/release/open_paid_boundary_audit.json`
- **Signing/approval quorum policy**: `artifacts/governance/signing_quorum.yaml`
- **Third-party-import manifest**: `artifacts/governance/third_party_import_manifest.yaml`
- **Critical-upstream health scorecard**: `artifacts/governance/upstream_health_scorecard.yaml`
- **Maintainer-coverage policy**: `docs/governance/maintainer_coverage_policy.md`
