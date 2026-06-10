# Evidence-Rich Patch Review Fixtures

This directory contains fixture files for the evidence-rich patch review lane.

## Files

- `valid_packet.json` — A fully valid evidence-rich patch review packet that passes all validation invariants.
- `missing_validation_receipt.json` — A packet where validation is required but no receipt is present, triggering `validation_receipt_missing`.
