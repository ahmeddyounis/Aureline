# M5 Batch-Review Sheet Fixtures

## batch_review_sheets_and_action_descriptors.json

A coverage fixture for the batch-review sheet packet. It binds reviewed selection
scopes to the first real broad M5 actions across pipeline runs, review queues,
incidents, marketplace results, and provider/admin tables, with one normalized
batch-action descriptor and review-gate contract per sheet.

The sheets exercise the truth states the lane must hold:

- **Locally reversible rerun with a mixed outcome** (pipeline run list): re-run 8
  of 11 runs (2 excluded, 1 client-capability blocked), `reversible_within_window`,
  with a post-execution `BatchResultSummary` that preserves per-item truth —
  7 queued, 1 failed to enqueue, 1 blocked — rather than collapsing to one toast.
- **Provider-backed approve with a policy block** (review queue): approve 20 of
  28 items, `compensatable_via_inverse`, `mixed_client_provider`, with 3 members
  blocked by a second-approver policy surfaced as an explicit `policy_blocked`
  scope block and 5 hidden outside the active filter.
- **Fully reversible suppress with skipped no-ops** (incident list): suppress 12
  of 17 incidents (3 excluded, 2 already-suppressed and skipped),
  `fully_reversible` by un-suppressing.
- **Provider-backed install with an incompatibility block** (marketplace
  results): install 6 of 12 extensions, `compensatable_via_inverse`, with 2
  members `provider_blocked` for incompatibility and 3 hidden off-page; select-all
  expansion was explicit.
- **Irreversible provider delete** (provider/admin table): delete 4 of 5 records,
  `irreversible` and `destructive_gated_batch`, with a named export-first recovery
  posture and 1 member blocked by a provider lock.

Every consequential sheet requires review before commit, blocks a generic
Continue button, names its included / excluded / blocked / skipped members, and
exposes a visible, exportable undo/recovery class. Policy and provider narrowing
is threaded into explicit scope blocks rather than hidden in a filter chip. No
sheet carries raw row bodies, provider payloads, or credentials.

The fixture validates against
`schemas/collections/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe.schema.json`
and is byte-identical to the checked support export at
`artifacts/collections/m5/add-batch-review-sheets-and-batch-action-descriptors-with-included-excluded-blocked-skippe/support_export.json`.
