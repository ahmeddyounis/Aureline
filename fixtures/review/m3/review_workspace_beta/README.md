# Review workspace beta fixtures

These fixtures exercise the beta review-workspace packet built from the
alpha local review seed. They prove that durable comment anchors retain
their source anchor identity, check freshness is independent of browser
state, browser handoff uses typed return anchors instead of raw URLs, and
support/export packets can reopen the review context.

| Fixture | Purpose |
| --- | --- |
| `local_workspace_with_reversible_browser_handoff.json` | Local diff workspace with one durable comment anchor, a current check, typed reversible browser handoff, and reopenable support export. |
| `stale_check_blocks_operator_truth.json` | Local diff workspace with a remapped durable anchor and stale check freshness that blocks operator-truth claims while keeping support export reopenable. |
