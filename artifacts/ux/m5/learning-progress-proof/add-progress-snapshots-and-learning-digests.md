# Learning progress snapshots and digests — release evidence

Reviewer-facing evidence packet for the M5 learning-progress lane. A *progress
snapshot* is the durable, user-owned memory of how far a person got through one
learnability flow — its completed and dismissed steps, a resume point, an
explicit device/local sync policy, export refs, and a privacy disclosure that
keeps it local-first. A *learning digest* is the durable surface that exposes
resume, dismiss, snooze, reset, and export actions over those snapshots, so
feature-family onboarding never depends on an ephemeral banner. Progress never
becomes repo or collaborator telemetry merely because a tour or exercise exists.
A record that cannot prove that posture is explicitly narrowed below Stable with
a named reason rather than inheriting an adjacent green row.

Canonical machine sources (do not clone status text from this packet — ingest the JSON):

- Schema: [`/schemas/help/m5-learning-progress-snapshots.schema.json`](../../../../schemas/help/m5-learning-progress-snapshots.schema.json)
- Fixture: [`/fixtures/help/m5/learning-progress/m5_learning_progress_snapshots.json`](../../../../fixtures/help/m5/learning-progress/m5_learning_progress_snapshots.json)
- Public doc: [`/docs/m5/learning-digest-and-progress.md`](../../../../docs/m5/learning-digest-and-progress.md)
- Aligns with: [`/schemas/learning/m5-feature-family-learning-rails.schema.json`](../../../../schemas/learning/m5-feature-family-learning-rails.schema.json) (shared surface-family taxonomy) and [`/schemas/learning/guided-learning-contracts.schema.json`](../../../../schemas/learning/guided-learning-contracts.schema.json) (shared verdict vocabulary)
- Typed source: `aureline_learning::progress_snapshots`
- Headless emitter: `aureline_learning_m5_progress_snapshots`
- Test: `cargo test -p aureline-learning progress_snapshots`

## The snapshot matrix

| Snapshot | Family | Flow | Verdict | Disclosure | Sync policy | Completed / dismissed | Resume | Narrowing reason |
|---|---|---|---|---|---|---|---|---|
| `notebook_intro_tour` | notebook | tour | **qualified_stable** | local_only | local_only_default | 1/4 · 1 dismissed | yes | — |
| `request_workspace_first_call` | request_workspace | exercise_rail | **qualified_stable** | local_only | sync_blocked_by_policy | 2/3 · 0 dismissed | yes | — |
| `docs_browser_glossary` | docs_browser | glossary_walkthrough | **qualified_stable** | exported | local_only_default | 2/2 · 0 dismissed | no | — |
| `database_workspace_tour` | database_workspace | tour | **narrowed_beta** | sync_eligible | device_sync_eligible_disclosed | 1/3 · 0 dismissed | yes | device-sync-eligible state may lag (disclosed) |

## The digest matrix

| Digest | Covers | Verdict | Actions | Narrowing reason |
|---|---|---|---|---|
| `local_progress` | 3 snapshots | **qualified_stable** | resume, dismiss, snooze, reset, export | — |
| `synced_progress` | 1 snapshot | **narrowed_beta** | resume, dismiss, snooze, reset, export | covers a narrowed_beta snapshot |

**Overall manifest verdict: narrowed_beta** — the device-sync-eligible snapshot's
disclosed sync narrows it, the digest that covers it folds that in, and the
narrowest member propagates to the overall verdict; the three local/exported
snapshots ship Stable individually.

## What this packet proves

1. **Pause and resume never lose progress, and never leak into the repo.**
   `survives_restart` is true on every snapshot, and every `resume_point` sets
   `resumable_after_restart: true`, so a paused flow resumes after a restart.
   Progress is `user_owned_local_first`, never `repo_visible`, never
   `shared_with_collaborators`, and never readable at telemetry grade by an
   extension — the schema and validator both enforce it.

2. **Disclosure states are explicit and survive support/export review.** Each
   snapshot's `disclosure_state` is one of `local_only`, `sync_eligible`,
   `exported`, or `reset`, and the schema's `if/then` rules keep it consistent
   with the evidence: `sync_eligible` requires a disclosed sync-eligible policy,
   `exported` requires an export ref, and `reset` forbids retained progress. The
   round-trip test proves the disclosure state survives export and reopen
   unchanged.

3. **Onboarding no longer depends on ephemeral banners.** Every digest sets
   `replaces_ephemeral_banners: true` and `durable_recovery_available: true`,
   exposes the full action set (resume, dismiss, snooze, reset, export), and is
   visible in settings, Help/About, diagnostics, and support export
   (`hidden_in_transient_overlay_only: false`). The manifest enforces that **every
   snapshot is covered by at least one durable digest**, so no progress is
   stranded in an ephemeral-only state.

4. **Every action is command-backed, reversible, and non-mutating.** Each
   `digest_action` has a `command_id_ref`, a `keyboard_shortcut_ref`,
   `reversible: true`, `inspectable: true`, `silent_write_allowed: false`, and
   `mutates_workspace: false`. A digest only touches local progress state; it
   never writes to the workspace and never writes silently.

5. **Dismissals and exports are safe.** A dismissed step always sets
   `dismissal_reversible: true`. Every `export_ref` sets
   `redacts_raw_payloads: true` and `user_initiated: true`, so progress can be
   carried out without leaking workspace bodies and never silently.

6. **Experts are never trapped, and the command graph never moves.**
   `blocking_onboarding_allowed` is false and `command_graph_unchanged` is true
   on every snapshot, and `authority_boundary_change_allowed` is false. A flow
   that uses educational AI (`flow_uses_educational_ai: true`) MUST set
   `educational_ai_uses_standard_preview_approval: true` — the schema enforces it
   via `if/then` and the validator reports an unfenced do as a hard violation.

7. **Sync is honest.** Local-only progress is live-authoritative. A
   `device_sync_eligible_disclosed` snapshot MUST set `sync_disclosed: true`; an
   undisclosed sync-eligible snapshot is a masquerade that the validator narrows
   to Preview and the schema rejects outright. A disclosed sync is an honest,
   user-chosen deviation that narrows to Beta with a named reason.

## How the verdict is derived

`derive_snapshot_verdict` folds each snapshot's authority, command-graph,
ownership, privacy, blocking-onboarding, restart-survival, resume, educational-AI
fence, dismissal-reversibility, disclosure-consistency, export-safety, and
sync-disclosure evidence into the strictest verdict. Hard safety violations
narrow to `narrowed_preview`; a disclosed device-sync-eligible snapshot narrows to
`narrowed_beta`. `derive_digest_verdict` folds in each digest's action, durability,
and exposure evidence and the narrowest verdict of the snapshots it covers, so a
digest can never present a covered snapshot as healthier than it is. The
manifest's `overall_verdict` is the narrowest across all snapshots and digests.
Stored verdicts are re-derived and checked by
`validate_m5_learning_progress_snapshots`, so a hand-edited fixture that disagrees
with its own evidence fails validation.

## How to reproduce

```sh
cargo test -p aureline-learning progress_snapshots
cargo run -q -p aureline-learning --bin aureline_learning_m5_progress_snapshots -- validate
cargo run -q -p aureline-learning --bin aureline_learning_m5_progress_snapshots -- summary
```
