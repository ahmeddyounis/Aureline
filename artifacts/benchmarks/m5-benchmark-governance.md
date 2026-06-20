# M5 benchmark-governance matrix — human-readable rendering

Human-readable rendering of the canonical M5 benchmark-governance matrix. The
machine-readable truth is at
[`artifacts/benchmarks/m5-benchmark-governance.json`](./m5-benchmark-governance.json),
validated by
[`schemas/benchmarks/m5-benchmark-governance.schema.json`](../../schemas/benchmarks/m5-benchmark-governance.schema.json).
The normative policy companion is
[`docs/benchmarks/m5-benchmark-governance.md`](../../docs/benchmarks/m5-benchmark-governance.md).
This row is governed by the canonical M5 evidence index
(`artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`).

The matrix binds every protected M5 metric to one named corpus-manifest
revision, one reference-hardware profile, one lab-image revision, one governed
threshold state, one owner, one waiver path, and one freshness rule — and
freezes the closed narrowing vocabulary that drops a claim when its evidence
goes stale or incomparable.

## Claim levels (ordered)

| Rank | Level | Claim-bearing | Meaning |
| --- | --- | --- | --- |
| 0 | `quarantined_not_comparable` | no | Carries no performance conclusion; only explains why a prior comparison is no longer comparable. |
| 1 | `internal_gate_only` | no | May gate CI internally but may not state an external result. |
| 2 | `methodology_only` | no | May explain how a task is measured but may not claim a win. |
| 3 | `aureline_only_claim` | yes | May state Aureline's own measured result against a reference capture. |
| 4 | `public_head_to_head_comparison` | yes | May compare against another product on a named task with full disclosure. |

The effective claim of any row is the **lowest-ranked** of its published ceiling
and every fired narrowing rule's target.

## Protected metrics

| Metric | Corpus | Hardware profile | Lab image | Threshold state | Waiver | Ceiling | Effective |
| --- | --- | --- | --- | --- | --- | --- | --- |
| `ff.warm_start_to_first_paint` | `corpus.workflow.startup_warm_to_first_paint` | `…macos15.arm64.apple_silicon_14in` | `lab_image.macos15.arm64.rev1` | `provisional_uncalibrated` | `none` | `methodology_only` | `methodology_only` |
| `ff.first_paint` | `corpus.workflow.startup_warm_to_first_paint` | `…macos15.arm64.apple_silicon_14in` | `lab_image.macos15.arm64.rev1` | `provisional_uncalibrated` | `none` | `methodology_only` | `methodology_only` |
| `ff.buffer_operations` | `corpus.micro.shaping_smoke_cases` | `…macos15.arm64.apple_silicon_14in` | `lab_image.macos15.arm64.rev1` | `frozen_calibrated` | `none` | `aureline_only_claim` | `aureline_only_claim` |
| `ff.vfs_save_conflict_handling` | `corpus.workflow.first_useful_edit_rust_self_host` | `…macos15.arm64.apple_silicon_14in` | `lab_image.macos15.arm64.rev1` | `frozen_calibrated` | `none` | `aureline_only_claim` | `aureline_only_claim` |
| `ff.benchmark_lab_health` | `corpus.micro.interaction_safety_cases` | `…macos15.arm64.apple_silicon_14in` | `lab_image.macos15.arm64.rev1` | `frozen_calibrated` | `none` | `internal_gate_only` | `internal_gate_only` |
| `ff.command_parity` | `corpus.workflow.first_useful_edit_rust_self_host` | `…macos15.arm64.apple_silicon_14in` | `lab_image.macos15.arm64.rev1` | `provisional_uncalibrated` | `performance_council_time_boxed` (expires `2026-09-18`) | `internal_gate_only` | `internal_gate_only` |

Each metric's threshold value lives in
[`artifacts/bench/protected_metrics.yaml`](../bench/protected_metrics.yaml); the
matrix binds the row identity, comparability anchors, and claim ceiling. Each
metric's threshold *history* — the typed change records, before/after evidence,
rationale, approval lineage, and waiver expiry — lives in the
[threshold-change ledger](./threshold-change-ledger.json) and is enforced by the
[threshold-change policy](../../docs/benchmarks/threshold-change-policy.md).

## Corpus manifests

| Corpus | Class | Rev | License | Retention | Access | External review |
| --- | --- | --- | --- | --- | --- | --- |
| `corpus.micro.shaping_smoke_cases` | microbenchmark | 1 | mit_0bsd | permanent_seed | public | no |
| `corpus.micro.interaction_safety_cases` | microbenchmark | 1 | synthetic | permanent_seed | public | no |
| `corpus.workflow.startup_warm_to_first_paint` | workflow | 1 | synthetic | permanent_seed | public | no |
| `corpus.workflow.first_useful_edit_rust_self_host` | workflow | 1 | mit_0bsd | permanent_seed | public | no |

External, partner-, or customer-derived corpora carry
`external_review_required: true` and pass licensing, redaction, retention, and
access-control review before CI admission.

## Reference hardware profiles

