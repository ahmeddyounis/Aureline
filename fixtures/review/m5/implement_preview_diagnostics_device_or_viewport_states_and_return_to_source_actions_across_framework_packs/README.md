# Preview Diagnostics, Device/Viewport States, and Return-to-Source Actions Fixtures

These fixtures are valid, export-safe packets that exercise the diagnostic,
device/viewport, source-mapping, and return-to-source narrowing behavior the
canonical support export keeps green. Each one keeps the trust-review and
consumer-projection invariants satisfied and proof freshness valid — the
difference is which states are narrowed and why.

## stale_source_map_blocked.json

A build-error diagnostic whose source map is from a prior build, so the freshness
is `stale_prior_build`, the return-to-source action is `unsupported_no_source_map`
and `blocked_source_map_stale_review_required`, and the diagnostic carries
explicit attention reasons. Demonstrates that a stale source map narrows the
return-to-source action rather than jumping to a possibly-wrong location.

## unknown_pack_blocked.json

A diagnostic from a provider-owned framework pack the contract cannot make
explicit, so the pack class, severity, diagnostic kind, viewport, mapping, and
freshness are all `unknown_*_provider_owned`, the return-to-source action is
`unsupported_no_source_map` and `blocked_generated_content_no_origin`, and the
diagnostic carries explicit attention reasons. Demonstrates that an unknown pack,
viewport, severity, or mapping is disclosed rather than assumed, and that a
diagnostic with no resolvable origin blocks its return-to-source action.
