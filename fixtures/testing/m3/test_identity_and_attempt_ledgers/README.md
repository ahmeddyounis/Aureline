# Test Identity And Attempt Ledgers

Fixtures in this directory exercise the promoted test identity ledger used by
runtime, shell, CLI/headless, review, support, and imported-CI overlay flows.

- `canonical_pytest_bundle.json` shows one local pytest item projected through
  canonical item, selector, session, attempt, surface-binding, imported-CI
  overlay, and support-export records.

The fixture intentionally stores ids, digests, counts, timestamps, class
tokens, and support summaries only. Raw runner output, raw command lines, raw
environment bodies, raw provider payloads, and raw assertion text stay outside
the fixture boundary.