| Hardware profile | Class | Display class | Power posture |
| --- | --- | --- | --- |
| `hardware_definition.ref.macos15.arm64.apple_silicon_14in` | reference_lab | `display_class.internal_14in_retina_3024x1964_sdr60` | `power_posture.ac_balanced` |
| `hardware_definition.ref.windows11.x86_64.thinkpad_t14_gen5` | reference_lab | `display_class.internal_14in_1920x1200_sdr60` | `power_posture.ac_balanced` |
| `hardware_definition.ref.ubuntu24_04.x86_64.framework13` | reference_lab | `display_class.internal_13_5in_2256x1504_sdr60` | `power_posture.ac_balanced` |
| `hardware_definition.self_capture.current_machine_reported` | self_capture | `display_class.self_capture.current_machine_reported` | `power_posture.self_capture.reported_out_of_band` |

## Lab images

| Lab image | Rev | Environment | Calibration rule set |
| --- | --- | --- | --- |
| `lab_image.macos15.arm64.rev1` | 1 | `environment_definition.ref.macos15.arm64.internal_14in_nominal` | `calibration_rule_set.reference_lab` |
| `lab_image.windows11.x86_64.rev1` | 1 | `environment_definition.ref.windows11.x86_64.internal_14in_nominal` | `calibration_rule_set.reference_lab` |
| `lab_image.ubuntu24_04.x86_64.rev1` | 1 | `environment_definition.ref.ubuntu24_04.x86_64.internal_13_5in_nominal` | `calibration_rule_set.reference_lab` |
| `lab_image.self_capture.unmanaged_local.rev1` | 1 | `environment_definition.self_capture.current_machine_default` | `calibration_rule_set.self_capture_disclosure` |

## Threshold states

| State | Claim-bearing allowed | Fires threshold drift |
| --- | --- | --- |
| `frozen_calibrated` | yes | no |
| `tightened` | yes | no |
| `eased_with_evidence` | yes | no |
| `provisional_uncalibrated` | no | no |
| `drifted_unreviewed` | no | **yes** |
| `stale_recalibration_pending` | no | **yes** |

## Narrowing rules

Every rule is mechanically detectable; the effective claim is recomputed from
the firing rules.

| Narrowing reason | Detects | Narrows to |
| --- | --- | --- |
| `stale_corpus_revision` | Bound corpus revision ≠ current manifest revision | `methodology_only` |
| `missing_hardware_identity` | No hardware profile / display class bound | `quarantined_not_comparable` |
| `missing_lab_image_identity` | No lab-image revision / environment bound | `quarantined_not_comparable` |
| `threshold_drift` | Threshold state drifted or recalibration-pending | `internal_gate_only` |
| `incomparable_run_metadata` | A comparability axis changed without a reset capture | `quarantined_not_comparable` |
| `expired_waiver` | Active waiver past its expiry | `internal_gate_only` |
| `stale_freshness` | Evidence capture past the freshness SLO | `methodology_only` |
| `undisclosed_publication_field` | A publication pack omits a required disclosure field | `methodology_only` |

## Publication packs

| Pack | Posture | Metric | Ceiling | Effective |
| --- | --- | --- | --- | --- |
| `publication_pack.methodology.startup_warm_to_first_paint` | methodology_only | `ff.warm_start_to_first_paint` | `methodology_only` | `methodology_only` |
| `publication_pack.aureline_only.buffer_operations` | aureline_only_claim | `ff.buffer_operations` | `aureline_only_claim` | `aureline_only_claim` |
| `publication_pack.head_to_head.first_useful_edit` | public_head_to_head_comparison | `ff.vfs_save_conflict_handling` | `public_head_to_head_comparison` | `public_head_to_head_comparison` |
| `publication_pack.quarantined.legacy_first_paint` | quarantined_not_comparable | `ff.first_paint` | `quarantined_not_comparable` | `quarantined_not_comparable` |

## Consumer bindings

| Consumer | Projection | What it ingests |
| --- | --- | --- |
| `release_shiproom` | `promotion_gate_projection` | Effective claims + narrowing reasons; holds promotion when a claim-bearing metric is quarantined or narrowed below its pack's posture. |
| `support_export` | `redaction_safe_projection` | Effective claims, narrowing reasons, and bound ids only — never raw run logs or machine labels. |
| `docs` | `claim_state_projection` | Effective claim + downgrade label per metric and pack. |
| `help` | `claim_state_projection` | Effective claim + downgrade label in Help/About benchmark cards. |

## Recompute and promotion gate

- **Recompute:** for each metric and pack, fire every narrowing rule whose
  condition holds, then set the effective claim to the lowest-ranked of the
  published ceiling and each fired rule's target.
- **Promotion gate:** promotion holds when any claim-bearing metric's effective
  claim is below the posture its publication pack asserts, or when a published
  pack omits a required disclosure field.

The narrowing fixtures under
[`fixtures/benchmarks/m5-benchmark-governance/`](../../fixtures/benchmarks/m5-benchmark-governance/)
prove each narrowing path is detected automatically.
