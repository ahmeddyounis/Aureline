# Query Envelope Alpha Fixtures

These fixtures pin the shared query-envelope vocabulary used by search, graph,
and docs consumers. The individual frame records are generated from the
reactive-state runtime in tests, while the support artifact and benchmark trace
show the same frames reused outside live UI rendering.

The fixtures intentionally exclude raw query text and result payloads. Support
and benchmark exports keep subscription, state, freshness, completeness,
refresh, cancellation, and invalidation tokens instead.
