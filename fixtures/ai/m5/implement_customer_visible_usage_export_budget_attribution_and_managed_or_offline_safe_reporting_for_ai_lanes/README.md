# Customer-Visible Usage Reporting Fixtures

## managed_outage_offline_fallback.json

A usage-reporting catalogue captured during a managed reporting-service outage.

The `offline-fallback-report` lane resolves to the managed mode with a
`managed_with_offline_fallback` continuity. When the managed reporting service
went down, the report was served from the offline fallback: its generation state
is `managed_unavailable_used_offline`, it is `offline_generatable`, and its
customer-visible export is `complete` and `available_now`. It keeps its Stable
claim because a clean offline fallback that served the full export is the
continuity feature working — not a degradation that the customer should distrust.
Its budget attribution splits the metered-medium period total across the
workspace and user dimensions, and no slice exceeds the period total.

The `managed-only-stranded-report` lane resolves to the managed mode with a
`managed_only` continuity and no offline fallback. The same outage stranded it:
its generation state is `managed_unavailable_no_fallback`, its export is
`unavailable` and `partial_degraded`, and its attribution is an
`estimated_unverified_band` estimate. Because it cannot back a Stable claim, it
sits at `experimental` and narrows to `unavailable` on either the stale-proof or
provider-unavailable trigger.

This demonstrates that an offline-safe lane stays trustworthy through a managed
outage while a managed-only lane with no offline fallback narrows its claim,
discloses its degraded state, and never advertises an available export when the
report could not be produced.
