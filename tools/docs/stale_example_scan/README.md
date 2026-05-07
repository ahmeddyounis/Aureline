# Docs freshness scanner

Entry point: `tools/docs/stale_example_scan/scan_docs_freshness.py`

Primary config and inputs:

- `ci/check_docs_freshness.yml` (scan scope + ledger path)
- `artifacts/docs/snippet_freshness_ledger.yaml` (expected anchors and drift contract)
- `docs/docs/stale_example_ci.md` (policy and usage)

Run locally:

```bash
./ci/check_docs_freshness.sh
```

