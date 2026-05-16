# Review-pack parity-harness alpha: local/CI parity for repo-defined review packs

The alpha review-pack parity-harness is the post-execution comparison
between a local-lane run and a CI-derived run of one upstream review-pack
DSL record. A parity-harness record is **pre-publication review truth
for the parity claim itself**: it names which lanes engaged or declined,
how each check behaved in each lane, what parity finding the harness
recorded per check, and whether drift downgraded the row — without
itself mutating any branch, worktree, working tree, or remote.

A reviewer can answer four questions from a single record before
adopting a pack's parity claim:

1. **Which upstream pack was harnessed, and where does it anchor?**
   `review_pack_ref` and `repo_anchor_ref` pin the upstream pack and
   repo anchor as opaque refs.
2. **Which lanes engaged or declined?** `harness_lane_observations`
   names every lane with one of `lane_engaged`,
   `lane_declined_uncertified`, `lane_declined_unsupported`, or
   `lane_degraded_unknown_requires_review`.
3. **What did each check do in each lane?** Each
   `check_parity_findings` row carries the expected parity class, the
   local and CI outcome classes, the observed parity class, the
   `parity_finding_class`, and a short reviewable sentence.
4. **Did drift downgrade the row?** `drift_downgrades`,
   `row_downgrade_class`, and `overall_verdict_class` make the answer
   explicit instead of leaving it to ad hoc harness behavior.

The companion schema lives at:

- [`/schemas/review/review_pack_parity_harness.schema.json`](../../../schemas/review/review_pack_parity_harness.schema.json)

The canonical fixtures live under:

- [`/fixtures/review/m3/review_pack_harness/`](../../../fixtures/review/m3/review_pack_harness/)

The headless validator that gates every fixture lives at:

- [`/ci/check_review_pack_parity_harness_alpha.py`](../../../ci/check_review_pack_parity_harness_alpha.py)

The deterministic parity report consumed by docs, support, and partner
packets lives at:

- [`/artifacts/review/m3/local_ci_parity_report.md`](../../../artifacts/review/m3/local_ci_parity_report.md)

The Rust types are exported from
`aureline_review::review_pack_parity_harness`, defined in
[`crates/aureline-review/src/review_pack_parity_harness/mod.rs`](../../../crates/aureline-review/src/review_pack_parity_harness/mod.rs).
The integration test
[`crates/aureline-review/tests/review_pack_parity_harness_alpha.rs`](../../../crates/aureline-review/tests/review_pack_parity_harness_alpha.rs)
replays every fixture and proves the closed acceptance states. The
first shell consumer is
[`crates/aureline-shell/src/review/parity_harness_inspector/mod.rs`](../../../crates/aureline-shell/src/review/parity_harness_inspector/mod.rs),
which renders deterministic parity-harness rows directly from the
checked-in alpha fixtures and a matching CLI / headless plaintext
export.

## 1 Why freeze this now

Review workflows already need to prove the review-pack DSL is more than
syntax: every claimed pack family must show that its checks behave
consistently across the local and CI lanes, or that any documented
divergence (local-only, CI-only, declined-uncertified) matches the
pack's parity declaration. Without a parity record, a green local run
can silently mask a CI lane that declined the bundle or drifted on a
parity-unknown probe.

Freezing this now keeps three guarantees ahead of convenience:

- **Lanes are named, not assumed.** Both `local_lane` and `ci_lane`
  must be observed on every record so a reviewer can see which lane
  engaged and which declined.
- **Drift downgrades the row.** Every `drift_detected` finding **must**
  pair with a `drift_downgrades` entry, the `row_downgrade_class` must
  not be `no_downgrade`, and the `overall_verdict_class` cannot stay
  `full_parity`. The harness can never silently preserve a green claim
  under drift.
- **The report is consumable downstream.** Consumer surfaces always
  include `parity_harness_inspector`; docs, support, partner packets,
  and CLI / headless surfaces read the same rows.

## 2 Record shape

Every parity-harness row is one
`review_pack_parity_harness_alpha_record` carrying:

| Block | Required content |
| --- | --- |
| `parity_harness_id` | Opaque, stable id quoted by support, CLI, and review surfaces. |
| `review_pack_ref` | Opaque review-pack id matching a checked-in `review_pack_alpha:*` record. |
| `repo_anchor_ref` | Opaque repo-anchor id matching the upstream pack's anchor. |
| `pack_authority_class` | One of the closed review-pack authority classes (mirrors the upstream DSL). |
| `display_label` | Short label for the inspector and review preview. |
| `summary` | Reviewable sentence summarising the run. |
| `operator_caveat` | Reviewable sentence about scope and what the harness will and will not do. |
| `harness_lane_observations` | Two-plus entries; **must** include both `local_lane` and `ci_lane`. |
| `check_parity_findings` | Non-empty array of per-check findings. |
| `drift_downgrades` | Array of `{check_ref, downgrade_class, summary}`. Empty unless drift was detected. |
| `overall_verdict_class` | One of `full_parity`, `drift_downgraded`, `lane_declined_documented`, `parity_unknown_requires_review`. |
| `row_downgrade_class` | One of `no_downgrade`, `downgraded_to_advisory`, `downgraded_to_review_required`, `suspended_pack`. |
| `consumer_surfaces` | Non-empty list; must include `parity_harness_inspector`. |
| `support_export` | Packet refs and the closed `raw_*_export_allowed = false`. |
| `review_invariants` | All of `review_pack_ref_pinned`, `harness_lanes_pinned`, `check_findings_pinned`, `drift_downgrades_pinned`, `overall_verdict_pinned`, `no_hidden_writes` must be `true`. |

