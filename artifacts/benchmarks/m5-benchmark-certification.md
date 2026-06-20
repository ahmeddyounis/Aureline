# M5 benchmark-certification — human-readable rendering

Human-readable rendering of the canonical M5 benchmark-certification proof
packet. The machine-readable truth is at
[`artifacts/benchmarks/m5-benchmark-proof-packet.json`](./m5-benchmark-proof-packet.json),
validated by
[`schemas/benchmarks/m5-benchmark-proof-packet.schema.json`](../../schemas/benchmarks/m5-benchmark-proof-packet.schema.json).
The normative policy companion is
[`docs/benchmarks/m5-benchmark-certification.md`](../../docs/benchmarks/m5-benchmark-certification.md).
This lane is registered under the canonical M5 evidence index
(`artifacts/release/m5/certify_the_full_m5_train_narrow_stale_rows_and_publish_the_canonical_evidence_index.json`).

The proof packet binds every claimed M5 **performance**, **compatibility**, and
**qualification** row to the upstream truth that backs it — the
benchmark-governance matrix, the shiproom freshness ledger, the threshold-change
ledger, the corpus-intake ledger, the public-comparison reproducibility
register, and the publication-ingestion register — and recomputes, for each row,
the certification gaps that fire across six evidence pillars, the narrowed
effective claim, and the certification state. It is the single certification lane
that decides whether a claimed row may stay green.

## Claim levels (ordered)

| Rank | Level | Claim-bearing |
| --- | --- | --- |
| 0 | `quarantined_not_comparable` | no |
| 1 | `internal_gate_only` | no |
| 2 | `methodology_only` | no |
| 3 | `aureline_only_claim` | yes |
| 4 | `public_head_to_head_comparison` | yes |

The effective claim of any row is the **lowest-ranked** of its published ceiling,
the freshness ledger entry's effective claim, and every fired certification gap's
target.

## Evidence pillars

Each row must prove all six pillars. The pillar's current truth lives in the
cited upstream artifact; the proof packet binds the row to it and recomputes the
gap mechanically.

| Pillar | Proves | Source of truth |
| --- | --- | --- |
| `corpus_identity` | Every bound corpus carries an approved, CI-admitted intake decision. | `corpus-intake-ledger.json` |
| `hardware_basis` | Every metric binds a reference-hardware profile and lab image; no claim-bearing row rides self-capture. | `m5-benchmark-governance.json` |
| `threshold_lineage` | Every metric carries an in-force threshold-change record with no expired waiver. | `threshold-change-ledger.json` |
| `reproducibility_pack` | Every governance pack binds a retained, in-force, fresh reproducibility pack. | `public-comparison-pack-register.json` |
| `freshness_comparability` | The backing run is current and comparable on every axis. | `shiproom-benchmark-freshness.json` |
| `publication_propagation` | The entry reaches every required publication surface. | `publication-ingestion-register.json` |

## Certification gaps (narrowing vocabulary)

Every gap is mechanically detectable; the effective claim is recomputed from the
firing gaps.

| Gap | Pillar | Narrows to |
| --- | --- | --- |
| `uncertified_corpus_intake` | corpus_identity | `quarantined_not_comparable` |
| `missing_hardware_basis` | hardware_basis | `quarantined_not_comparable` |
| `missing_threshold_lineage` | threshold_lineage | `internal_gate_only` |
| `expired_threshold_waiver` | threshold_lineage | `internal_gate_only` |
| `missing_reproducibility_pack` | reproducibility_pack | `methodology_only` |
| `incomplete_reproducibility_pack` | reproducibility_pack | `methodology_only` |
| `stale_reproducibility_pack` | reproducibility_pack | `methodology_only` |
| `stale_freshness_evidence` | freshness_comparability | `methodology_only` |
| `incomparable_freshness_evidence` | freshness_comparability | `quarantined_not_comparable` |
| `missing_publication_propagation` | publication_propagation | `methodology_only` |

## Certification states

| State | Certified | Holds promotion when claim-bearing |
| --- | --- | --- |
| `certified` | yes | no |
| `narrowed` | no | yes |
| `quarantined` | no | yes |

## Certification rows

| Row | Class | Entry | Posture | Effective | State | Blocks |
| --- | --- | --- | --- | --- | --- | --- |
| `cert.performance.buffer_operations` | performance | `publication_pack.aureline_only.buffer_operations` | `aureline_only_claim` | `aureline_only_claim` | certified | no |
| `cert.performance.warm_start_to_first_paint` | performance | `publication_pack.methodology.startup_warm_to_first_paint` | `methodology_only` | `methodology_only` | certified | no |
| `cert.compatibility.first_useful_edit_head_to_head` | compatibility | `publication_pack.head_to_head.first_useful_edit` | `public_head_to_head_comparison` | `public_head_to_head_comparison` | certified | no |
| `cert.qualification.legacy_first_paint` | qualification | `publication_pack.quarantined.legacy_first_paint` | `quarantined_not_comparable` | `quarantined_not_comparable` | quarantined | no |

Each row also lists the in-force threshold-change record, the corpus-intake
decision, the reproducibility pack, and the publication-surface bindings it
rides, plus the reference-workspace, qualification-matrix, and family-certification
objects it stays aligned with. The quarantined qualification row asserts no claim
and is retained for diagnosis; it does not hold promotion.

## Promotion gate

- **Recompute:** for each row, resolve its bindings, fire every certification gap
  whose condition holds, set the effective claim to the lowest-ranked of the
  published ceiling, the freshness entry's effective claim, and each fired gap's
  target, then derive the state.
- **Promotion gate:** promotion holds when any claim-bearing row's effective claim
  is below the posture it asserts. The current packet resolves to **proceed**: no
  claim-bearing row is narrowed below its posture.

## Consumer bindings

| Consumer | Projection | What it ingests |
| --- | --- | --- |
| `release_shiproom` | `promotion_gate_projection` | Certification states, fired gaps, effective claims, and the promotion verdict. |
| `release_packet` | `certification_alignment_projection` | Each packet's declared certification state and effective claim, which must equal the recompute. |
| `support_export` | `redaction_safe_projection` | States, gaps, labels, and bound ids only — never raw logs, machine labels, or provider payloads. |
| `docs_help` | `certification_state_projection` | Effective claim and certification label per row. |

The certification fixtures under
[`fixtures/benchmarks/m5-benchmark-certification/`](../../fixtures/benchmarks/m5-benchmark-certification/)
prove each fail-closed certification path is detected automatically.
