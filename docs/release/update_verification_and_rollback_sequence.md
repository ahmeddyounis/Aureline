# Update verification, rollback, and post-rollback evidence sequence

This document defines the **reviewable update journey** Aureline uses to
coordinate update discovery, verification, backup/checkpoint creation, restart,
rollback triggers, and post-rollback evidence capture.

The goal is that release engineering, support, enterprise reviewers, and UI
surfaces can all answer the same questions with the same stable identifiers:

- when the update was discovered and reviewed;
- when artifacts were resolved (official feed vs mirror vs offline bundle);
- when signature / digest / platform trust / provenance checks completed;
- when extension impact and helper/agent compatibility were reviewed;
- when the pre-restart checkpoint and backup became durable;
- when restart and post-restart health validation occurred;
- when rollback was triggered and completed; and
- when post-rollback evidence was captured for support and release proof.

This is a sequencing contract and evidence index, not an updater implementation.

## Companion artifacts

- [`/artifacts/release/update_rollback_sequence.yaml`](../../artifacts/release/update_rollback_sequence.yaml) —
  machine-readable checkpoint catalog and ordered variant paths.
- [`/fixtures/release/update_rollback_cases/`](../../fixtures/release/update_rollback_cases/) —
  worked cases that exercise the successful update path and failed-update with
  rollback path across the required variant set.

Related contracts this sequence composes with:

- [`/docs/release/update_ready_review_contract.md`](./update_ready_review_contract.md) —
  pre-apply review packet (verification posture, extension impact forecast,
  restart/rollback visibility).
- [`/docs/release/update_and_rollback_contract.md`](./update_and_rollback_contract.md) —
  update manifest and rollback reconstruction fields.
- [`/docs/release/mirror_integrity_and_offline_verification_contract.md`](./mirror_integrity_and_offline_verification_contract.md) —
  mirror / offline integrity and manual import receipt posture.
- [`/artifacts/release/install_update_about_truth_packet.md`](../../artifacts/release/install_update_about_truth_packet.md) —
  shared “About / update center / support bundle” install truth projection.
- [`/docs/governance/post_install_notice_and_provenance_contract.md`](../governance/post_install_notice_and_provenance_contract.md) —
  post-install provenance disclosure record that update and rollback surfaces
  cite after any install/import/mirror/rollback action.

## Scope

Frozen here:

- stable **checkpoint ids** that update center, restart banners, silent update
  summaries, support bundles, and release evidence can reuse verbatim;
- the required ordering and handoffs of update discovery → review → acquire →
  verify → checkpoint → restart → validate → commit/rollback → evidence; and
- the variant-specific deltas for online, offline/mirror, helper skew,
  side-by-side channel, and policy-constrained enterprise flows.

Out of scope:

- building the updater, package manager, or installer;
- defining platform-specific service managers or per-OS installer scripts; and
- authoring the full user-facing copy for every disclosure (this doc names the
  disclosure points and stable ids only).

## Stable checkpoint ids

The sequence packet defines a closed set of `checkpoint_id` values that are:

- **opaque and stable** (safe to log/export without embedding machine identity);
- **shared across surfaces** (UI, support exports, and release evidence refer to
  the same ids); and
- **ordered** (a sequence consumer can prove “backup happened before apply”).

Checkpoint ids represent *journey points*, not per-machine checkpoint storage
handles. Per-update concrete refs (update manifests, review packets, mirror
integrity packets, checkpoint storage refs, and support bundle ids) attach as
evidence to these checkpoints.

## Core journey (shared ordering)

The following checkpoints appear in order for every conforming update attempt:

1. `checkpoint.update.offer_discovered` — update discovery resolves a candidate
   update offer for the current channel/install posture.
2. `checkpoint.update.policy_evaluated` — policy gates are evaluated (pinned
   version, disallowed channel switch, approval required, offline-only, etc).
3. `checkpoint.update.review_emitted` — the update-ready review packet is
   emitted and rendered for user/admin review.
4. `checkpoint.update.user_review_completed` — a user/admin review decision is
   recorded before any irreversible step (download, apply, restart) proceeds.
5. `checkpoint.update.source_resolved` — the artifact source is resolved
   (official feed, approved mirror, customer mirror, offline bundle import).
6. `checkpoint.update.payload_acquired` — target artifacts are downloaded or
   imported and content-addressed.
7. `checkpoint.update.verification_completed` — digest/signature/platform trust
   and provenance checks complete with explicit “verified / missing / stale /
   blocked” posture.
8. `checkpoint.update.extension_impact_reviewed` — extension/package impact
   forecast is generated and disclosed before restart/apply.
9. `checkpoint.update.helper_negotiation_completed` — helper, sidecar, or remote
   agent version negotiation completes before mutation.
10. `checkpoint.update.pre_restart_checkpoint_created` — a durable
    backup/checkpoint is created *before* apply/restart.
11. `checkpoint.update.apply_staged` — update is staged/applied up to the
    restart boundary (restart-required class is explicit).
12. `checkpoint.update.restart_initiated` — restart begins under the staged
    update.
13. `checkpoint.update.post_restart_health_validated` — post-restart health
    validation confirms the target build is usable (or records failure).
14. `checkpoint.update.commit_or_rollback_decided` — decision point: commit the
    update (keep it) or trigger rollback.
15. One of:
    - `checkpoint.update.post_success_evidence_captured`, or
    - `checkpoint.update.rollback_triggered` → `checkpoint.update.rollback_completed`
      → `checkpoint.update.post_rollback_evidence_captured`.

No sequence may claim “rollback available” unless
`checkpoint.update.pre_restart_checkpoint_created` is satisfied and the
rollback target (or checkpoint restore path) is named in the emitted review.

## Variant deltas

The sequence packet includes ordered variants. Each variant:

- reuses the same checkpoint ids;
- adds required evidence bindings and disclosures; and
- names where the journey may block (policy, mirror integrity, helper skew).

Required variants:

- **Online (official feed)** — canonical path: discovery from the official feed,
  download, verification, checkpoint, restart, validate, commit or rollback.
- **Offline / mirror** — acquisition resolves via approved mirror or offline
  bundle import; mirror integrity and manual-import receipts become required
  evidence before staging apply.
- **Helper / version-skew** — helper/sidecar/remote-agent negotiation can block
  before apply; the sequence forces “negotiate before mutation”.
- **Side-by-side channel** — acquisition and checkpointing respect channel state
  root isolation and side-by-side admission rules; disclosures must state which
  install/profile is being updated and which is untouched.
- **Policy-constrained enterprise** — policy can require admin approval, offline
  acquisition, pinning, deferral, or block; the review packet must surface the
  controlling policy ref before “Update” is offered.

## Rollback triggers and post-rollback evidence

Rollback triggers are explicit and attributable:

- automatic trigger (post-restart health validation fails, crash loop detected,
  helper negotiation invalidated, or rollback is required by policy); or
- user/admin trigger (manual rollback action chosen from an explicit rollback
  affordance).

Post-rollback evidence capture MUST:

- name which update offer was attempted;
- name which checkpoint/backup/last-known-good target was restored;
- capture the rollback trigger and validation outcome; and
- remain exportable via the support bundle boundary without embedding raw
  secrets or private endpoints.

The packet in `artifacts/release/update_rollback_sequence.yaml` is the common
index: support exports and release evidence cite checkpoint ids from the same
catalog so “what happened when” is reconstructable without reading installer
code or relying on tribal knowledge.