## 3 Frozen rules

The validator and the integration test both enforce:

1. **Versioning is pinned.** `schema_version` and `harness_version`
   both carry the alpha constant `1`. Adding a new closed-vocabulary
   value is additive-minor and bumps `schema_version`; repurposing a
   value or changing harness semantics is breaking and bumps
   `harness_version`.
2. **Drift always downgrades.** A finding with `parity_finding_class =
   drift_detected` requires a matching `drift_downgrades` entry, a
   non-`no_downgrade` `row_downgrade_class`, and an
   `overall_verdict_class` other than `full_parity`. The harness can
   never silently preserve a green claim under drift.
3. **Both lanes are observed.** `harness_lane_observations` must
   include both `local_lane` and `ci_lane`. A lane that did not engage
   uses a non-`lane_engaged` status class so the decline is documented.
4. **Closed export.** Raw paths, raw glob bodies, raw command lines,
   and raw check outputs are never exported through the parity-harness
   record. Support packets quote opaque refs, class tokens, and short
   reviewable sentences only.
5. **Consumer wiring.** `consumer_surfaces` always includes
   `parity_harness_inspector` so the first product surface stays bound.
6. **Pre-publication review.** The record is inspectable before its
   parity claim ships and writes nothing on its own. The local and CI
   harnesses still own their executions; the record describes them.

## 4 Vocabulary, by block

### 4.1 `harness_lane_class`

- `local_lane`
- `ci_lane`

### 4.2 `harness_lane_status_class`

- `lane_engaged`
- `lane_declined_uncertified`
- `lane_declined_unsupported`
- `lane_degraded_unknown_requires_review`

### 4.3 `check_lane_outcome_class`

- `passed_parity`
- `passed_with_drift_note`
- `failed_blocking`
- `failed_advisory`
- `declined_by_lane_documented`
- `skipped_by_lane_documented`
- `lane_outcome_unknown_requires_review`

### 4.4 `parity_finding_class`

- `full_parity`
- `local_only_documented_match`
- `ci_only_documented_match`
- `declined_documented_match`
- `drift_detected`
- `parity_unknown_requires_review`

### 4.5 `row_downgrade_class`

- `no_downgrade`
- `downgraded_to_advisory`
- `downgraded_to_review_required`
- `suspended_pack`

### 4.6 `overall_verdict_class`

- `full_parity`
- `drift_downgraded`
- `lane_declined_documented`
- `parity_unknown_requires_review`

### 4.7 `consumer_surface`

- `parity_harness_inspector`
- `review_pack_inspector`
- `review_preview`
- `cli_headless_entry`
- `support_export`
- `docs_review`
- `activity_center`

## 5 Fixtures

The checked-in fixtures under
[`/fixtures/review/m3/review_pack_harness/`](../../../fixtures/review/m3/review_pack_harness/)
cover every authority class and both the full-parity and
drift-downgrade verdicts:

| Fixture | Upstream pack | Overall verdict | Row downgrade |
| --- | --- | --- | --- |
| `first_party_full_parity_run.json` | `review_pack_alpha:first_party:core_review` | `full_parity` | `no_downgrade` |
| `team_shared_mixed_parity_documented.json` | `review_pack_alpha:team_shared:mixed_parity` | `full_parity` | `no_downgrade` |
| `partner_signed_ci_only_documented.json` | `review_pack_alpha:partner:cosign_audit` | `full_parity` | `no_downgrade` |
| `uncertified_community_drift_downgrade.json` | `review_pack_alpha:community:experimental_lints` | `drift_downgraded` | `downgraded_to_review_required` |

Every fixture keeps raw paths, raw glob bodies, raw command lines, and
raw check outputs closed; only opaque ref labels, closed-vocabulary
tokens, and short reviewable sentences cross the boundary.

## 6 Consumer wiring

The first product surface bound to this record is the shell
parity-harness inspector in
[`crates/aureline-shell/src/review/parity_harness_inspector/mod.rs`](../../../crates/aureline-shell/src/review/parity_harness_inspector/mod.rs).
It builds a deterministic parity-harness row per fixture and exports a
matching plaintext block (`render_alpha_parity_harness_plaintext`) for
CLI / headless / docs / support consumers, proving the parity claim is
inspectable and not doc-only.

The deterministic, support-and-partner-facing summary lives at
[`/artifacts/review/m3/local_ci_parity_report.md`](../../../artifacts/review/m3/local_ci_parity_report.md);
the CI validator gates it so the report cannot drift away from the
fixtures.

## 7 Out of scope

- Full M6 collaboration and full cloud-control-plane productization.
- Executing the checks themselves: the record describes what each lane
  did; the local and CI review-pack harnesses still own their
  executions.
- Embedding raw glob bodies, raw command lines, or raw check outputs in
  the record. Those stay in the harnesses' execution envelopes.
- Mutating branches, worktrees, or working trees. The Git mutation,
  publish, branch, and conflict-handoff services still own writes.
