# AI Review Evidence, Finding Cards, and Review-Pack Change-Object Fixtures

These fixtures are valid, export-safe packets that exercise the labeling and
narrowing behavior the canonical support export keeps green. Each one keeps the
trust-review and consumer-projection invariants satisfied and proof freshness
valid — the difference is which states are narrowed and why.

## stale_evidence_apply_blocked.json

The evidence row is `superseded`, the finding card it backs is `superseded` and
`apply_blocked` with an explicit block-reason label, and the review-pack binding
is on a `bound_stale_base` with `required_missing_labeled` coverage. Demonstrates
that stale or superseded evidence is labeled rather than hidden and that a
suggestion whose evidence no longer holds is blocked from applying with a reason,
never silently applied.

## detached_binding_offline.json

CI status is unavailable because the workspace is offline, so the binding's
required-check coverage reports `unavailable_offline`. The review-pack binding
detached after the change object was reset and is `detached_relabeled` with an
explicit detach label; the finding card it scopes remains bound to that change
object. Demonstrates that a detached review-pack/change-object binding is
relabeled rather than dropped and that coverage stays explicit even when CI has
not reported.
