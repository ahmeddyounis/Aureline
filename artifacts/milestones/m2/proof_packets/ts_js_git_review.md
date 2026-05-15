# External alpha TS/JS Git and review proof packet

```yaml
packet_id: review_packet:alpha.ts_js_git_review.2026-05-15
artifact_index_ref: artifacts/milestones/m2/artifact_index.yaml
scoreboard_row_refs:
  - scoreboard_row:alpha_scope.ts_js_git_review
owner_dri: "@ahmeddyounis"
reviewer_refs:
  - "@alpha-review"
freshness_date: 2026-05-15
captured_at: 2026-05-15T08:08:00Z
stale_after: P14D
source_revision: git:3a11f1e5296ad033d3d82ed9cb47ebf115d7a7e7
trigger_revision: ts_js_git_review_contract_set@2026-05-15
exact_build_identity_ref: artifacts/build/build_identity.json
channel_context: preview
deployment_context:
  - individual_local
claim_change_state: no_claim_widening
same_change_truth_refs:
  docs_ref: docs/milestones/m2_alpha_scope.md
  migration_ref: docs/migration/source_ecosystem_coverage_matrix.md
  help_truth_ref: docs/docs/help_about_service_health_routes.md
  known_limits_ref: artifacts/feedback/external_alpha_known_limits.md
  support_export_ref: docs/support/support_bundle_contract.md
```

This packet registers the current proof root for the external alpha TS/JS Git
and review basics floor. It closes the blocked packet state for clone/open
admission, branch preview/apply, local diff and change-list review, staging and
discard mutations, conflict handoff, local commit, local review-workspace
anchors, and structured Git/review activity support export.

## Canonical Artifacts

- Scoreboard row: `scoreboard_row:alpha_scope.ts_js_git_review`
- Persona rows: `artifacts/product/p0_persona_rows.yaml`
- Reference workspace seed: `fixtures/workspaces/reference/ts_web_app_archetype_seed.json`
- Launch bundle: `artifacts/bundles/tsjs_launch_bundle_alpha.yaml`
- Review packet template: `docs/review/m2_review_packet_template.md`
- Latest capture: `artifacts/milestones/m2/captures/ts_js_git_review_validation_capture.json`
- Exact build identity: `artifacts/build/build_identity.json`

## Required Outcome

`accept_narrowed`

Evidence is fresh and scoped to the individual-local preview channel. The row
is green because the TS/JS wedge can prove the local daily Git loop: clone/open
review admission, local branch changes, diff inspection, stage/unstage/discard
review, local commit, review-workspace anchoring, and support-export projection.

The row remains deliberately narrowed. It does not claim hosted review parity,
pull-request mutation parity, merge-queue support, provider comments, provider
approvals, hosted check status, or publish-now parity. Provider-facing behavior
is limited to the already declared publish-later, explicit publish review,
failed-publish recovery, provider-overlay, and browser-handoff postures.

## Evidence Rows

| Evidence | Value |
|---|---|
| Owning packet | `artifacts/milestones/m2/proof_packets/ts_js_git_review.md` |
| Latest capture | `artifacts/milestones/m2/captures/ts_js_git_review_validation_capture.json` |
| Validator commands | See `validator_commands` in the capture |
| Exact build identity | `artifacts/build/build_identity.json` |
| Freshness rule | `docs_claim_truth_proof`, `stale_after: P14D` |

## Protected Proof Path

Run the substrate checks cited by the capture:

```sh
cargo test -p aureline-shell clone
cargo test -p aureline-shell --test git_change_list_alpha
cargo test -p aureline-git --test status_alpha_fixtures
cargo test -p aureline-git --test branch_switch_alpha
cargo test -p aureline-git --test mutation_review_alpha
cargo test -p aureline-git --test conflict_handoff_alpha
cargo test -p aureline-git --test commit_alpha
cargo test -p aureline-git --test publish_review_alpha
cargo test -p aureline-review --test diff_view_alpha
cargo test -p aureline-review --test workspace_seed_alpha
cargo test -p aureline-shell --test git_review_activity_alpha
cargo test -p aureline-shell --test alpha_review_enforcement alpha_review_enforcement_snapshot_covers_required_lanes
python3 ci/check_alpha_scope.py --repo-root .
```

## Persona Coverage

The capture cites every persona row this packet exercises from
`artifacts/product/p0_persona_rows.yaml`:

- `persona:p0.typescript_web_app_developer` is exercised for the
  `git_review` workflow and `git_daily_loop` capability family through the
  TS/JS launch bundle and reference workspace seed.

The Python, Rust self-host, and held-persona rows are named as boundary rows in
the capture and are not promoted by this packet.

## Substrate Consumed

- `crates/aureline-shell/src/start_center/admission_review.rs` and
  `crates/aureline-shell/src/clone/` keep clone/open admission explicit,
  destination-reviewed, typed by failure class, and independent of account
  sign-in.
- `crates/aureline-git/src/status/` and
  `crates/aureline-shell/src/git_changes/` keep local Git status authoritative
  for staged/unstaged rows, source-control chips, diff-open targets, and
  degraded local-Git states.
- `crates/aureline-git/src/branches/` proves preview-first branch operations
  with dirty-worktree warnings, detached-head disclosure, missing-remote
  blocking, and drift checks before apply.
- `crates/aureline-git/src/mutations/` proves stage, unstage, discard, local
  checkpoint, revert, activity, and support-export lineage for source-control
  row mutations.
- `crates/aureline-git/src/conflicts/` proves Git merge-conflict and
  external-change handoff packets shared by editor, Git, CLI, and support
  surfaces without committing a silent write.
- `crates/aureline-git/src/commit/` proves normal commit, amend guardrails,
  author validation, staged-scope drift blocking, local-only publish readiness,
  and publish-later queue posture.
- `crates/aureline-git/src/publish/` proves explicit publish review,
  target/route disclosure, no merge-queue claim, local-state preservation, and
  failed-publish reopen recovery.
- `crates/aureline-review/src/diff/` and
  `crates/aureline-review/src/workspace/` prove local diff packets, safe-copy
  rows, reopen continuity, review-workspace anchors, provider-overlay
  attachability, and work-item linkage without replacing local diff truth.
- `crates/aureline-shell/src/activity_center/git_review.rs` proves
  Git/review activity and support-export rows preserve branch, target, action,
  exact reopen, and redaction posture.
- `crates/aureline-shell/src/review_preview/` and
  `crates/aureline-shell/src/save_review/` keep preview/apply/revert and
  compare-before-save review sheets on the same review-before-mutation posture.

## Same-Change-Set Checklist

- Owning proof packet added.
- Validator capture added.
- Scoreboard row moved to `green` and `conditional_go`.
- Scoreboard row now cites persona rows, reference workspace seed, launch
  bundle, review packet template, owning packet, and latest capture.
- Docs, migration notes, Help/About truth, known limits, and support-export
  truth are explicitly unchanged because this is `no_claim_widening`.
- `artifacts/milestones/m2/known_limits_alpha.yaml` is unchanged.
