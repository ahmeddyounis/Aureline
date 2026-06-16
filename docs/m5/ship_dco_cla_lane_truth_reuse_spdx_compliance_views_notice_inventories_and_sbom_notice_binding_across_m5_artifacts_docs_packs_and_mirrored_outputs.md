# Ship DCO/CLA lane truth, REUSE/SPDX compliance views, notice inventories, and SBOM/notice binding across M5 artifacts, docs packs, and mirrored outputs

This document is the human-readable companion to the canonical repository-compliance and notice-binding register checked in at `artifacts/governance/m5-compliance-and-notice-binding.json` and described by the schema at `schemas/governance/m5-compliance-and-notice-binding.schema.json`. The typed consumer is `aureline_governance::m5_compliance_and_notice_binding`.

## Purpose

The open/local-boundary and upstream-durability matrix (`artifacts/governance/m5-boundary-and-upstream-durability.json`) records, per asset lane, *whether* a repository-compliance control is satisfied as one coarse satisfied/unsatisfied flag. It answers *is the lane durable right now?* — but it collapses contribution provenance, file-level licensing, and SBOM/notice hygiene into a single flag and does not, per claimed M5 artifact family, docs pack, and mirrored output, publish the inspectable compliance truth a contributor, admin, or procurement reviewer reads off the product.

This register is that compliance-truth layer. For every claimed M5 subject it records one record that states, in one copy-safe record:

- the **DCO/CLA contribution-provenance** lane truth — whether every contribution is signed off (`dco_state`), whether the contributor agreement is on file (`cla_state`), and how many commits still lack provenance;
- the **REUSE/SPDX file-level licensing** coverage — how many files carry SPDX/REUSE licensing, how many gaps are covered by a *documented* exception, and whether any exception is undocumented;
- the **notice inventory** state — whether the third-party notice inventory is `complete`, `partial`, or `missing`;
- the **SBOM/notice binding** — whether the SPDX primary SBOM is present, whether the CycloneDX export is available, and whether the SBOM is actually `bound` to the notice inventory;
- the **mirror/offline binding** — whether the compliance artifacts are mirrored and whether that mirror is fresh.

The same compliance truth is published for shipped artifact families, the docs packs that document them, and the mirrored/offline outputs that redistribute them — so a gap on a docs pack or a mirror cannot hide behind a clean artifact family. Every M5 family is covered by exactly one `artifact_family` record; docs packs and mirrored outputs are additional subjects joined to their family.

## The two anti-patterns the spec forbids

The register makes the two guardrails from the source documents impossible to ship silently:

- **A green SBOM may not imply broader clearance than was reviewed.** Each record carries a `scan_posture` (what the repository-compliance scan found) and a `surface_posture` (what the user/admin notice/SBOM surface shows). The two **must agree**, and a record whose notice inventory is only `partial` still narrows on the notice axis even when its SBOM is present and `bound`. A present SBOM is never sufficient on its own: the review register carries a present-but-`unbound` SBOM and narrows on the SBOM axis precisely because presence is not binding.
- **File-level licensing gaps and contribution-provenance gaps may not be hidden on promoted families.** Every structural gap surfaces its reason (a `dco_signoff_missing` or `licensing_coverage_incomplete` gap can never be present without its narrowing reason), and the per-dimension control state is derived from the facts, so a control can never assert `satisfied` over a gap.

## Per-axis narrowing, never one global flag

A record narrows on the *specific* axis that thinned out, and the worst axis wins by precedence:

- `narrowed_provenance` — a DCO sign-off is missing or a CLA is unresolved (`dco_signoff_missing`, `cla_unresolved`).
- `narrowed_licensing` — SPDX coverage is incomplete or a licensing exception is undocumented (`licensing_coverage_incomplete`, `license_exception_undocumented`).
- `narrowed_notice` — the notice inventory is partial or missing (`notice_inventory_partial`, `notice_inventory_missing`).
- `narrowed_sbom` — the SPDX primary is missing, the SBOM is unbound, or the required CycloneDX export is unavailable (`sbom_primary_missing`, `sbom_notice_binding_broken`, `cyclonedx_export_unavailable`).
- `narrowed_mirror` — the compliance mirror/offline pack is stale (`mirror_stale`).
- `narrowed_stale` — the proof packet, owner sign-off, or waiver thinned out (`compliance_proof_stale`, `compliance_proof_missing`, `owner_signoff_missing`, `waiver_expired`).

A **cleared** record is provenance-clean, licensing-complete, notice-complete, SBOM-present-and-bound, mirror-fresh, proof-fresh, and owner-signed. A narrowed record drops its `effective_label` below the launch cutline and may never publish an effective label wider than the one it declares.

Every narrowing reason is watched by a stop rule. An **inherited** narrowing — a subject whose declared label already sits below the cutline, or a gap held by an unexpired waiver — is gated upstream and does not itself hold promotion. A **compliance-layer** failure on a subject whose declared label is still at or above the cutline holds promotion through a stop rule, recorded in `publication`.

## Consumption

Downstream Help/About, service-health, release-center, support-export, and evaluation-pack surfaces should ingest `reuse_projection()` from the typed model rather than cloning status text, so every surface renders one source of truth — the projection carries the family, the scope kind, the declared and effective labels, the support class, the compliance state, the scan/surface-agreement flag, the SPDX-present and CycloneDX-available flags, the notice state, the active reasons, and the reuse surfaces for every record.

## Regeneration and proof

The artifact, the negative fixtures, the cases manifest, and the frozen validation capture are emitted by `tools/regenerate_m5_compliance_and_notice_binding.py`, whose summary/parity/promotion logic mirrors the typed Rust consumer. Inline unit coverage lives in `crates/aureline-governance/src/m5_compliance_and_notice_binding/tests.rs`; the protected gate is `crates/aureline-governance/tests/m5_compliance_and_notice_binding.rs`, run by `.github/workflows/check_m5_compliance_and_notice_binding.yml`, and it cross-checks the typed model against the frozen capture and proves the negative fixtures are rejected.
