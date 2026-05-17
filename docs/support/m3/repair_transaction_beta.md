# Typed repair-transaction preview skeleton with checkpoint, compensation class, and blast-radius truth

A repair preview skeleton is the bounded recovery posture a blocked user
enters when Project Doctor or the Support Center proposes a fix. The
skeleton *declares* — by name and class — what object classes a repair
would touch, what its blast radius is, what compensation the reviewer must
accept, and what checkpoint will be captured before any apply path runs.
Every skeleton is cancellable: a reviewer can compare it against a prior
baseline, accept it for apply review, or cancel it before any durable
state mutates. The beta projection preserves the alpha
`repair_transaction_record` id and `transaction_reversal_class` verbatim
so support and audit packets carry the same transaction id and reversal
class as the live shell.

The implementation lives in
[`crates/aureline-support/src/repair_transactions/mod.rs`](../../../crates/aureline-support/src/repair_transactions/mod.rs)
and the boundary schema lives at
[`/schemas/support/repair_transaction_preview_skeleton.schema.json`](../../../schemas/support/repair_transaction_preview_skeleton.schema.json).
The protected fixture corpus lives under
[`fixtures/recovery/m3/repair_transaction_preview/`](../../../fixtures/recovery/m3/repair_transaction_preview/).

## What this beta row owns

- A typed [`RepairPreviewSkeleton`] record that declares every previewed
  fix as a typed
  `(blast_radius_class, compensation_class, affected_object_class[],
  checkpoint_disposition_class, preview_disposition_class)` row bound to
  the alpha `repair_transaction_ref` and `reversal_class`. Blast radius
  is one of `single_object_class`, `multi_object_class_same_family`,
  `multi_object_class_cross_family`, or
  `no_local_blast_escalation_only`; compensation class is one of five
  closed tokens distinct from the alpha reversal class
  (`no_compensation_needed`, `regenerate_from_authoritative_source`,
  `semantic_inverse_compensation`, `manual_followup_required`,
  `audit_only_no_state_change`).
- A typed [`RepairPreviewComparison`] record for every cancellable
  before-apply review. Comparisons carry a closed `differing_axes` list
  (`blast_radius_diff`, `compensation_diff`, `affected_object_diff`,
  `preserved_state_diff`, `checkpoint_diff`, `reversal_class_diff`) and
  a closed `cancellation_class` (`continue_comparison`,
  `cancel_before_apply`, `ready_for_apply_review`) so a reviewer can
  cancel before any apply path runs.
- A [`RepairPreviewSkeletonEvaluator`] that validates skeleton and
  comparison shapes, cross-checks that a comparison's `bound_skeleton_ref`
  and `candidate_skeleton_ref` both match the supplied skeleton id, and
  projects one [`RepairPreviewSupportPacket`] per skeleton. The packet
  excludes raw private material and ambient authority, quotes the doc and
  schema refs verbatim, and preserves the alpha transaction id and
  reversal class on every row.
- A [`RepairPreviewSkeletonEvaluator::from_alpha_transaction`] compiler
  that consumes any alpha `repair_transaction_record` and emits the
  matching beta skeleton without re-deriving truth from a side channel:
  the affected-object rows mirror the alpha `impacted_state_classes`, the
  blast radius is derived from the object-class families, the
  compensation class is derived from the alpha reversal class, and the
  checkpoint disposition is derived from the alpha `apply_mode_class`.

## Acceptance and how this row meets it

- **Repair previews show what will change, what is reversible, what
  requires compensation, and what object classes are affected.** Every
  skeleton declares typed affected-object rows, a typed `reversal_class`
  preserved from the alpha transaction, a typed `compensation_class`
  distinct from reversal, and a typed `blast_radius_class`. The schema
  refuses a `single_object_class` skeleton that lists multiple objects
  and a `no_local_blast_escalation_only` skeleton that lists any object;
  the evaluator further refuses an `audit_only_no_state_change`
  compensation class with non-empty affected-object rows.
- **Repair flows can be cancelled or compared before mutation rather than
  acting as hidden "fix it" buttons.** Every skeleton carries a typed
  `preview_disposition_class`, and every fixture pairs the skeleton with
  a `repair_preview_comparison_record` whose `cancellation_class` is one
  of `continue_comparison`, `cancel_before_apply`, or
  `ready_for_apply_review`. The evaluator refuses an
  `authorized_for_apply` disposition on a compensating or manual
  compensation class without a prior comparison, so non-exact repairs
  travel through compare-before-apply on every path.
