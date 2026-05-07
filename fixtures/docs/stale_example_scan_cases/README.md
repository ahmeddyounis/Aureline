# Docs freshness scanner fixture cases

These fixtures demonstrate both passing and failing cases for the docs
freshness scanner:

- version mismatch
- missing source anchor
- stale screenshot-safe copy
- drifted command id
- stale migration example
- stale provider/browser-handoff guidance

Run the passing set:

```bash
python3 tools/docs/stale_example_scan/scan_docs_freshness.py --config fixtures/docs/stale_example_scan_cases/check_docs_freshness_pass.yml
```

Run the failing set (expected to exit non-zero):

```bash
python3 tools/docs/stale_example_scan/scan_docs_freshness.py --config fixtures/docs/stale_example_scan_cases/check_docs_freshness_fail.yml
```

