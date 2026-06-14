# M5 Dense Collection Certification Fixtures

## certification_blocks_regression_and_auto_narrows_missing_proof.json

A release-gate drill fixture for the dense-collection certification packet. Every
claimed M5 dense collection surface — pipeline run list, review queue, incident
list, graph list, marketplace results, activity rows, provider/admin table,
query-backed result set, and support/export projection — carries one certification
row with a proof per required dimension (filter-AST, saved-view, result-count,
selection-scope, and batch-action).

Seven rows present current proof on every dimension and are `certified` at their
claim. Two rows exercise the gate:

- The provider/admin row's candidate build hid provider/policy narrowing inside a
  generic filter chip. Because a regression of a release-gating invariant must
  block promotion, the row is `blocked`, records the
  `provider_policy_narrowing_erased` regression class, narrows from `beta` to
  `held`, and carries a precise narrowed label rather than a generic provider
  error.
- The support/export projection row's batch-action review proof is not yet
  published (`batch_action` status is `missing`). Because a claimed row may not
  outrun current proof, the row is `auto_narrowed` from `beta` to `held` with a
  precise narrowed label.

Every certified row holds its release invariants: selection survives by stable
identity, provider/policy narrowing is disclosed, the visible count stays distinct
from the all-matching count, and every broad batch action previews before commit.

The fixture validates against
`schemas/collections/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle.schema.json`
and is byte-identical to the checked support export at
`artifacts/collections/m5/certify-filter-saved-view-selection-and-batch-action-truth-on-every-claimed-m5-dense-colle/support_export.json`.