- **Support and audit packets preserve the same transaction IDs and
  reversal classes.** The metadata-safe
  `repair_preview_skeleton_support_packet_record` quotes
  `repair_transaction_ref` and `reversal_class` verbatim from the
  skeleton (and therefore the alpha transaction), and the evaluator
  refuses any comparison whose `bound_skeleton_ref` or
  `candidate_skeleton_ref` does not match the bound skeleton id. Tests
  cover packet round-trip per fixture and refusal of mixed-skeleton
  packets.

## Failure-drill posture

The evaluator fails closed before any apply path is admitted:

- A skeleton that declares a destructive reset is refused.
- A skeleton that drops `user_authored_files` from preserved state is
  refused.
- A skeleton with a `durable_pre_apply_checkpoint` or
  `ephemeral_pre_apply_checkpoint` disposition but no `checkpoint_ref`
  is refused; conversely a `no_checkpoint_*` disposition with a
  non-null `checkpoint_ref` is refused.
- A `single_object_class` blast radius with anything other than exactly
  one affected-object row is refused; a `no_local_blast_escalation_only`
  blast radius with any affected-object row is refused.
- An `audit_only_no_state_change` compensation class with any
  affected-object row is refused.
- A `refused_no_local_repair` preview disposition that does not also
  declare `no_checkpoint_escalation_only` is refused.
- An `authorized_for_apply` disposition with a compensating or manual
  compensation class is refused (compare-before-apply is mandatory for
  non-exact repairs).
- A comparison with empty `differing_axes`, an empty baseline or
  candidate ref, or duplicate axes is refused.
- A comparison whose `bound_skeleton_ref` or `candidate_skeleton_ref`
  does not match the bound skeleton id is refused at `support_packet`
  time.

## First consumers

- The `aureline-support` `repair_transactions` module is the canonical
  projection for support-export and recovery-ladder review.
  `RepairPreviewSkeletonEvaluator::support_packet` folds one skeleton
  and its bound comparisons into a metadata-safe
  [`RepairPreviewSupportPacket`] that the support-export pipeline can
  serialize verbatim.
- The boundary schema is the contract the headless export writer and the
  support-export chrome share — both reconstruct the same packet shape
  from the on-disk record verbatim, never re-derive it from a side
  channel.
- The
  [`RepairPreviewSkeletonEvaluator::from_alpha_transaction`] compiler is
  the bridge consumers use to upgrade alpha transactions into the beta
  preview surface without retyping the alpha contract.

## Related contracts

- [Repair-transaction contract (alpha)](../repair_transaction_contract.md)
  — the parent contract for the alpha
  `repair_transaction_record`, `repair_preview_record`, and
  `repair_outcome_record`. This beta row preserves the alpha transaction
  id and reversal class verbatim.
- [Repair preview alpha](../repair_preview_alpha.md) — the alpha
  inspectability path. The beta skeleton consumes the alpha transaction
  and produces a cancellable/comparable preview before any apply path
  runs.
- [Recovery-action schema](../../../schemas/support/recovery_action.schema.json)
  — the source of the `lost_capability_class` and
  `preserved_state_class` vocabularies this beta row mirrors.
- [Project Doctor finding contract](../project_doctor_contract_alpha.md)
  — the `doctor.finding.*` ref every skeleton cites.

## Out of scope for this beta row

- Live apply execution and rollback execution. The beta surface is the
  cancellable preview skeleton; the alpha journal and outcome records
  continue to own apply lineage.
- Cross-tenant policy reconciliation; the fixture corpus covers
  single-tenant repairs and a guided-export escalation lane.
- AI- or operator-assisted repair authoring; the beta surface assumes a
  Project Doctor finding has already proposed the repair.

[`RepairPreviewSkeleton`]: ../../../crates/aureline-support/src/repair_transactions/mod.rs
[`RepairPreviewComparison`]: ../../../crates/aureline-support/src/repair_transactions/mod.rs
[`RepairPreviewSkeletonEvaluator`]: ../../../crates/aureline-support/src/repair_transactions/mod.rs
[`RepairPreviewSkeletonEvaluator::from_alpha_transaction`]: ../../../crates/aureline-support/src/repair_transactions/mod.rs
[`RepairPreviewSkeletonEvaluator::support_packet`]: ../../../crates/aureline-support/src/repair_transactions/mod.rs
[`RepairPreviewSupportPacket`]: ../../../crates/aureline-support/src/repair_transactions/mod.rs
