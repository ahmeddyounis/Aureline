# Expose signer rosters, split authority, promotion quorum, registry moderation, and security-response owner/backup/runbook continuity across M5 release lanes

This document is the human-readable companion to the canonical release-authority continuity register checked in at `artifacts/governance/m5-release-authority-continuity.json` and described by the schema at `schemas/governance/m5-release-authority-continuity.schema.json`. The typed consumer is `aureline_governance::m5_release_authority_continuity`.

## Purpose

The open/local-boundary and upstream-durability matrix (`artifacts/governance/m5-boundary-and-upstream-durability.json`) records, per asset lane, the emergency signing/registry/security authority as one coarse block (a primary owner, a backup list, a signer quorum), and the critical-upstream health register (`artifacts/governance/m5-critical-upstream-health.json`) makes each critical *dependency* inspectable. Neither makes each protected *authority lane* — the release-signing, promotion-approval, registry-moderation, and security-response operations a protected M5 family runs — inspectable as a durable continuity record rather than an ad hoc on-call note: who the named primary owner is, whether a backup owner exists so the lane is not a single-person system, whether the roster meets its quorum, whether split (two-person) authority is enforced where required, whether a current backup runbook exists, and — when the lane is critical or already single-owner — whether the shiproom has been told.

This register is that authority-continuity layer. For every protected authority lane a protected M5 family runs it records one record that states, in one copy-safe record:

- the **owner coverage** — a named primary owner (`owner_coverage.owner_state`, `primary_owner_ref`), so a release/security lane is never quietly ownerless;
- the **backup coverage** — at least one named backup owner (`backup_coverage.backup_state`, `backup_owner_count`), so a protected lane is never a single-person system by accident — the headline guardrail;
- the **roster quorum** — the signer roster, promotion-approval quorum, moderation-operator roster, or security-responder roster against its threshold (`roster.roster_kind`, `quorum_state`, `required_quorum`, `available_members`);
- the **split authority** — two-person / split control enforced for critical lanes (`split_authority.split_state`, `required`, `distinct_authorities`);
- the **runbook coverage** — a current backup runbook, a due-for-review reminder, a stale runbook, or a missing one (`runbook.runbook_state`, `next_review_due`);
- the **shiproom escalation** — required for any critical or single-owner lane (`escalation.escalation_state`, `required`).

The same continuity truth is published for release signing, promotion approval, registry moderation, and security response — so a single-owner security-response lane cannot hide behind a healthy release-signing lane. Every authority lane is exercised by at least one record, and each lane's `roster.roster_kind` must match the lane.

## The two anti-patterns the spec forbids

The register makes the two guardrails from the source documents impossible to ship silently:

- **Authority concentration may not hide behind a generic owner field.** Owner coverage and backup coverage are separate axes: a lane with a named primary owner but no backup (`backup_state` = `single_owner`, `backup_owner_count` = 0) is an effectively single-person system and narrows on the backup axis even when the lane is otherwise healthy and escalated. Each record carries a `scan_posture` (what the continuity scan found) and a `surface_posture` (what the governance-dashboard/promotion-packet surface shows). The two **must agree**, and every structural gap surfaces its reason, so a green authority card can never mask a single-owner, under-quorum, or runbook-less lane.
- **Backup coverage may not exist only in tribal memory or one-off spreadsheets.** Backup owners, rosters, and runbooks are first-class fields bound to source registers (`source_contract_refs`), and a missing or stale backup runbook (`runbook_missing`, `runbook_stale`) narrows on the runbook axis. A continuity gap on a subject still claiming a label at or above the cutline holds promotion through the shiproom stop rule recorded in `publication` — a single-owner, under-quorum, or runbook-less protected lane may not widen a stable claim without coverage.

## Per-axis narrowing, never one global flag

A record narrows on the *specific* axis that thinned out, and the worst axis wins by precedence (backup, then owner, authority, quorum, runbook, and finally evidence-staleness):

- `narrowed_owner` — the lane has no named primary owner (`primary_owner_vacant`).
- `narrowed_backup` — the lane has no backup owner and is effectively single-person (`backup_owner_missing`).
- `narrowed_quorum` — the signer/approval/operator/responder roster is below its required quorum (`roster_quorum_below_threshold`).
- `narrowed_runbook` — the backup runbook is stale or missing (`runbook_stale`, `runbook_missing`).
- `narrowed_authority` — split authority is required but unmet, or the shiproom escalation is required but pending (`split_authority_unmet`, `shiproom_escalation_missing`).
- `narrowed_stale` — the continuity proof packet, owner sign-off, or waiver thinned out (`continuity_proof_stale`, `continuity_proof_missing`, `owner_signoff_missing`, `waiver_expired`).

A **cleared** record has a named primary and backup owner, a roster at quorum, a current runbook, enforced split authority where required, any required escalation raised, fresh proof, and an owner sign-off. A narrowed record drops its `effective_label` below the launch cutline and may never publish an effective label wider than the one it declares.

A coming-due runbook review is surfaced as a **reminder** (`due_for_review`) without narrowing a still-covered lane — only a stale or missing runbook is a gap. Split authority and the shiproom escalation are required only for **critical** (`critical`/`blocking`) or **single-owner** lanes; for routine and elevated lanes with backup coverage they are `not_required` and never narrow.

Every narrowing reason is watched by a stop rule. An **inherited** narrowing — a subject whose declared label already sits below the cutline, or a gap held by an unexpired waiver — is gated upstream and does not itself hold promotion. A **continuity** failure on a subject whose declared label is still at or above the cutline holds promotion through a stop rule, recorded in `publication`.

## Consumption

Downstream Help/About, service-health, release-center, support-export, and shiproom surfaces should ingest `reuse_projection()` from the typed model rather than cloning status text, so every surface renders one source of truth — the projection carries the family, the authority lane, the declared and effective labels, the support class, the criticality grade, the continuity state, the scan/surface-agreement flag, the owner/backup/quorum/runbook/escalation posture, the active reasons, and the reuse surfaces for every record.

## Regeneration and proof

The artifact, the negative fixtures, the cases manifest, and the frozen validation capture are emitted by `tools/regenerate_m5_release_authority_continuity.py`, whose summary/parity/promotion logic mirrors the typed Rust consumer. Inline unit coverage lives in `crates/aureline-governance/src/m5_release_authority_continuity/tests.rs`; the protected gate is `crates/aureline-governance/tests/m5_release_authority_continuity.rs`, run by `.github/workflows/check_m5_release_authority_continuity.yml`, and it cross-checks the typed model against the frozen capture and proves the negative fixtures are rejected.
