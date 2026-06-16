# Add critical-upstream health registers, review-cadence reminders, red-risk escalation, and shiproom gating for unowned or degraded protected-path dependencies

This document is the human-readable companion to the canonical critical-upstream health register checked in at `artifacts/governance/m5-critical-upstream-health.json` and described by the schema at `schemas/governance/m5-critical-upstream-health.schema.json`. The typed consumer is `aureline_governance::m5_critical_upstream_health`.

## Purpose

The open/local-boundary and upstream-durability matrix (`artifacts/governance/m5-boundary-and-upstream-durability.json`) records, per asset lane, *whether* a critical upstream is owned as one coarse flag, and the import-provenance and fork-review register (`artifacts/governance/m5-import-provenance-and-fork-review.json`) records where each protected-path import came from. Neither makes each critical upstream — the third-party packages, protocols, and curated imports a protected M5 family leans on — inspectable as a durable health record rather than an ad hoc engineering note: how healthy its maintainer base is, what its security posture is, how fast it still ships, whether its review is on cadence, whether its license is clear, how feasible a replacement would be, who owns it, and — when it is red-risk or unowned — whether a sponsor/fork/replace plan is recorded and the shiproom has been told.

This register is that upstream-health layer. For every critical upstream a protected M5 family depends on it records one record that states, in one copy-safe record:

- the **maintainer health** — the rating (`maintainer.rating`), the active maintainer count, and the bus factor, so a critical upstream is never left to coast on an abandoned maintainer base;
- the **security posture** — open advisories and unpatched criticals (`security.posture`, `open_advisory_count`);
- the **update cadence** — whether the upstream still ships (`update_cadence.cadence`) and how long since the last release;
- the **review cadence** — a current review, a due-for-review reminder, an overdue review, or a missing one (`review_cadence.cadence_state`, `next_review_due`);
- the **license clarity** — clear, ambiguous, or incompatible (`license.clarity`, `spdx_license_id`);
- the **replacement feasibility** and **ownership** — how feasible a swap is (`contingency.replacement_feasibility`) and whether an owner is assigned (`ownership.ownership_state`, `owner_ref`), so an upstream is never left ownerless because it is "just infrastructure";
- the **sponsor/fork/replace contingency** — required for any red-risk upstream (`contingency.plan_state`, `disposition`);
- the **shiproom escalation** — required for any red-risk or unowned upstream (`escalation.escalation_state`, `required`).

The same health truth is published for packages, protocols, curated imports, and toolchain components — so a stalled protocol or an abandoned toolchain component cannot hide behind a healthy package. Every upstream kind is exercised by at least one record.

## The two anti-patterns the spec forbids

The register makes the two guardrails from the source documents impossible to ship silently:

- **Upstream fragility may not be treated as a mere engineering housekeeping issue when it affects a claimed row.** Each record carries a `scan_posture` (what the upstream-health scan found) and a `surface_posture` (what the governance-dashboard/promotion-packet surface shows). The two **must agree**, and an upstream-health gap on a subject still claiming a label at or above the cutline holds promotion through the shiproom stop rule recorded in `publication`. Every structural gap surfaces its reason, and the per-dimension control state is derived from the facts, so a control can never assert `satisfied` over a gap and a green upstream card can never mask an abandoned, unpatched, or unowned dependency.
- **Red-risk dependencies may not ship on protected paths without an approved sponsor, fork, or replacement plan.** A `red`- or `blocked`-grade upstream is required to record a sponsor/fork/replace contingency plan (`disposition` is one of `sponsor_upstream`, `maintain_fork`, or `replace_dependency`) and to raise a shiproom escalation; a red-risk or unowned upstream whose plan is pending (`contingency_plan_missing`) or whose escalation is pending (`shiproom_escalation_missing`) narrows on the ownership axis and holds promotion.

## Per-axis narrowing, never one global flag

A record narrows on the *specific* axis that thinned out, and the worst axis wins by precedence (ownership, then security, maintainer, cadence, license, and finally evidence-staleness):

- `narrowed_maintainer` — the maintainer base has thinned to a bus-factor risk or been abandoned (`maintainer_health_thinning`, `maintainer_abandoned`).
- `narrowed_security` — an advisory is open or a critical is unpatched (`security_advisories_open`, `security_unpatched_critical`).
- `narrowed_cadence` — the update cadence has stalled, or the upstream-health review is overdue or missing (`update_cadence_stalled`, `review_cadence_overdue`, `review_cadence_missing`).
- `narrowed_license` — the license is ambiguous or incompatible (`license_ambiguous`, `license_incompatible`).
- `narrowed_ownership` — the upstream is unowned, or a required contingency plan or shiproom escalation is missing (`upstream_unowned`, `contingency_plan_missing`, `shiproom_escalation_missing`).
- `narrowed_stale` — the proof packet, owner sign-off, or waiver thinned out (`health_proof_stale`, `health_proof_missing`, `owner_signoff_missing`, `waiver_expired`).

A **cleared** record has a healthy maintainer base, a clean security posture, an active cadence with a current review, a clear license, an assigned owner, any required contingency plan and escalation recorded, fresh proof, and an owner sign-off. A narrowed record drops its `effective_label` below the launch cutline and may never publish an effective label wider than the one it declares.

A slowing update cadence and a coming-due review are surfaced as **reminders** (`slowing`, `due_for_review`) without narrowing a still-healthy upstream — only a stalled cadence or an overdue/missing review is a gap.

Every narrowing reason is watched by a stop rule. An **inherited** narrowing — a subject whose declared label already sits below the cutline, or a gap held by an unexpired waiver — is gated upstream and does not itself hold promotion. An **upstream-health** failure on a subject whose declared label is still at or above the cutline holds promotion through a stop rule, recorded in `publication`.

## Consumption

Downstream Help/About, service-health, release-center, support-export, and shiproom surfaces should ingest `reuse_projection()` from the typed model rather than cloning status text, so every surface renders one source of truth — the projection carries the family, the upstream kind, the declared and effective labels, the support class, the risk grade, the health state, the scan/surface-agreement flag, the ownership/contingency/escalation posture, the active reasons, and the reuse surfaces for every record.

## Regeneration and proof

The artifact, the negative fixtures, the cases manifest, and the frozen validation capture are emitted by `tools/regenerate_m5_critical_upstream_health.py`, whose summary/parity/promotion logic mirrors the typed Rust consumer. Inline unit coverage lives in `crates/aureline-governance/src/m5_critical_upstream_health/tests.rs`; the protected gate is `crates/aureline-governance/tests/m5_critical_upstream_health.rs`, run by `.github/workflows/check_m5_critical_upstream_health.yml`, and it cross-checks the typed model against the frozen capture and proves the negative fixtures are rejected.
