# Ship advisory templates, extension/provider revocation packets, emergency-disable bundles, mirror-propagation drills, and offline-import response evidence for M5 ecosystem/release lanes

This document is the human-readable companion to the canonical emergency-response evidence register checked in at `artifacts/governance/m5-emergency-response-evidence.json` and described by the schema at `schemas/governance/m5-emergency-response-evidence.schema.json`. The typed consumer is `aureline_governance::m5_emergency_response_evidence`.

## Purpose

The open/local-boundary and upstream-durability matrix (`artifacts/governance/m5-boundary-and-upstream-durability.json`) records, per asset lane, the emergency signing/registry/security authority, and the release-authority continuity register (`artifacts/governance/m5-release-authority-continuity.json`) makes each protected *authority lane* inspectable. Neither records the *emergency-response evidence* a protected M5 ecosystem/release lane produces when something goes wrong — the signed security advisory, the extension/provider revocation packet, the emergency-disable bundle, and the high-severity postmortem — nor whether that evidence actually reached the hosted, mirror, and offline customers that claim it, whether the action is attributable and reversible where policy allows, whether a break-glass action carried its audit markers and post-incident reconciliation, and whether the evidence is linked to the release artifact-graph and support exports rather than a side channel.

This register is that emergency-response evidence layer. For every protected M5 ecosystem/release lane and emergency packet it records one copy-safe record that states:

- the **packet template** — a signed advisory/revocation/disable/postmortem packet, bound and digested (`packet_template.template_state`, `packet_kind`, `signed`, `digest_ref`);
- the **distribution reach** — the hosted, mirror, and offline channels, each claimed and propagated (`distribution_reach.channels[].channel`, `claimed`, `state`), so a mirror or offline customer is never left on a hosted-only path — the headline guardrail;
- the **attribution** — the emergency action is attributable to an authorized actor (`attribution.attribution_state`, `actor_ref`, `authorization_ref`);
- the **reversibility** — a reversal runbook where policy permits reversal (`reversibility.reversibility_state`, `policy_reversible`, `reversal_runbook_ref`);
- the **audit trail** — audit markers and, for break-glass or high-severity actions, post-incident reconciliation (`audit_trail.audit_markers_present`, `reconciliation_state`, `mutation_journal_ref`), so a break-glass action never bypasses the audit and reconciliation rules;
- the **evidence linkage** — the release artifact-graph identity and support-export packet (`evidence_linkage.linkage_state`, `release_artifact_ref`, `support_export_ref`), not a side channel.

The same response truth is published for advisories, extension/provider revocations, emergency-disable bundles, and high-severity postmortems — so a revocation that never reached a mirror customer cannot hide behind a healthy advisory. Every packet kind is exercised by at least one record, and each record's `packet_template.packet_kind` must match the record's `packet_kind`.

## The anti-patterns the spec forbids

The register makes the two guardrails from the source documents impossible to ship silently:

- **A hosted-only advisory path may not stand in for a claimed mirror/offline customer.** Distribution reach is per-channel: a claimed mirror channel still propagating (`mirror_propagation_incomplete`), a claimed offline channel that never received the offline-import response (`offline_import_response_missing`), or any claimed channel whose evidence aged out (`channel_evidence_stale`) narrows on the distribution axis even when the hosted channel is current. Each record carries a `scan_posture` (what the response scan found) and a `surface_posture` (what the service-health/release-center/support surface shows). The two **must agree**, and every structural gap surfaces its reason, so a green emergency-response card can never mask a mirror/offline customer that never received the advisory, an unattributable break-glass action, or a side-channel-only disable.
- **Break-glass actions may not bypass audit markers, reversal rules, or post-incident reconciliation.** Audit markers (`audit_markers_present`), the reversal runbook (`reversibility_state` = `reversal_rule_missing`), and reconciliation (`reconciliation_state`) are first-class fields. A break-glass or high-severity action requires post-incident reconciliation; a break-glass action without audit markers narrows on the audit axis. A response gap on a subject still claiming a label at or above the cutline holds promotion through the stop rule recorded in `publication` — a protected lane whose advisory/revocation/disable evidence did not reach a claimed mirror/offline customer, or whose break-glass action bypassed audit/reconciliation, may not widen a stable claim without coverage.

## Per-axis narrowing, never one global flag

A record narrows on the *specific* axis that thinned out, and the worst axis wins by precedence (distribution reach, then the audit/break-glass axis, attribution, reversibility, linkage, the template, and finally evidence-staleness):

- `narrowed_template` — the signed packet template is not bound (`packet_template_unbound`).
- `narrowed_distribution` — a claimed hosted/mirror/offline channel did not receive current evidence (`mirror_propagation_incomplete`, `offline_import_response_missing`, `channel_evidence_stale`).
- `narrowed_attribution` — the emergency action has no attributable actor (`action_unattributable`).
- `narrowed_reversibility` — a reversible action lacks its reversal runbook (`reversal_rule_missing`).
- `narrowed_audit` — audit markers are missing or post-incident reconciliation is pending (`audit_markers_missing`, `reconciliation_pending`).
- `narrowed_linkage` — the evidence is linked only through a side channel (`evidence_linkage_missing`).
- `narrowed_stale` — the response proof packet, owner sign-off, or waiver thinned out (`response_proof_stale`, `response_proof_missing`, `owner_signoff_missing`, `waiver_expired`).

A **cleared** record has a bound template, every claimed channel propagated, an attributable action, a reversal runbook where policy permits reversal, audit markers (and reconciliation where required), evidence linked to release/support, fresh proof, and an owner sign-off. A narrowed record drops its `effective_label` below the launch cutline and may never publish an effective label wider than the one it declares.

A channel that is not claimed for a subject's customer profile (`not_claimed`) is not a gap — a subject with no offline customers is not forced to reach the offline channel. Post-incident reconciliation is required only for **break-glass** or **high-severity** (`high`/`critical`) actions; for routine moderate non-break-glass packets it is `not_required` and never narrows.

Every narrowing reason is watched by a stop rule. An **inherited** narrowing — a subject whose declared label already sits below the cutline, or a gap held by an unexpired waiver — is gated upstream and does not itself hold promotion. A **response** failure on a subject whose declared label is still at or above the cutline holds promotion through a stop rule, recorded in `publication`.

## Consumption

Downstream Help/About, service-health, release-center, support-export, and shiproom surfaces should ingest `reuse_projection()` from the typed model rather than cloning status text, so every surface renders one source of truth — the projection carries the family, the packet kind, the declared and effective labels, the support class, the severity, the response state, the scan/surface-agreement flag, the template/attribution/reversibility/reconciliation/linkage posture, the active reasons, and the reuse surfaces for every record.

## Regeneration and proof

The artifact, the negative fixtures, the cases manifest, and the frozen validation capture are emitted by `tools/regenerate_m5_emergency_response_evidence.py`, whose summary/parity/promotion logic mirrors the typed Rust consumer. Inline unit coverage lives in `crates/aureline-governance/src/m5_emergency_response_evidence/tests.rs`; the protected gate is `crates/aureline-governance/tests/m5_emergency_response_evidence.rs`, run by `.github/workflows/check_m5_emergency_response_evidence.yml`, and it cross-checks the typed model against the frozen capture and proves the negative fixtures are rejected.
