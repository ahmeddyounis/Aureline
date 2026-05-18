# Provider Event Replay And Draft Reconciliation Fixtures

This directory contains provider-event reconciliation fixtures covering:

- duplicate webhook delivery dedupe by delivery identity plus scoped object;
- partial import sessions with explicit omissions and freshness labels;
- mirror-derived provider truth that remains non-canonical;
- callback denial export without user-visible provider mutation;
- deferred publish reconciliation that blocks on material provider drift;
- deferred publish reconciliation that drains only after clean revalidation.

The canonical fixture is `page.json`.
