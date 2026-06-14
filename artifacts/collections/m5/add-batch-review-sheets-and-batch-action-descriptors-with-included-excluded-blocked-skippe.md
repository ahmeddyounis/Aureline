# M5 Batch-Review Sheets And Batch-Action Descriptors

- Packet: `m5-batch-review-sheet:stable:0001`
- Label: `M5 Batch-Review Sheets And Batch-Action Descriptors`
- Sheets: 5 (5 gated)
- Surfaces: 5 / 5
- Undo classes: 4 / 6

## Sheets

- **sheet:pipeline-rerun:0001** (pipeline_run_list / list / rerun): Re-run 8 failed pipeline runs
  - included=8 excluded=2 blocked=1 skipped=0 hidden=0 total=11
  - undo=`reversible_within_window` origin=`local_client` scope=`local_reversible_batch` gated=true
  - recovery: Reversible within a 5-minute cancel window; cancelled reruns leave runs untouched.
  - block `client_capability` x1: 1 run is locked by an in-progress deployment and cannot re-run yet.
  - result: ok=7 failed=1 skipped=0 blocked=1 (7 reruns queued, 1 failed to enqueue, 1 blocked by an active deployment.)
- **sheet:review-approve:0001** (review_queue / queue / approve): Approve 20 reviewed items
  - included=20 excluded=0 blocked=3 skipped=0 hidden=5 total=28
  - undo=`compensatable_via_inverse` origin=`mixed_client_provider` scope=`mixed_client_provider_batch` gated=true
  - recovery: Compensatable: approvals are reversed by re-opening each item from its history.
  - block `policy_blocked` x3: 3 items require a second approver before they can be approved in bulk.
- **sheet:incident-suppress:0001** (incident_list / list / suppress): Suppress 12 incidents
  - included=12 excluded=3 blocked=0 skipped=2 hidden=0 total=17
  - undo=`fully_reversible` origin=`local_client` scope=`local_reversible_batch` gated=true
  - recovery: Fully reversible: un-suppress restores each incident with no data loss.
- **sheet:marketplace-install:0001** (marketplace_results / table / install): Install 6 extensions
  - included=6 excluded=1 blocked=2 skipped=0 hidden=3 total=12
  - undo=`compensatable_via_inverse` origin=`mixed_client_provider` scope=`mixed_client_provider_batch` gated=true
  - recovery: Compensatable: uninstall each extension to reverse the install.
  - block `provider_blocked` x2: 2 extensions are not compatible with the current provider and cannot install.
- **sheet:admin-delete:0001** (provider_admin_table / table / delete): Delete 4 provider records
  - included=4 excluded=0 blocked=1 skipped=0 hidden=0 total=5
  - undo=`irreversible` origin=`provider_authoritative` scope=`destructive_gated_batch` gated=true
  - recovery: Irreversible: deletion is permanent. Export the 4 records first to retain a copy.
  - block `provider_blocked` x1: 1 record is locked by the provider and cannot be deleted.
